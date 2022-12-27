use std::io;

#[derive(Debug)]
struct Item{
    item: String,
    quantity: u32,
}

fn main(){
    println!("Simple Inventory System");
    let mut items: Vec<Item> = vec![];
    loop{
        let mut choice: String = String::from("0");
        println!("Please enter choice");
        println!("1. View Items");
        println!("2. Add Items");
        println!("3. Modify Items");
        println!("4. Delete Items");
        println!("5. Exit");
        
        io::stdin().read_line(&mut choice).expect("Please enter valid integers!");

        let choice: u32 = choice.trim().parse().expect("Parse failed!");

        if choice == 1{
            view_item(&items);
        }
        if choice == 2{
            add_item(&mut items);
        }
        if choice == 3{
            modify_item(&mut items);
        }
        if choice == 4{
            delete_item(&mut items);
        }
        if choice == 5{
            println!("Thank you");
            break;
        }

    }
}

fn view_item(items: &Vec<Item>){
    clear();
    println!("vec size: {}", items.len());
    println!("{}", "-".repeat(52));
    println!("| {: <3} | {: <20} | {: <20}|","No.", "Item Name", "Item Quantity");
    println!("{}", "-".repeat(52));
    for (idx, item) in items.iter().enumerate(){
        println!("| {: <3} | {: <20} | {: <20}|",idx+1,  item.item, item.quantity);
    }
    if items.len() > 0{
        println!("{}", "-".repeat(52));
    }
}
fn add_item(items: &mut Vec<Item>){
    clear();
    println!("Add Item");
    let mut item_name: String = String::from("");
    let mut item_qty: String = String::from("");
    println!("Please enter item name:");
    io::stdin().read_line(&mut item_name).expect("please enter valid string");
    println!("Please enter item quantity:");
    io::stdin().read_line(&mut item_qty).expect("please enter valid string");
    let item_qty = item_qty.trim().parse().expect("parse failed");
    items.push(Item { item: item_name.trim().to_string(), quantity: item_qty });
}
fn modify_item(items: &mut Vec<Item>){
    view_item(items);
    println!("Please enter the item you wish to modify:");
    let mut item_idx = String::from("");
    io::stdin().read_line(&mut item_idx).expect("failed to read");
    let item_idx = item_idx.trim().parse::<usize>().unwrap();
    if item_idx-1 >= items.len() {
        println!("Invalid item index!");
        return;
    }else{
        let mut new_name = String::from("");
        let mut new_qty = String::from("0");
        println!("Please enter new name:");
        io::stdin().read_line(&mut new_name).expect("read failed");
        println!("Please enter new quantity:");
        io::stdin().read_line(&mut new_qty).expect("read failed");
        let new_name: String = new_name.trim().to_string();
        let new_qty: u32 = new_qty.trim().parse().expect("parse failed");
        items[item_idx-1].item = new_name;
        items[item_idx-1].quantity = new_qty;
        println!("Item modified successfully!");
    }



}
fn delete_item(items: &mut Vec<Item>){
    view_item(items);
    println!("Please enter the item number you wish to delete:");
    let mut item_idx = String::from("");
    io::stdin().read_line(&mut item_idx).expect("read failed");
    let item_idx: usize = item_idx.trim().parse().expect("parse failed");
    if item_idx - 1 > items.len(){
        return;
    }else{
        items.remove(item_idx - 1);
        println!("Item removed successfully!");
    }
}

fn clear(){
    print!("{}[2J", 27 as char);
}

