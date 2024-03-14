use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub struct QueryString<'buflifetime> {
    data: HashMap<&'buflifetime str, Value<'buflifetime>>
    //we may need to represent different data types therefore an enum. Example of why: if we get a query like: a=1&b=2&c&d=&e=7&d=abc then c has to be ==, d has multiples.
}
#[derive(Debug)]
pub enum Value<'buflifetime> {
    Single(&'buflifetime str),
    Multiple(Vec<&'buflifetime str>), //gonna have to heap allocate this
}

impl<'buflifetime> QueryString<'buflifetime> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buflifetime> From<&'buflifetime str> for QueryString<'buflifetime> {
    fn from(s:&'buflifetime str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&'){
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i+1..]
            }

            //handle if it exists, what type it is etc

            data.entry(key)
            .and_modify(|existing: &mut Value| match existing {
                Value::Single(previous_val) => {
                    *existing = Value::Multiple(vec![previous_val,val]);

                }
                Value::Multiple(vec) => vec.push(val)
            }) //takes anon func
            .or_insert(Value::Single(val)); //does exist? if not then insert - case of new key
        }
        QueryString {data}

    }
}