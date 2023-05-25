pub struct RootController {
}

impl RootController {
    pub async fn get_message() -> String {
        format!("Welcome to the Brewtooth-Pi server!")
    }
}