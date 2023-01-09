use std::collections::HashMap;

pub fn group_anagrams(strs:Vec<&str>)->Vec<Vec<&str>> {
    let mut map: HashMap<[u8;26],Vec<&str>> = HashMap::new();

    for s in strs {
        let mut key = [0_u8;26];
        for c in s.chars() {
            key[c as usize - 'a' as usize]+=1;
        }
        if let Some(vals) = map.get_mut(&key){
            vals.push(s)
        }else {
            map.insert(key, vec![s]);
        }
    }
    
    map.into_values().collect()
}

