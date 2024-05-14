use std::io;
struct TodoItem{
    id: u32,
    name: String,
    completed: bool
}


fn complete_item(item: &mut TodoItem){
    item.completed = true;
}

fn display_items(items: &Vec<TodoItem>){
    for item in items{
        println!("{} - {} ({})", item.id, item.name, item.completed);
    }
}

fn main() {
    let mut todo_list: Vec<TodoItem> = Vec::new();

    loop{
        println!("Что бы ты хотел сделать?");
        println!("1. Добавить задание");
        println!("2. Завершить задание");
        println!("3. Показать список дел");
        println!("4. Закрыть");


        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Error to read line!");

        let choice = choice.trim().parse::<u32>().expect("Invalid input string!");

        match choice {
            1=>{
                println!("Введите список дел:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim().to_string();

                let id = todo_list.len() as u32 +1;

                let item = TodoItem{
                    id,
                    name,
                    completed: false,
                };

                todo_list.push(item)
            }

            2=>{
                println!("Введите ID дела,которое вы хотите завершить.");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failde to read line");
                let id = id.trim().parse::<u32>().expect("Invalid input");

                let item =
                    todo_list.iter_mut().find(|i| i.id == id).unwrap();
                complete_item(item);
            }
            3=>{
                display_items(&todo_list);
            }
            4=>{
                println!("Пока-пока!");
                return;
            }

            _ => {
                println!("Invalid choice");
            }
        };
    }

}
