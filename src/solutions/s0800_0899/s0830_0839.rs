pub struct Solution;

impl Solution {
    /// 833. 字符串中的查找与替换
    #[allow(dead_code)]
    pub fn find_replace_string(
        s: String,
        indices: Vec<i32>,
        sources: Vec<String>,
        targets: Vec<String>,
    ) -> String {
        let mut res = String::new();
        let mut indices_map: std::collections::HashMap<i32, Vec<usize>> =
            std::collections::HashMap::new();
        indices.iter().enumerate().for_each(|(i, indice)| {
            if let Some(index) = indices_map.get_mut(indice) {
                index.push(i);
            } else {
                indices_map.insert(*indice, vec![i]);
            }
        });
        let mut i = 0;
        while i < s.len() {
            if let Some(indexes) = indices_map.get(&(i as i32)) {
                let mut replace_flag = false;
                for &index in indexes {
                    if s.get(i..i + sources[index].len()) == Some(sources[index].as_str()) {
                        res.push_str(&targets[index]);
                        i += sources[index].len();
                        replace_flag = true;
                    }
                }
                if !replace_flag {
                    res.push_str(&s[i..i + 1]);
                    i += 1;
                }
            } else {
                res.push_str(&s[i..i + 1]);
                i += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 833. 字符串中的查找与替换
    #[test]
    fn find_replace_string() {
        let s = "abcd".to_string();
        let indices = vec![0, 2];
        let sources = vec!["a".to_string(), "cd".to_string()];
        let targets = vec!["eee".to_string(), "ffff".to_string()];
        let res = Solution::find_replace_string(s, indices, sources, targets);
        assert_eq!(res, "eeebffff".to_string());
    }
}
