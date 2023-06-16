use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new();
    }

    thread::scope(|s| {
        let mut handles = Vec::with_capacity(worker_count + 1);

        for chunk in input.chunks((input.len() / worker_count).max(1)) {
            handles.push(s.spawn(move || -> HashMap<char, usize> {
                let mut res: HashMap<char, usize> = HashMap::new();
                chunk.iter().flat_map(|line| line.chars()).for_each(|c| {
                    if c.is_alphabetic() {
                        *res.entry(c.to_ascii_lowercase()).or_default() += 1;
                    }
                });
                res
            }));
        }

        let mut res = HashMap::new();
        for handle in handles {
            let mp = handle.join().unwrap();
            for (k, v) in &mp {
                *res.entry(*k).or_default() += *v;
            }
        }

        res
    })
}
