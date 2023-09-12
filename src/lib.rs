use pinyin::ToPinyin;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn han_to_pinyin(hans: &str) -> String {
    let mut results = String::new();
    for pinyin in hans.to_pinyin() {
        if let Some(pinyin) = pinyin {
            let result = pinyin.plain();
            results = format!("{}{}", results, result);
        }
    }
    results
}
