pub const LANG_CONFIG: &'static str = r#"
[ja]
### 列、行の現在位置 ###
row="行"
col="列"
yes="はい"
no="いいえ"
cancel="キャンセル"
close="閉じる"
fixed="確定"
new_file="新規ファイル"
search="検索"
search_bottom="下部検索"
search_top="上部検索"
search_str="検索文字"
search_file="検索ファイル"
search_folder="検索フォルダ"
searching="検索中..."
all_replace="全置換"
move_input_field="入力箇所移動"
replace_char="置換文字"
unable_to_edit="編集不可"
open_target_file_in_another_terminal="対象のファイルを別のターミナルで開く"
key_recording="操作記録中..."
complement="補完"

## Msg ##
save_confirmation_to_close="保存して終了しますか？"
terminal_size_small="ターミナルサイズが小さい為に終了します。サイズを大きくして再実行して下さい"
set_new_filenm="新規に作成するファイル名を入力してください"
not_entered_filenm="ファイル名が入力されていません"
# Search・Replace
set_search="検索する文字を入力してください"
set_replace="検索する文字と置換する文字を入力して下さい"
set_grep="検索する文字と検索するファイル、検索するフォルダを入力して下さい"
not_entered_search_str="検索する文字が入力されていません"
not_entered_search_file="検索するファイルが入力されていません"
not_entered_search_folder="検索するフォルダが入力されていません"
not_entered_replace_str="置換する文字が入力されていません"
cannot_find_char_search_for="検索する文字が見つかりません"
long_time_to_search="検索対象ファイル多い場合に非常に時間がかかる場合があります"
show_search_result="検索結果を表示しています"
show_search_no_result="検索対象は1件も存在しませんでした"
# File open
no_read_permission="読み込み権限がありません"
no_write_permission="書込権限無し"
file_opening_problem="ファイルを開く際に問題が発生しました"
file_not_found="ファイルが存在しません"
# Save
file_already_exists="既に存在するファイル名です"
# Not sel range
no_sel_range="範囲を指定して下さい"
# paste
cannot_paste_multi_lines="複数行を貼り付けることは出来ません"

# undo・redo
no_undo_operation="元に戻す操作はありません"
no_operation_re_exec="再実行する操作はありません"
# key record
no_key_record_exec="実行する操作記録はありません"
# other
unsupported_operation="サポートされていない操作です"

[en]
row="row"
col="col"
yes="yes"
no="no"
cancel="cancel"
close="close"
fixed="fixed"
search="search"
search_bottom="bottom search"
search_top="top search"
search_str="search character"
search_file="search file"
search_folder="search folder"
searching="searching..."
new_file="new_file"
all_replace="all replace"
move_input_field="move input field"
replace_char="replace character"
unable_to_edit="unable to edit"
open_target_file_in_another_terminal="open target file in another terminal"
key_recording="operation recording..."
complement="complement"

## Msg ##
save_confirmation_to_close="Do you want to save and exit?"
terminal_size_small="It will end because the terminal size is small. Please increase the size and try again"
set_new_filenm="Enter the name of the newly created file"
not_entered_filenm="File name is not entered"
# Search・Replace
set_search="Enter the characters you want to search for"
set_replace="Enter the character to search for and the character to replace"
set_grep="Enter the characters to search, the files to search, and the folder to search"
not_entered_search_str="Search charctor is not entered"
not_entered_search_file="Search file is not entered"
not_entered_search_folder="Search folder is not entered"
not_entered_replace_str="Replace charctor is not entered"
cannot_find_char_search_for="Cannot find the character to search for"
long_time_to_search="It may take a long time if there are many files to be searched"
show_search_result="show search result"
show_search_no_result="There was no search target"
# File open
no_read_permission="No read permission"
no_write_permission="No write permission"
file_opening_problem="There was a problem in opening the file"
file_not_found="File not found"
# Save
file_already_exists="file already exists"
# Not sel range
no_sel_range="Please specify the copy range"
# paste
cannot_paste_multi_lines="Can not paste multilines"
# undo・redo
no_undo_operation="There is no undo operation"
no_operation_re_exec="There is no operation re-execute"
# key record
no_key_record_exec="There is no operation record to be executed"
# other
unsupported_operation="This is an unsupported operation"


"#;
