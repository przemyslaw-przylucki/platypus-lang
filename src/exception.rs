pub struct Exception {

}

impl Exception {
    pub fn throw(message: String, line: usize) -> Result<(), String> {
        return Err(format!("Error at line {}: {}", line, message))
    }
}