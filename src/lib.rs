use std::fs;

pub fn urls_to_js(urls: Vec<String>) -> String {
        let mut result = String::new();
        for url in urls {
                let file_size = match fs::metadata(&url) {
                        Ok(result) => result.len(), 
                        _ => 0
                };
                result = format!("{},{{url:'{}',size:'{}'}}",
                result,
                url,
                file_size
                );
        }
        return match result.len() {
                0 => result,
                _ => result[1..].to_owned()
        };
}