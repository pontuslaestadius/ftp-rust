pub struct Metadata<'a> {
    fields: Vec<&'a str>,
    values: Vec<&'a str>,
}

pub fn new<'b>(fields: Vec<&'b str>, values: Vec<&'b str>) -> Metadata<'b> {
    if fields.len() != values.len() {
        panic!("meta data fields needs to match the number of values");
    }
    Metadata {fields, values}
}


impl<'a> Metadata<'a> {
    // TODO should this take ownership?

    // Why call it format? Come up with something more generic.
    pub fn format(&self) -> String {
        let mut buffer = String::new();
        for i in 0..self.fields.len() {
            buffer.push_str(self.fields[i]);
            buffer.push(':');
            buffer.push_str(self.values[i]);
            buffer.push_str(";\n");
        }
        buffer
    }

}