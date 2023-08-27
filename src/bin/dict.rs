use std::io::BufRead;

/**
 * DICT - Search in the dictionary!
 *
 * Eloy the byte is helping his son, Marcos the little one, with the homework, which consists on: Given one or more words
 * you should search in the dictionary which word contains the proper prefix of the word given, for example 'set' is a
 * prefix of 'setter', but also of 'setting'.
 *
 * Now, neither Eloy nor Marcos wants to search the words in the dictionary (they just won’t do it.) Marcos will give you
 * the words in the dictionary and the words to find the prefix of. You should make a program that given this, output the
 * list of words that contains as a prefix the word given, Marcos can make mistakes sometimes, so, be careful! Marcos won’t
 * always be telling the real prefixes all the time, in addition, sometimes Marcos can repeat the same word (while reading
 * the dictionary), in this case, the word should be treated as it was mentioned just one time.
 *
 * Input
 * The input will consist in an integer N (1 <= N <= 25.000), next, N lines will follow containing a single word (maximum of
 * 20 characters (all lowercase letters)), after that, there will be an integer K (1 <= K <= 22.000) containing the number
 * of words to look in the dictionary, then, K lines containing the prefix-word.
 *
 * Output
 * The output will consist in displaying the words that are composed from the word given, you should output “Case #i:” where
 * i is the i-th case, then, at the next line, output the list of the composed words (lexicographical-ordered), if there is
 * none, you should output “No match.”
 *
 * Sample
 *   Input:
 *   5
 *   set
 *   lol
 *   setter
 *   setting
 *   settings
 *   2
 *   set
 *   happy
 *   
 *   Output:
 *   Case #1:
 *   setter
 *   setting
 *   settings
 *   Case #2:
 *   No match.
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let mut trie = Trie::new();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.by_ref().take(n) {
        let line = line.unwrap();
        trie.insert(line.as_bytes());
    }

    let k: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for (i, line) in lines.take(k).enumerate() {
        let line = line.unwrap();

        let mut result = vec![];
        trie.query(line, &mut result);

        println!("Case #{}:", i + 1);
        if result.is_empty() {
            println!("No match.");
        } else {
            for word in result {
                println!("{}", word);
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Trie {
    children: Vec<Option<Trie>>,
    word: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            children: vec![None; 26],
            word: false,
        }
    }

    fn insert(&mut self, word: &[u8]) {
        if word.is_empty() {
            self.word = true;
            return;
        }

        let i = (word[0] - b'a') as usize;
        let child = match self.children[i].as_mut() {
            Some(child) => child,
            None => {
                self.children[i] = Some(Trie::new());
                self.children[i].as_mut().unwrap()
            }
        };

        child.insert(&word[1..]);
    }

    fn query(&self, mut request: String, result: &mut Vec<String>) {
        fn query_inner(trie: &Trie, word: &mut String, result: &mut Vec<String>, add: bool) {
            if trie.word && add {
                result.push(word.clone());
            }

            for (i, child) in trie.children.iter().enumerate() {
                if let Some(child) = child {
                    let letter = (i as u8 + b'a') as char;
                    word.push(letter);
                    query_inner(child, word, result, true);
                    word.pop();
                }
            }
        }

        // Finding ending child
        let mut root = Some(self);
        for letter in request.bytes() {
            let i = (letter - b'a') as usize;
            root = root.unwrap().children[i].as_ref();

            // If we can't find full word -> then there's no words with this start
            if root.is_none() {
                return;
            }
        }

        // Querying all the inner words
        query_inner(root.unwrap(), &mut request, result, false)
    }
}
