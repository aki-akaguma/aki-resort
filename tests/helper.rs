#[allow(unused_macros)]
macro_rules! help_msg {
    () => {
        concat!(
            version_msg!(),
            "\n",
            indoc::indoc!(
                r#"
            Usage:
              aki-resort [options]

            sort lines of text.

            Ordering options:
              -r, --reverse                 reverse the result of comparisons
                  --according-to <word>     sort according to <word>
              -h, --head <num>              unsort the first <num> lines.
              -t, --tail <num>              unsort the last <num> lines.

            Other options:
                  --color <when>            use markers to highlight the matching strings
              -e, --exp <exp>               regular expression. sort by the entires match
              -u, --unique                  output only the first line of an equal
                  --max-buffer <size>       max buffer size

              -H, --help        display this help and exit
              -V, --version     display version information and exit
              -X <x-options>    x options. try -X help

            Option Parameters:
              <word>    'month', 'numeric', 'string', 'time', 'version'
              <when>    'always', 'never', or 'auto'
              <exp>     regular expression, sort by the entires match.
              <size>    if a reading size is more than <size>, then it is not output,
                        quit and display error message.

            Environments:
              AKI_RESORT_COLOR_SEQ_ST   color start sequence specified by ansi
              AKI_RESORT_COLOR_SEQ_ED   color end sequence specified by ansi

            Examples:
              This sort via utf-8 code:
                cat file1.txt | aki-resort
              This sort via 1st chunk of numeric character according to numeric:
                cat file1.txt | aki-resort -e "[0-9]+" --according-to numeric
              This sort via 1st chunk of numeric character according to month:
                cat file1.txt | aki-resort -e ":([^:]+)$" --according-to month
              This sort via 1st chunk of numeric version character according to version:
                cat file1.txt | aki-resort -e "[^:]+:[^:]+:([0-9.]+):" --according-to version
              This sort via 1st chunk of numeric time character according to time:
                cat file1.txt | aki-resort -e "([0-9]+:([0-9]+:)?[0-9]+(.[0-9]+)?)" --according-to time
            "#
            ),
            "\n",
        )
    };
}

#[allow(unused_macros)]
macro_rules! x_help_msg {
    () => {
        concat!(
            indoc::indoc!(
                r#"
            Options:
              -X rust-version-info     display rust version info and exit
            "#
            ),
            "\n",
        )
    };
}

#[allow(unused_macros)]
macro_rules! try_help_msg {
    () => {
        "Try --help for help.\n"
    };
}

#[allow(unused_macros)]
macro_rules! program_name {
    () => {
        "aki-resort"
    };
}

#[allow(unused_macros)]
macro_rules! version_msg {
    () => {
        concat!(program_name!(), " ", env!("CARGO_PKG_VERSION"), "\n")
    };
}

#[allow(unused_macros)]
macro_rules! fixture_invalid_utf8 {
    () => {
        "fixtures/invalid_utf8.txt"
    };
}

#[allow(unused_macros)]
macro_rules! fixture_fruit {
    () => {
        "fixtures/fruit.txt"
    };
}

#[allow(unused_macros)]
macro_rules! fixture_fruit_header {
    () => {
        "fixtures/fruit_header.txt"
    };
}

#[allow(unused_macros)]
macro_rules! fixture_fruit_footer {
    () => {
        "fixtures/fruit_footer.txt"
    };
}

#[allow(unused_macros)]
macro_rules! fixture_time {
    () => {
        "fixtures/time.txt"
    };
}

/*
#[allow(unused_macros)]
macro_rules! fixture_text10k {
    () => {
        "fixtures/text10k.txt"
    };
}
*/

#[allow(unused_macros)]
macro_rules! color_start {
    //() => { "\u{1B}[01;31m" }
    () => {
        "<S>"
    };
}

#[allow(unused_macros)]
macro_rules! color_end {
    //() => {"\u{1B}[0m"}
    () => {
        "<E>"
    };
}

#[allow(unused_macros)]
macro_rules! env_1 {
    () => {{
        vec![
            ("AKI_RESORT_COLOR_SEQ_ST", color_start!()),
            ("AKI_RESORT_COLOR_SEQ_ED", color_end!()),
        ]
    }};
}
