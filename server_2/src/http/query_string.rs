use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>> // type of key and type of value
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str), 
    Multiple(Vec<&'buf str>), //a vector is a heap allocated array, allowing it to grow dynamically a we don't know the required size at compile time
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    } //the &self is a reference to the query string as this is a method created for the QS
}

// a=&b=2&c&d=&e===&d=7&d=abc

impl<'buf> From<&'buf str> for QueryString<'buf> { // using From instead of TryFrom as this conversion cannot fail, any string that goes in would be valid
    fn from(s: &'buf str) -> Self {

        let mut data = HashMap::new();

        for sub_str in s.split('&') {// since .split iterates, we can use a for loop on it
            let mut key = sub_str;
            let mut val = ""; //initial value set to empty string to catch for cases where there's nothing after the key after the &, e.g. initial 'c' above
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i+1..];

            } //since we only care if it finds an '=', we use if let to unpack the option

            data.entry(key)
            .and_modify(|existing: &mut Value| match existing {
                Value::Single(prev_value) => {
                    // let mut vec = Vec::new();
                    // vec.push(val);
                    // vec.push(prev_value);
                    // instead of above, we use a special macro from Rust called vec![]
                    // let mut vec = vec![prev_value, val] // so, we must change the existing value, so instead, we use code below
                    *existing = Value::Multiple(vec![prev_value, val]); 
                    // with the * we are derefencing, which basically says, go to the place of the pointer, and replace it with this new value
                    // we know this is safe because all the variants of an enum take up the same space, so we know mult variant fit into space of single var
                },
                Value::Multiple(vec) => vec.push(val)
            })
            .or_insert(Value::Single(val)); // or_insert checks to see if key is not there, if not, it is created
        }
        QueryString {data}
    }
}