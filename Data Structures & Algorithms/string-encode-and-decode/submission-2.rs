impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut encoded = String::new();

        for str in strs {
            encoded.push_str(&format!("{}#{}", str.len(), str));
        }
    
        encoded
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut decoded: Vec<String> = vec![];
        let mut start_index: usize = 0;
        let mut end_pos = s.find("#");
    
        while let Some(pos) = end_pos {
            let end_index = start_index + pos;
            let current_length = *(&s[start_index..end_index].parse::<usize>().unwrap());
            let str_length = end_index + current_length + 1;
    
            let current_string = String::from(&s[end_index + 1..str_length]);
            decoded.push(current_string);
            start_index = str_length;
            end_pos = *&s[(str_length)..].find("#");
        }
    
        decoded
    }
}
