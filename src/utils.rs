use rustc_hash::FxHashMap;

/// FxHashMapの内容を出力
pub fn display_map(map: &FxHashMap<String, FxHashMap<String, String>>) {
    for (key, sub_map) in map {
        println!("{}", key);
        for (sub_key, value) in sub_map {
            println!("  {} {}", sub_key, value);
        }
        println!();
    }
}

