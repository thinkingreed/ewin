pub const LANG_CONFIG: &'static str = r#"
ja:
  row: 行
  col: 列
  "yes": はい
  "no": いいえ
  cancel: キャンセル
  close: 閉じる
  fixed: 確定
  end: 終了
  save: 保存
  copy: コピー
  paste: ペースト
  undo: 元に戻す
  redo: 再実行
  cut: 切取
  grep: grep
  range_select: 範囲選択
  all_select: 全選択
  move_row: 行移動
  search: 検索
  search_bottom: 下部検索
  search_top: 上部検索
  search_str: 検索文字
  search_file: 検索ファイル
  search_folder: 検索フォルダ
  searching: 検索中...
  case_sens: 大/小文字区別
  new_file: 新規ファイル
  move_input_field: 入力箇所移動
  replace: 置換
  all_replace: 全置換
  replace_char: 置換文字
  complement: 補完
  unable_to_edit: 編集不可
  key_record_start: 記録開始
  key_record_stop: 記録終了
  key_recording: 操作記録中...
  help: help
  ## Msg ##
  save_confirmation_to_close: 保存して終了しますか？
  terminal_size_small: ターミナルサイズが小さい為に終了します。サイズを大きくして再実行して下さい
  set_new_filenm: 新規に作成するファイル名を入力してください
  set_search: 検索する文字を入力してください
  set_replace: 検索する文字と置換する文字を入力して下さい
  set_grep: 検索する文字と検索するファイル、検索するフォルダを入力して下さい
  set_move_row: 移動する行番号を半角数字で入力して下さい
  move_to_specified_row: 指定行に移動
  open_target_file_in_another_terminal: 対象のファイルを別のターミナルで開く
  not_entered_filenm: ファイル名が入力されていません
  not_entered_search_str: 検索する文字が入力されていません
  not_entered_search_file: 検索するファイルが入力されていません
  not_entered_search_folder: 検索するフォルダが入力されていません
  not_entered_replace_str: 置換する文字が入力されていません
  not_entered_row_number_to_move: 移動する行番号が入力されていません
  cannot_find_char_search_for: 検索する文字が見つかりません
  long_time_to_search: 検索対象ファイル多い場合に非常に時間がかかる場合があります
  show_search_result: 検索結果を表示しています
  show_search_no_result: 検索対象は1件も存在しませんでした
  no_undo_operation: 元に戻す操作はありません
  no_operation_re_exec: 再実行する操作はありません
  number_within_current_number_of_rows: 現在の行数内の数字を入力して下さい
  no_read_permission: 読み込み権限がありません
  no_write_permission: 書込権限無し
  file_opening_problem: ファイルを開く際に問題が発生しました
  file_not_found: ファイルが存在しません
  file_loading_failed: ファイルの読込に失敗しました
  file_parsing_failed: ファイルの解析に失敗しました
  file_already_exists: 既に存在するファイル名です
  no_sel_range: 範囲を指定して下さい
  no_value_in_clipboard: クリップボードに値はありません
  cannot_paste_multi_rows: 複数行を貼り付けることは出来ません
  no_key_record_exec: 実行する操作記録はありません
  unsupported_operation: サポートされていない操作です
en:
  row: row
  col: col
  "yes": "yes"
  "no": "no"
  cancel: cancel
  close: close
  fixed: fixed
  end: end
  save: save
  copy: copy
  paste: paste
  undo: undo
  redo: redo
  cut: cut
  grep: grep
  range_select: range select
  all_select: all select
  move_row: move row
  search: search
  search_bottom: bottom search
  search_top: top search
  search_str: search character
  search_file: search file
  search_folder: search folder
  searching: searching...
  case_sens: case sens
  new_file: new_file
  move_input_field: move input field
  replace: replace
  all_replace: all replace
  replace_char: replace character
  complement: complement
  unable_to_edit: unable to edit
  key_record_start: record start
  key_record_stop: record stop
  key_recording: operation recording...
  help: help
  ## Msg ##
  save_confirmation_to_close: Do you want to save and exit?
  terminal_size_small: It will end because the terminal size is small. Please increase the size and try again
  set_new_filenm: Enter the name of the newly created file
  set_search: Enter the characters you want to search for
  set_replace: Enter the character to search for and the character to replace
  set_grep: "Enter the characters to search, the files to search, and the folder to search"
  set_move_row: Enter the line number to move in half width numbers
  move_to_specified_row: move to specified line
  open_target_file_in_another_terminal: open target file in another terminal
  not_entered_filenm: File name is not entered
  not_entered_search_str: Search charctor is not entered
  not_entered_search_file: Search file is not entered
  not_entered_search_folder: Search folder is not entered
  not_entered_replace_str: Replace charctor is not entered
  not_entered_row_number_to_move: Line number to move is not entered 
  cannot_find_char_search_for: Cannot find the character to search for
  long_time_to_search: It may take a long time if there are many files to be searched
  show_search_result: show search result
  show_search_no_result: There was no search target
  no_undo_operation: There is no undo operation
  no_operation_re_exec: There is no operation re-execute
  number_within_current_number_of_rows: Enter a number within the current number of lines
  no_read_permission: No read permission
  no_write_permission: No write permission
  file_opening_problem: There was a problem in opening the file
  file_not_found: File not found
  file_loading_failed: file loading failed
  file_parsing_failed: file parsing failed
  file_already_exists: file already exists
  no_sel_range: Please specify the copy range
  no_value_in_clipboard: There is no value in the clipboard
  cannot_paste_multi_rows: Can not paste multilines
  no_key_record_exec: There is no operation record to be executed
  unsupported_operation: This is an unsupported operation

"#;
