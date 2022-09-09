#[derive(Debug)]

pub struct Bill {
    pub name: String,
    pub amount: f32,
}

impl Bill {
    pub fn print(&self) {
        println!("{:?}", self)
    }
    pub fn update_amount(&mut self, amount: f32) {
        self.amount = amount;
    }
}
