mod parser;
mod utils;

use std::io;

fn main() -> io::Result<()> {
    // 再帰的に探索するディレクトリ
    let directories = [
        "config/etc/sysctl.d",
        "config/run/sysctl.d",
        "config/usr/local/lib/sysctl.d",
        "config/usr/lib/sysctl.d",
        "config/lib/sysctl.d",
        "config/etc",
        "config",
    ];

    parser::parse_all_sysctl_files(&directories)?;

    Ok(())
}
