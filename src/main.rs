use std::collections::HashMap;
use std::io;

fn main() {
    let mut departments = HashMap::new();

    departments.insert("Engineering".to_string(),
    "Tom Riddle".to_string(),
    );
}

// write a function that allows user to select whether to add employees / retreive
// employees from database. If select to add, choose which department and then their
// name, from there push the name to the department that *matches*
// when retreiving employees from a department sort them alphabetically

// Using a hash map and vectors, create a text interface to allow a user to add
// employee names to a department in a company, ex:
// "Add Sally to Engineering", or "Add Amir to Sales".
// Then let the user retrieve a list of all people in a department or all people
// in the company by department sorted alphabetically.