use assert_cmd::Command;

fn bat_with_config() -> Command {
    let mut cmd = Command::cargo_bin("bat").unwrap();
    cmd.current_dir("tests/examples");
    cmd.env_remove("PAGER");
    cmd.env_remove("BAT_PAGER");
    cmd.env_remove("BAT_CONFIG_PATH");
    cmd.env_remove("BAT_STYLE");
    cmd.env_remove("BAT_THEME");
    cmd.env_remove("BAT_TABS");
    cmd
}

fn bat() -> Command {
    let mut cmd = bat_with_config();
    cmd.arg("--no-config");
    cmd
}

#[test]
fn basic() {
    bat()
        .arg("test.txt")
        .assert()
        .success()
        .stdout("hello world\n")
        .stderr("");
}

#[test]
fn stdin() {
    bat()
        .write_stdin("foo\nbar\n")
        .assert()
        .success()
        .stdout("foo\nbar\n");
}

#[test]
fn concatenate() {
    bat()
        .arg("test.txt")
        .arg("test.txt")
        .assert()
        .success()
        .stdout("hello world\nhello world\n");
}

#[test]
fn concatenate_stdin() {
    bat()
        .arg("test.txt")
        .arg("-")
        .arg("test.txt")
        .write_stdin("stdin\n")
        .assert()
        .success()
        .stdout("hello world\nstdin\nhello world\n");
}

#[test]
fn concatenate_empty_first() {
    bat()
        .arg("empty.txt")
        .arg("test.txt")
        .assert()
        .success()
        .stdout("hello world\n");
}

#[test]
fn concatenate_empty_last() {
    bat()
        .arg("test.txt")
        .arg("empty.txt")
        .assert()
        .success()
        .stdout("hello world\n");
}

#[test]
fn concatenate_empty_both() {
    bat()
        .arg("empty.txt")
        .arg("empty.txt")
        .assert()
        .success()
        .stdout("");
}

#[test]
fn concatenate_empty_between() {
    bat()
        .arg("test.txt")
        .arg("empty.txt")
        .arg("test.txt")
        .assert()
        .success()
        .stdout("hello world\nhello world\n");
}

#[test]
fn concatenate_empty_first_and_last() {
    bat()
        .arg("empty.txt")
        .arg("test.txt")
        .arg("empty.txt")
        .assert()
        .success()
        .stdout("hello world\n");
}

#[test]
fn concatenate_single_line() {
    bat()
        .arg("single-line.txt")
        .arg("single-line.txt")
        .assert()
        .success()
        .stdout("Single LineSingle Line");
}

#[test]
fn concatenate_single_line_empty() {
    bat()
        .arg("single-line.txt")
        .arg("empty.txt")
        .arg("single-line.txt")
        .assert()
        .success()
        .stdout("Single LineSingle Line");
}

#[test]
fn line_numbers() {
    bat()
        .arg("multiline.txt")
        .arg("--style=numbers")
        .arg("--decorations=always")
        .assert()
        .success()
        .stdout("   1 line 1\n   2 line 2\n   3 line 3\n   4 line 4\n");
}

#[test]
fn line_range_2_3() {
    bat()
        .arg("multiline.txt")
        .arg("--line-range=2:3")
        .assert()
        .success()
        .stdout("line 2\nline 3\n");
}

#[test]
fn line_range_first_two() {
    bat()
        .arg("multiline.txt")
        .arg("--line-range=:2")
        .assert()
        .success()
        .stdout("line 1\nline 2\n");
}

#[test]
fn line_range_last_3() {
    bat()
        .arg("multiline.txt")
        .arg("--line-range=2:")
        .assert()
        .success()
        .stdout("line 2\nline 3\nline 4\n");
}

#[test]
fn line_range_multiple() {
    bat()
        .arg("multiline.txt")
        .arg("--line-range=1:2")
        .arg("--line-range=4:4")
        .assert()
        .success()
        .stdout("line 1\nline 2\nline 4\n");
}

#[test]
fn tabs_numbers() {
    bat()
        .arg("tabs.txt")
        .arg("--tabs=4")
        .arg("--style=numbers")
        .arg("--decorations=always")
        .assert()
        .success()
        .stdout(
            "   1     1   2   3   4
   2 1   ?
   3 22  ?
   4 333 ?
   5 4444    ?
   6 55555   ?
   7 666666  ?
   8 7777777 ?
   9 88888888    ?
",
        );
}

#[test]
fn tabs_passthrough_wrapped() {
    bat()
        .arg("tabs.txt")
        .arg("--tabs=0")
        .arg("--style=plain")
        .arg("--decorations=always")
        .assert()
        .success()
        .stdout(
            "	1	2	3	4
1	?
22	?
333	?
4444	?
55555	?
666666	?
7777777	?
88888888	?
",
        );
}

#[test]
fn tabs_4_wrapped() {
    bat()
        .arg("tabs.txt")
        .arg("--tabs=4")
        .arg("--style=plain")
        .arg("--decorations=always")
        .assert()
        .success()
        .stdout(
            "    1   2   3   4
1   ?
22  ?
333 ?
4444    ?
55555   ?
666666  ?
7777777 ?
88888888    ?
",
        );
}

#[test]
fn tabs_8_wrapped() {
    bat()
        .arg("tabs.txt")
        .arg("--tabs=8")
        .arg("--style=plain")
        .arg("--decorations=always")
        .assert()
        .success()
        .stdout(
            "        1       2       3       4
1       ?
22      ?
333     ?
4444    ?
55555   ?
666666  ?
7777777 ?
88888888        ?
",
        );
}

#[test]
fn tabs_passthrough() {
    bat()
        .arg("tabs.txt")
        .arg("--tabs=0")
        .arg("--style=plain")
        .arg("--decorations=always")
        .assert()
        .success()
        .stdout(
            "	1	2	3	4
1	?
22	?
333	?
4444	?
55555	?
666666	?
7777777	?
88888888	?
",
        );
}

#[test]
fn tabs_4() {
    bat()
        .arg("tabs.txt")
        .arg("--tabs=4")
        .arg("--style=plain")
        .arg("--decorations=always")
        .assert()
        .success()
        .stdout(
            "    1   2   3   4
1   ?
22  ?
333 ?
4444    ?
55555   ?
666666  ?
7777777 ?
88888888    ?
",
        );
}

#[test]
fn tabs_8() {
    bat()
        .arg("tabs.txt")
        .arg("--tabs=8")
        .arg("--style=plain")
        .arg("--decorations=always")
        .assert()
        .success()
        .stdout(
            "        1       2       3       4
1       ?
22      ?
333     ?
4444    ?
55555   ?
666666  ?
7777777 ?
88888888        ?
",
        );
}

#[test]
fn fail_non_existing() {
    bat().arg("non-existing-file").assert().failure();
}

#[test]
fn fail_directory() {
    bat().arg("sub_directory").assert().failure();
}

#[test]
fn do_not_exit_directory() {
    bat()
        .arg("sub_directory")
        .arg("test.txt")
        .assert()
        .stdout("hello world\n")
        .failure();
}

#[test]
fn pager_basic() {
    bat()
        .env("PAGER", "echo pager-output")
        .arg("--paging=always")
        .arg("test.txt")
        .assert()
        .success()
        .stdout("pager-output\n");
}

#[test]
fn pager_overwrite() {
    bat()
        .env("PAGER", "echo other-pager")
        .env("BAT_PAGER", "echo pager-output")
        .arg("--paging=always")
        .arg("test.txt")
        .assert()
        .success()
        .stdout("pager-output\n");
}

#[test]
fn pager_disable() {
    bat()
        .env("PAGER", "echo other-pager")
        .env("BAT_PAGER", "")
        .arg("--paging=always")
        .arg("test.txt")
        .assert()
        .success()
        .stdout("hello world\n");
}

#[test]
fn config_location_test() {
    bat_with_config()
        .env("BAT_CONFIG_PATH", "bat.conf")
        .arg("--config-file")
        .assert()
        .success()
        .stdout("bat.conf\n");
}

#[test]
fn config_read_arguments_from_file() {
    bat_with_config()
        .env("BAT_CONFIG_PATH", "bat.conf")
        .arg("test.txt")
        .assert()
        .success()
        .stdout("dummy-pager-from-config\n");
}

#[test]
fn utf16() {
    // The output will be converted to UTF-8 with a leading UTF-8 BOM
    bat()
        .arg("--plain")
        .arg("--decorations=always")
        .arg("test_UTF-16LE.txt")
        .assert()
        .success()
        .stdout(std::str::from_utf8(b"\xEF\xBB\xBFhello world\n").unwrap());
}

#[test]
fn can_print_file_named_cache() {
    bat_with_config()
        .arg("cache")
        .assert()
        .success()
        .stdout("test\n")
        .stderr("");
}

#[test]
fn can_print_file_named_cache_with_additional_argument() {
    bat_with_config()
        .arg("cache")
        .arg("test.txt")
        .assert()
        .success()
        .stdout("test\nhello world\n")
        .stderr("");
}

#[test]
fn can_print_file_starting_with_cache() {
    bat_with_config()
        .arg("cache.c")
        .assert()
        .success()
        .stdout("test\n")
        .stderr("");
}

#[test]
fn does_not_print_unwanted_file_named_cache() {
    bat_with_config().arg("cach").assert().failure();
}

#[test]
fn unicode_wrap() {
    bat_with_config()
        .arg("unicode-wrap.txt")
        .arg("--style=numbers,snip")
        .arg("--decorations=always")
        .arg("--terminal-width=40")
        .assert()
        .success()
        .stdout(
            "   1 ビタミンA  ビタミンD  ビタミンE  ビ
     タミンK  ビタミンB1  ビタミンB2  ナ
     イアシン  パントテン酸  ビタミンB6 
      ビタミンB12  葉酸  ビオチン  ビタ
     ミンC
   2 
   3 고양이 고양이 고양이 고양이 고양이 
     고양이 고양이 고양이 고양이 고양이 
     고양이 고양이 고양이 고양이 고양이 
     고양이 고양이 고양이 고양이 고양이 
     고양이 고양이 고양이 고양이 고양이 
     고양이 고양이 고양이 고양이 고양이 
     고양이 고양이 고양이 고양이 고양이 
     고양이 고양이 고양이 고양이 고양이 
     고양이
   4 
   5 1 บวก 2 บวก 3 บวก 4 บวก 5 บวก 6 บวก
      7 บวก 8 บวก 9 บวก 10 บวก 11 บวก 12
      บวก 13 บวก 14 บวก 15 บวก 16 บวก 17
      บวก 18 บวก 19 บวก 20
   6 
   7 Бельгия Болгария Чехия Дания Герман
     ия Эстония Ирландия Греция Испания 
     Франция Хорватия Италия Кипр Латвия
      Литва Люксембург Венгрия Мальта Ни
     дерланды Австрия Польша Португалия 
     Румыния Словения Словакия Финляндия
      Швеция Великобритания
",
        );
}

#[test]
fn snip() {
    bat()
        .arg("multiline.txt")
        .arg("--style=numbers,snip")
        .arg("--decorations=always")
        .arg("--line-range=1:2")
        .arg("--line-range=4:")
        .arg("--terminal-width=80")
        .assert()
        .success()
        .stdout(
            "   1 line 1
   2 line 2
 ...─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ 8< ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
   4 line 4
",
        );
}

#[test]
fn empty_file_leads_to_empty_output_with_grid_enabled() {
    bat()
        .arg("empty.txt")
        .arg("--style=grid")
        .arg("--decorations=always")
        .arg("--terminal-width=80")
        .assert()
        .success()
        .stdout("");
}

#[test]
fn filename_basic() {
    bat()
        .arg("test.txt")
        .arg("--decorations=always")
        .arg("--style=header")
        .arg("-r=0:0")
        .arg("--file-name=foo")
        .assert()
        .success()
        .stdout("File: foo\n")
        .stderr("");
}

#[test]
fn filename_binary() {
    bat()
        .arg("test.binary")
        .arg("--decorations=always")
        .arg("--style=header")
        .arg("-r=0:0")
        .arg("--file-name=foo")
        .assert()
        .success()
        .stdout("File: foo   <BINARY>\n")
        .stderr("");
}

#[test]
fn filename_stdin() {
    bat()
        .arg("--decorations=always")
        .arg("--style=header")
        .arg("-r=0:0")
        .arg("-")
        .write_stdin("stdin\n")
        .arg("--file-name=foo")
        .assert()
        .success()
        .stdout("File: foo\n")
        .stderr("");
}

#[test]
fn filename_stdin_binary() {
    let vec = vec![0; 1];
    bat_with_config()
        .arg("--decorations=always")
        .arg("--style=header")
        .write_stdin(vec)
        .arg("--file-name=foo")
        .assert()
        .success()
        .stdout("File: foo   <BINARY>\n")
        .stderr("");
}

#[test]
fn filename_multiple_ok() {
    bat()
        .arg("--decorations=always")
        .arg("--style=header")
        .arg("-r=0:0")
        .arg("test.txt")
        .arg("--file-name=foo")
        .arg("single-line.txt")
        .arg("--file-name=bar")
        .assert()
        .success()
        .stdout("File: foo\nFile: bar\n")
        .stderr("");
}

#[test]
fn filename_multiple_err() {
    bat()
        .arg("--decorations=always")
        .arg("--style=header")
        .arg("-r=0:0")
        .arg("test.txt")
        .arg("--file-name=foo")
        .arg("single-line.txt")
        .assert()
        .failure();
}

#[cfg(target_os = "linux")]
#[test]
fn file_with_invalid_utf8_filename() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    bat()
        .arg(OsStr::from_bytes(b"test-invalid-utf8-\xC3(.rs"))
        .assert()
        .success();
}

#[test]
fn do_not_panic_regression_tests() {
    for filename in &[
        "issue_28.md",
        "issue_190.md",
        "issue_314.hs",
        "issue_914.rb",
        "issue_915.vue",
    ] {
        bat()
            .arg("--color=always")
            .arg(&format!("regression_tests/{}", filename))
            .assert()
            .success();
    }
}
