use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut out: HashSet<&'a str> = HashSet::new();
    let mut hi = Vec::new();
    for &s in possible_anagrams {
       hi.push(s);
    }
    for h in hi {
    let yes = test_a(word, h);
    if yes {
        out.insert(h);
    }
    
    }
    out
        }
pub fn test_a(word: &str, test: &str) -> bool {
    if word.len() != test.len() {
        return false;
    }
    
    let word = word.to_lowercase();
    let test = test.to_lowercase();
   if word == test {
       return false;
   }
    
    let mut out = false;
    let mut w_one: Vec<char> = Vec::new();
    for c in word.chars(){
       w_one.push(c);
    }
    let mut testt: Vec<char> = Vec::new();
    for c in test.chars(){
       testt.push(c);
    }
    
    out = recite(testt, w_one);
    out
}
    
    
    
    
    
    
    
    
    
    
    pub fn recite(test: Vec<char>, w_one: Vec<char>) -> bool {
        if test.len() == 1 && w_one.len() == 1 {
            if test[0] == w_one[0] {
                return true
            } else {
                return false
            }
        }
        let mut out = false;
        let h = test[0];
        let l = w_one.len();
        let mut index = 0;
        for x in 0..l {
            let y = w_one[x];
            if y == h {
                out = true;
                index = x;
                break;
            }
        }
        if out {
 
                let mut test = test;
let mut w_one = w_one;
let _ = test.remove(0);
let _ = w_one.remove(index);
out = recite(test, w_one);
            
        } else {
            out = false;
        }
        out
    }