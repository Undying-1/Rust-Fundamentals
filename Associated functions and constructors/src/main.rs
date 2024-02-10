

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    email: String,
    phone_number: String,
} 


impl Person {
    fn print_full_name(&self) -> String{
        return format!("{} {}", self.first_name, self.last_name);  
    }
}


#[derive(Debug)]
struct User {
    username: String,
    email: String,
    uri: Option<String>,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri: Some(uri),
            active: true,
        }
    }

    fn from_email(email: String) -> Self {
        let temp = email.clone();
        let mut parts = temp.split("@");
        Self{
            username: parts.next().unwrap().to_string(),
            email,
            uri: Some(parts.next().unwrap().to_string()),
            active: true,
        }
    }

    fn deactivate_user(&mut self){
        self.active = false;
    }

    fn update_email(&mut self, uri: String){
        self.uri = Some(uri);
    }
}


fn main() {
    let person = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 25,
        email: "user@example.com".to_string(),
        phone_number: "+99899100530".to_string(),
    };
    println!("{:#?}", person);

    println!("full name is: {}", person.print_full_name());

    println!("{} {}", person.first_name, person.last_name);


    let mut user =  User::from_email(person.email);

    user.update_email("https://google.com/".to_string());
    println!("{:?}", user)
}