impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut res : String = String::new();
        // for each string
        for s in strs {
            //// write the len then #
            res.push_str(&s.len().to_string());
            res.push('#');
            res.push_str(&s);
        }  
        return res
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut res = Vec::new();
        let mut s = s.as_str(); 

        while !s.is_empty() {
            let Some(i) = s.find('#') else {break;};
            let Ok(n) = s[..i].parse::<usize>() else {break;};
            let start = i + 1;
            let end = start + n;
            if end > s.len() {break;}
            res.push(s[start..end].to_string());
            s = &s[end..]; 
        }
        res
    }
}
