use crate::{ewin_com::def::*, ewin_com::log::*, ewin_com::model::*, model::*};
use ewin_com::_cfg::cfg::Cfg;
use regex::Regex;
use std::collections::BTreeSet;

impl FormatXml {
    pub fn format_xml_html(text: String, fmt_type: FmtType, nl: String) -> String {
        Log::debug_key("format_xml_html");
        Log::debug("fmt_type", &fmt_type);
        Log::debug("nl", &nl);

        let edit_text_non_comment = Regex::new(r"<!--[\s\S]*?-->").unwrap().replace_all(&text, "").to_string();

        // "<" → "~::~<"   ※~::~ is mark
        let edit_text = Regex::new(r"<").unwrap().replace_all(&text, "~::~<").to_string();
        // " xmlns:" → "~::~xmlns:"
        let edit_text = Regex::new(r"\s*xmlns:").unwrap().replace_all(&edit_text, "~::~xmlns:").to_string();
        // " xmlns=" → "~::~xmlns="
        let edit_text = Regex::new(r"\s*xmlns=").unwrap().replace_all(&edit_text, "~::~xmlns=").to_string();
        // HTML(Javascript)
        let edit_text = Regex::new(r"\(function\(").unwrap().replace_all(&edit_text, r"~::~(function(").to_string();
        let edit_text = Regex::new(r"function\s").unwrap().replace_all(&edit_text, r"~::~function ").to_string();

        let str_array = edit_text.split("~::~").collect::<Vec<_>>();

        let len = str_array.len();
        let mut in_comment = false;
        let mut deep = 0;
        let mut string = "".to_string();
        let indent = FormatXml::create_indent_arr(nl.clone());
        let mut start_tag_map: BTreeSet<(String, usize)> = BTreeSet::new();

        for idx in 0..len {
            // start comment or <![CDATA[...]]> or <!DOCTYPE
            // Comments are output as they are, without trim
            if Regex::new(r"<!").unwrap().is_match(str_array[idx]) {
                in_comment = true;
                // end comment  or <![CDATA[...]]>
                let node;
                if Regex::new(r"-->").unwrap().is_match(str_array[idx]) || Regex::new(r"\]>").unwrap().is_match(str_array[idx]) || Regex::new(r"!DOCTYPE").unwrap().is_match(str_array[idx]) || Regex::new(r"!doctype").unwrap().is_match(str_array[idx]) {
                    node = str_array[idx].trim();
                    in_comment = false;
                } else {
                    node = str_array[idx];
                }
                string = format!("{}{}{}", string, indent[deep], node);

                // end comment  or <![CDATA[...]]>
            } else if Regex::new(r"-->").unwrap().is_match(str_array[idx]) || Regex::new(r"\]>").unwrap().is_match(str_array[idx]) {
                string += str_array[idx];
                in_comment = false;

                // function
            } else if fmt_type == FmtType::HTML && (Regex::new(r"\(function\(").unwrap().is_match(str_array[idx]) || Regex::new(r"function\s").unwrap().is_match(str_array[idx])) {
                string = if !in_comment { format!("{}{}{}", string, indent[deep], str_array[idx]) } else { format!("{}{}", string, str_array[idx]) };

                // <elm>..</elm> or <elm></elm>
            } else if idx > 0 && Regex::new(r"^<\w").unwrap().is_match(str_array[idx - 1]) && Regex::new(r"^</\w").unwrap().is_match(str_array[idx]) && Regex::new(r"^<[\w:\-\.,]+").unwrap().captures(str_array[idx - 1]).unwrap()[0] == Regex::new(r"</[\w:\-\.,]+").unwrap().captures(str_array[idx]).unwrap()[0].replace("/", "") {
                string += &FormatXml::remove_space_between_tag_and_elm(in_comment, str_array[idx], r"</[\s\S]*?>");
                FormatXml::unmemorize_tag(str_array[idx].trim(), &mut start_tag_map);
                if !in_comment {
                    deep = if deep == 0 { 0 } else { deep - 1 };
                }

                // <elm>
            } else if Regex::new(r"<\w").unwrap().is_match(str_array[idx]) && !Regex::new(r"</").unwrap().is_match(str_array[idx]) && !Regex::new(r"/>").unwrap().is_match(str_array[idx]) {
                let node = &FormatXml::remove_space_between_tag_and_elm(in_comment, str_array[idx], r"<[\s\S]*?>");
                string = if in_comment { format!("{}{}", string, node) } else { format!("{}{}{}", string, indent[deep], node) };

                FormatXml::memorize_tag(str_array[idx].trim(), &mut start_tag_map, deep);
                if !in_comment && FormatXml::is_exist_end_tag(str_array[idx].trim(), &edit_text_non_comment) {
                    deep += 1;
                }
                // </elm>
            } else if Regex::new("</").unwrap().is_match(str_array[idx]) {
                if let Some(start_tag_indent_deep) = FormatXml::unmemorize_tag(str_array[idx].trim(), &mut start_tag_map) {
                    if !in_comment {
                        deep = if deep < start_tag_indent_deep { deep - 1 } else { start_tag_indent_deep };
                    }
                } else if !in_comment {
                    deep = if deep == 0 { 0 } else { deep - 1 };
                }

                string = if in_comment { format!("{}{}", string, str_array[idx]) } else { format!("{}{}{}", string, indent[deep], str_array[idx].trim()) };

                // <elm/>
            } else if Regex::new(r"/>").unwrap().is_match(str_array[idx]) {
                string = if !in_comment { format!("{}{}{}", string, indent[deep], str_array[idx]) } else { format!("{}{}", string, str_array[idx]) };

                // <? xml ... ?> || xmlns
            } else if Regex::new(r"<\?").unwrap().is_match(str_array[idx]) || Regex::new(r"xmlns:").unwrap().is_match(str_array[idx]) || Regex::new(r"xmlns=").unwrap().is_match(str_array[idx]) {
                string = format!("{}{}{}", string, indent[deep], str_array[idx].trim());
            } else {
                string += str_array[idx].trim();
            }
        }
        string.replacen(&nl, "", 1)
    }

    fn remove_space_between_tag_and_elm(in_comment: bool, tgt_node: &str, regex: &str) -> String {
        let node = if in_comment {
            tgt_node.to_string()
        } else {
            // Trim the element
            let cap = Regex::new(regex).unwrap().captures(tgt_node).unwrap();
            let tag = cap[0].to_string();
            let elm = tgt_node.replace(&tag, "").trim().to_string();
            format!("{}{}", tag, elm)
        };
        node
    }
    // Support for HTML optional tags
    fn is_exist_end_tag(node: &str, string: &str) -> bool {
        // let OPTIONAL_TAG_VEC: Vec<&'static str> = vec!["p", "dt", "dd", "li", "option", "thead", "tfoot", "th", "tr", "td", "rt", "rp", "optgroup", "caption"];

        let caps = Regex::new(r"<\s{0,}\w*").unwrap().captures(node).unwrap();
        let tag = caps[0].to_string().replace("<", "").replace("/", "");

        let end_tag_prefix = r"<\s{0,}/\s{0,}";
        let regex = format!("{}{}", end_tag_prefix, tag);
        let caps = Regex::new(&regex).unwrap().captures(string);

        caps.is_some()
    }

    // Save to  match the indentation depth of the start and end tags
    fn memorize_tag(node: &str, start_tag_map: &mut BTreeSet<(String, usize)>, indent_deep: usize) {
        // Get tag name
        let caps = Regex::new(r"<\s{0,}\w*\s{0,}").unwrap().captures(node).unwrap();
        let node_tag = caps[0].to_string();
        let tagnm = node_tag.replace("<", "").trim().to_string();

        start_tag_map.insert((tagnm, indent_deep));
    }

    fn unmemorize_tag(node: &str, start_tag_map: &mut BTreeSet<(String, usize)>) -> Option<usize> {
        let mut rtn_indent_deep: Option<usize> = None;
        // Get tag name
        let caps = Regex::new(r"<\s{0,}/\s{0,}\w*\s{0,}").unwrap().captures(node).unwrap();
        let mut end_tag = caps[0].to_string();
        end_tag = end_tag.replace("<", "").replace("/", "");

        let (mut del_tag, mut del_indent) = ("".to_string(), USIZE_UNDEFINED);
        for (start_tag, indent_deep) in start_tag_map.iter().rev() {
            if start_tag == &end_tag {
                del_tag = start_tag.clone();
                del_indent = *indent_deep;
                rtn_indent_deep = Some(*indent_deep);
                break;
            }
        }
        start_tag_map.remove(&(del_tag, del_indent));

        rtn_indent_deep
    }

    fn create_indent_arr(nl: String) -> Vec<String> {
        let mut indent_arr = vec![nl]; // array of shifts
        for idx in 0..100 {
            indent_arr.push(format!("{}{}", indent_arr[idx], &Cfg::get().general.editor.format.indent));
        }
        indent_arr
    }
}
