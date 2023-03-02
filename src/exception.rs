struct Exception {

}

impl Exception {
    pub fn throw(self: Self, message: String, line: i32) {
        println!("Error: {}", message)
    }
}