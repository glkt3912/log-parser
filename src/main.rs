use clap::{Arg, ArgAction, Command};
use log_parser::{Config, LogParser, Result};
use std::path::PathBuf;

fn main() -> Result<()> {
    env_logger::init();

    let matches = Command::new("log-parser")
        .version("0.1.0")
        .about("高性能ログファイル解析・フィルタリングCLIツール")
        .arg(
            Arg::new("file")
                .help("ログファイルのパス")
                .required(true)
                .value_name("FILE")
                .index(1),
        )
        .arg(
            Arg::new("level")
                .long("level")
                .short('l')
                .help("ログレベルでフィルタ (error, warn, info, debug)")
                .value_name("LEVEL"),
        )
        .arg(
            Arg::new("since")
                .long("since")
                .help("開始日時 (例: '2024-01-01', '2024-01-01 12:00:00')")
                .value_name("DATETIME"),
        )
        .arg(
            Arg::new("until")
                .long("until")
                .help("終了日時 (例: '2024-01-31', '2024-01-31 23:59:59')")
                .value_name("DATETIME"),
        )
        .arg(
            Arg::new("grep")
                .long("grep")
                .short('g')
                .help("正規表現パターンで検索")
                .value_name("PATTERN"),
        )
        .arg(
            Arg::new("format")
                .long("format")
                .short('f')
                .help("出力形式 (text, json, csv)")
                .value_name("FORMAT")
                .default_value("text"),
        )
        .arg(
            Arg::new("follow")
                .long("follow")
                .short('F')
                .help("リアルタイム監視 (tail -f)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("stats")
                .long("stats")
                .help("統計情報を表示")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("tui")
                .long("tui")
                .help("インタラクティブTUIモード")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let file_path = PathBuf::from(matches.get_one::<String>("file").unwrap());

    let config = Config {
        file_path,
        level_filter: matches.get_one::<String>("level").cloned(),
        since: matches.get_one::<String>("since").cloned(),
        until: matches.get_one::<String>("until").cloned(),
        grep_pattern: matches.get_one::<String>("grep").cloned(),
        output_format: matches.get_one::<String>("format").unwrap().clone(),
        follow: matches.get_flag("follow"),
        show_stats: matches.get_flag("stats"),
        tui_mode: matches.get_flag("tui"),
    };

    let mut parser = LogParser::new(config)?;
    parser.run()
}
