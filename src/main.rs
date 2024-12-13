use std::fs::File;
use std::io;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};





#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Bill{
    name:String,
    amount:f64,
}

pub struct  Bills{
     inner:HashMap<String,Bill>
}

impl Bills{
    fn new()->Self{
        Self{
            inner:HashMap::new(),
        }
    }
    fn add(&mut self ,bill:Bill){
        self.inner.insert(bill.name.to_string(),bill);
    }

    fn get_all(&self)->Vec<&Bill>{
        self.inner.values().collect()
    }

    fn remove(&mut self,name:&str)->bool{
        self.inner.remove(name).is_some()
    }

    fn update(& mut self,name:&str,amount:f64)->bool{
        match self.inner.get_mut(name) {
            Some(bill)=>{
                bill.amount=amount;
                return true;
            }None=>return  false  
        };
    }
}
fn getInput()->Option<String>{
    let mut buffer=String::new();
    while io::stdin().read_line(&mut buffer).is_err(){
        println!("Please enter your data again");
    }
    let input=buffer.trim().to_string();
    if &input==""{
        None
    }else{
        Some(input)
    }
}

fn get_bill_amount()->Option<f64>{
    println!("Amount:");
    loop{
        let input=match getInput() {
            Some(input)=>input,
            None=>return None,
        };
        if &input==""{
            return None;
        }
        let parsed_int:Result<f64,_>=input.parse();
        match parsed_int{
            Ok(amount)=>return Some(amount),
            Err(_)=>println!("please enter a number")
        }
    }
}


mod menu{
    use crate::{getInput, get_bill_amount, Bill, Bills};

    pub fn add_bill(bills:&mut Bills){
        println!("Bill name:");
        let name=match getInput(){
            Some(input)=>input,
            None=>return,
        };
        println!("Bill amount:");
        let amount=match get_bill_amount(){ 
            Some(amount)=>amount,
            None=>return,
        };

        let bill=Bill{name,amount};
        bills.add(bill);
        println!("Bill added");
    }

    pub fn view_bills(bills:&Bills){
        for bill in bills.get_all(){
            println!("{:?}",bill);
        }
    }

    pub fn remove_bill(bills:&mut Bills){
        for bill in bills.get_all(){
            println!("{:?}",bill);
        }
        println!("Enter bill name to remove:");
        let name=match getInput(){
            Some(name)=>name,
            None=>return,
        }; 

        if bills.remove(&name) {
            println!("Bill Removed");
            
        }
        else{
            println!("Bill Not Found");
        }
    }

    pub fn update_bill(bills:&mut Bills){
        for bill in bills.get_all(){
            println!("{:?}",bill);
        }

        println!("Enter bill to update:");
        let name =match getInput() {
            Some(name)=>name,
            None=>return,
        };
        let amount=match get_bill_amount() {
            Some(amount)=>amount,
            None=>return
        };
        if bills.update(&name, amount){
            println!("updated");
        }else{
            println!("Bill not found")
        }
    }
}

enum MainMenu{
    AddBill,
    ViewBill,
    RemoveBill,
    UpdateBill
}

impl MainMenu{
    fn from_str(input: &str)->Option<MainMenu>{
        match input{
            "1"=>Some(Self::AddBill),
            "2"=>Some(Self::ViewBill),
            "3"=>Some(Self::RemoveBill),
            "4"=>Some(Self::UpdateBill),
            _=>None,
        }
    }

    fn show(){
        println!("");
        println!("==Bill Manager==");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("3. Remove Bills");
        println!("4. Update Bill");
        println!("");
        println!("Enter Selection: ");
    }
}

fn save_bills(bills: &Bills, filename: &str) -> Result<(), io::Error> {
    let file = File::create(filename)?;
    serde_json::to_writer(file, &bills.inner)?;
    Ok(())
}

fn load_bills(filename: &str) -> Result<Bills, io::Error> {
    let file = File::open(filename)?;
    let inner = serde_json::from_reader(file)?;
    Ok(Bills { inner })
}


fn run_program(bills: &mut Bills) -> Option<()> {
    loop {
        MainMenu::show();
        let input = getInput()?;
        match MainMenu::from_str(&input) {
            Some(MainMenu::AddBill) => menu::add_bill(bills),
            Some(MainMenu::ViewBill) => menu::view_bills(bills),
            Some(MainMenu::RemoveBill) => menu::remove_bill(bills),
            Some(MainMenu::UpdateBill) => menu::update_bill(bills),
            None => break,
        }
    }
    None
}

fn main() {
    let filename = "bills.json";
    let mut bills = match load_bills(filename) {
        Ok(bills) => bills,
        Err(_) => {
            println!("Error loading bills. Starting with an empty list.");
            Bills::new()
        }
    };

    run_program(&mut bills);

    if let Err(e) = save_bills(&bills, filename) {
        eprintln!("Error saving bills: {}", e);
    }
}
