use csv;
use std::{io::{self, Write}, path::Path};
static mut ONCE: i8 = 0;
fn main() {
    let mut path_of_oper=String::new();
    loop {
        print!("Enter Path to store files \t(DON'T USE ~)\n>>");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut path_of_oper).expect("read err");
    path_of_oper=path_of_oper.trim().to_string();
    if std::path::Path::new(&path_of_oper).exists()
    {
        std::env::set_current_dir(path_of_oper).expect("couldnt change path");
        break;}
        else {
            println!("Path Doesnt exists !!!");
            path_of_oper=String::new();
        }
    }
    menu();
}

fn edit() -> bool{
    let mut edited:bool = false;
    println!("Enter the SNO of the Book to edit: ");
    let mut sno_edit = String::new();
    io::stdin().read_line(&mut sno_edit).expect("read err");
    sno_edit=sno_edit.trim().to_string();
    if sno_edit.parse::<i32>().is_ok() && sno_edit.parse::<i32>().expect("it shouldnt raise err")>count()
    {return false;}
    let mut new_name=String::new();
    println!("Enter the new Name: ");
    io::stdin().read_line(&mut new_name).expect("read err");
    new_name=new_name.trim().to_string();
    let mut new_num =String::new();
    println!("Enter the new ISBN Number: ");
    io::stdin().read_line(&mut new_num).expect("read err");
    new_num=new_num.trim().to_string();
    let file_path_source = "./Books.csv";
    let file_path_target = "./Books_temp.csv";
    std::fs::File::create("./Books_temp.csv").expect("couldnt create file");
    reset_or_create("Books_temp");
    {   
        let file_source = std::fs::OpenOptions::new().read(true).open(file_path_source).expect("couldnt open file");
        let file_target = std::fs::OpenOptions::new().append(true).open(file_path_target).expect("couldnt open file");
        let mut reader_source = csv::Reader::from_reader(file_source);
        let mut writer_target = csv::Writer::from_writer(file_target);
        for record in reader_source.records()
        {
            let record=record.expect("couldnt read");
            if sno_edit==record[0]
            {
                edited=true;
            writer_target.write_record(&[record[0].to_string(),new_name.clone(),new_num.clone()]).expect("couldnt write");
            writer_target.flush().expect("couldnt write");
            continue;
            }
            writer_target.write_record(&record).expect("couldnt write");
            writer_target.flush().expect("couldnt write");
        }
    }
    std::fs::remove_file("./Books.csv").expect("couldnt remove temp file");
    std::fs::File::create("./Books.csv").expect("couldnt create file");
    reset_or_create("Books");
    {
        let file_source = std::fs::OpenOptions::new().read(true).open(file_path_target).expect("couldnt open file");
        let file_target = std::fs::OpenOptions::new().append(true).open(file_path_source).expect("couldnt open file");
        let mut reader_source = csv::Reader::from_reader(file_source);
        let mut writer_target = csv::Writer::from_writer(file_target);
        for record in reader_source.records()
        {
            let record=record.expect("couldnt read");
            let sno = count()+1;
            let name=&record[1];
            let isbnnumber=&record[2];
            writer_target.write_record(&[sno.to_string(), name.to_string(), isbnnumber.to_string()]).expect("couldnt write");
            writer_target.flush().expect("couldnt write");
        }
    }
    std::fs::remove_file("./Books_temp.csv").expect("couldnt remove temp file");
    return edited;
}

fn reset_or_create(s:&str) {
    let path=format!("./{s}.csv");
    let file_obj = std::fs::OpenOptions::new().write(true).open(path).expect("couldnt open file");
    let mut csv_writer = csv::Writer::from_writer(file_obj);
    csv_writer.write_record(["SNO".to_string(), "NAME".to_string(), "ISBN NUMBER".to_string()]).expect("couldnt write");
    csv_writer.flush().expect("couldnt flush changes");
}

fn del() -> bool {
    let mut deletion:bool = false;
    println!("Enter the SNO of the Book to delete: ");
    let mut sno_del = String::new();
    io::stdin().read_line(&mut sno_del).expect("read err");
    sno_del=sno_del.trim().to_string();
    if sno_del.parse::<i32>().is_ok() && sno_del.parse::<i32>().expect("it shouldnt raise err")>count()
    {return false;}
    let file_path_source = "./Books.csv";
    let file_path_target = "./Books_temp.csv";
    std::fs::File::create("./Books_temp.csv").expect("couldnt create file");
    reset_or_create("Books_temp");
    {   
        let file_source = std::fs::OpenOptions::new().read(true).open(file_path_source).expect("couldnt open file");
        let file_target = std::fs::OpenOptions::new().append(true).open(file_path_target).expect("couldnt open file");
        let mut reader_source = csv::Reader::from_reader(file_source);
        let mut writer_target = csv::Writer::from_writer(file_target);
        for record in reader_source.records()
        {
            let record=record.expect("couldnt read");
            if sno_del==record[0]{deletion=true;continue;}
            writer_target.write_record(&record).expect("couldnt write");
            writer_target.flush().expect("couldnt write");
        }
    }
    std::fs::remove_file("./Books.csv").expect("couldnt remove temp file");
    std::fs::File::create("./Books.csv").expect("couldnt create file");
    reset_or_create("Books");
    {
        let file_source = std::fs::OpenOptions::new().read(true).open(file_path_target).expect("couldnt open file");
        let file_target = std::fs::OpenOptions::new().append(true).open(file_path_source).expect("couldnt open file");
        let mut reader_source = csv::Reader::from_reader(file_source);
        let mut writer_target = csv::Writer::from_writer(file_target);
        for record in reader_source.records()
        {
            let record=record.expect("couldnt read");
            let sno = count()+1;
            let name=&record[1];
            let number=&record[2];
            writer_target.write_record(&[sno.to_string(), name.to_string(), number.to_string()]).expect("couldnt write");
            writer_target.flush().expect("couldnt write");
        }
    }
    std::fs::remove_file("./Books_temp.csv").expect("couldnt remove temp file");
    return deletion;
}

fn write_file() {
    let file_obj = std::fs::OpenOptions::new().append(true).open("./Books.csv").expect("couldnt open file");
    let mut csv_writer = csv::Writer::from_writer(file_obj);
    println!("Enter the number of Books to append: ");
    let mut num_Books = String::new();
    io::stdin().read_line(&mut num_Books).expect("read err");
    let num_Books: usize = num_Books.trim().parse().expect("Invalid input");
    for _ in 0..num_Books {
        println!("Enter name : ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("read err");
        let name = input.trim().to_string();
        println!("Enter number : ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("read err");
        let number = input.trim().to_string();
        let sno = count()+1;
        csv_writer.write_record(&[sno.to_string(), name, number]).expect("couldnt write");
        csv_writer.flush().expect("couldnt flush changes");
    }
}

fn start() {
    let precheck:bool = Path::new("./Books.csv").exists();
    if !precheck
    {
        std::fs::File::create("./Books.csv").expect("couldnt create file");
        reset_or_create("Books");
    }
	print!("\n\tBooks\n\n");
	print!("1.Read Books\n");
	print!("2.Append Books\n");
	print!("3.Count number of Books saved\n");
	print!("4.Edit Book\n");
	print!("5.Delete a Book\n");
	print!("6.Go back to menu\n");
	print!("7.Exit\n");
	print!(">>");
    io::stdout().flush().unwrap();
    let ch: i32;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("read err");
        match input.trim().parse() {
            Ok(num) => {
                ch = num;
                break;
            }
            Err(_) => println!("Please type a number !!!"),
        }
    }
	match ch {
        1 => {
            print!("\n\tRead Books\n\n");
            println!("\n{}\n", "-".repeat(50));
            read_file();
            println!("\n{}\n", "-".repeat(50));
            print!("\nOperation Completed\n");
            back();
            start();
        }
	 2 =>{
	print!("\n\tAppend Books\n\n");
            println!("\n{}\n", "-".repeat(50));
            write_file();
            println!("\n{}\n", "-".repeat(50));
            print!("\nOperation Comleted\n");
	back();
	start();
	}
	 3=>{
	print!("\n\tCount Books\n\n");
    print!("Number of Books are : {}\n",count());
	print!("\nOperation Comleted\n");
	back();
	start();
	}
    4=>{
        print!("\n\tEdit a Book\n\n");
        if edit(){
        print!("\nOperation Comleted\n");
        }
        else {
        print!("\nOperation Failed , SNO not found\n");
        }
        back();
        start();
        }
	 5=>{
	print!("\n\tDelete a Book\n\n");
    if del(){
	print!("\nOperation Comleted\n");
    }
    else {
	print!("\nOperation Failed , SNO not found\n");
    }
    back();
	start();
	}
	 6=>{
	menu();
	}
	7=>{
	end();}
	_=>{
	print!("Invalid input , program is exiting !!!");
	std::process::exit(0);}
	}
}

fn count() -> i32 {
let mut csv_reader = csv::Reader::from_path("./Books.csv").expect("file couldnt be opened");
let mut count=0;
for _ in csv_reader.records() {
count+=1;
}
return count;
}

fn end() {
		println!("\t\tThanks for Trying this program \n\n\t\t  A Program by TBA5854\n");
		std::process::exit(0);
}

fn back() {
    loop{
    print!("\nType \"back\" to go previous page\n>>");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("read err");
    if input.trim() == "back" {
        return;
    }
    else {
        println!("\n\nPlease type back !!!\n");
    }}
}

fn menu() {
if unsafe { ONCE } == 0 {
    println!("Hello fellow user, you have started the Books Program!\n");
    unsafe{ONCE += 1;}
}
print!("\n\tMenu\n\n");
print!("Use the resources below and type 4 to start\n(Enter number to choose the option)\n\n");
print!("1. Disclaimer\n");
print!("2. Info\n");
print!("3. Contact Developer\n");
print!("4. Start\n");
print!("5. Exit\n");
print!(">>");
io::stdout().flush().unwrap();
let ch: i8;
loop {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("read err");
    match input.trim().parse() {
        Ok(num) => {
            ch = num;
            break;
        }
        Err(_) => println!("Please type a number !!!"),
    }
}
match ch {
        1 => {println!("\n\tDisclaimer\n\n");
        println!("-->This program works the best when\n the user follows instructions\n-->Check Info for instructions");
        back();
        menu();},
        2 => {println!("\n\tInfo\n\n");
        info();
        back();
        menu();},
        3 => {println!("\n\Contact Developer\n\n");
        println!("For any querries or reporting bugs , Book TBA58584 via github or twitter\n");
        println!("www.github.com/TBA5854\n");
        println!("www.twitter.com/TBA5854\n");
        back();
        menu();},
        4 => {start();},
        5 => {end();},
        _ => {println!("Invalid input , program is exiting !!!");
        std::process::exit(0);}
    }
}

fn info() {
	println!("1.This is a library program\n");
	println!("2.This program can store , count , read Books in csv file\n");
	println!("3.Only enter option number\n");
	println!("4.Never enter a alphabet unless asked to do so\n");
}

fn read_file() {
    let mut csv_reader = csv::Reader::from_path("./Books.csv").expect("file couldnt be opened");
    println!("SNO\t\t\tNAME\t\t\tISBN");
    
    for result in csv_reader.records() {
        let record = result.expect("file couldnt be read");
        let sno = &record[0];
        let name = &record[1];
        let number = &record[2];
        println!("{}\t\t\t{}\t\t\t{}", sno, name, number);
    }   
}