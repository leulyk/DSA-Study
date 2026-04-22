impl Solution {
    pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
        strs.sort();

        let initial: Vec<char> = strs[0].chars().collect();
        let last: Vec<char> = strs[strs.len() - 1].chars().collect();

        let mut index = 0;
        let min_length = std::cmp::min(initial.len(), last.len());
        
        while index < min_length && initial[index] == last[index] {
            index += 1;
        }

        strs[0][..index].to_owned()
    }
}
