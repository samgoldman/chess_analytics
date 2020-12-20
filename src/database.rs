// Credit: https://stackoverflow.com/questions/59795662/insert-in-arbitrary-nested-hashmap

use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct Database {
    pub children: HashMap<String, Database>,
    pub data: Vec<i32>,
}

impl Database {
    pub fn insert_path(&mut self, path: Vec<String>) -> &mut Self {
        // node is a mutable reference to the current database
        let mut node = self;
        // iterate through the path
        for subkey in path.clone().iter() {
            // insert the new database object if necessary and
            // set node to (a mutable reference to) the child node
            node = node
                .children
                .entry(subkey.to_string())
                .or_insert_with(Database::default);
        }
        node
    }

    pub fn get_paths(&self) -> Vec<Vec<String>> {
        let mut out = vec![];
        self.deep_keys(vec![], &mut out);
        out
    }

    
    fn deep_keys(&self, current_path: Vec<String>, output: &mut Vec<Vec<String>>) {
        let mut has_children = false;
        for (k, v) in self.children.iter() {
            has_children = true;
            let mut new_path = current_path.clone();
            new_path.push(k.to_owned());
            v.deep_keys(new_path, output);
        }
        if !has_children && current_path.len() > 0 {
            output.push(current_path.clone());
        }

    }
}