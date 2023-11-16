use anyhow::Result;
use std::fs;
use rand::Rng;

trait Command{
    fn get_name(&self)-> &str;
    fn exec(&mut self, slice: &[String]) -> Result<()>;
}

struct PingCommand{

}

impl Command for PingCommand{
    fn get_name(&self)-> &str{
        return "ping";
    }
    fn exec(&mut self, slice: &[String]) -> Result<()>{
        if slice.is_empty(){
            println!("pong!");
        }
        else{
            println!("Ping se foloseste fara argumente!");
        }
        Ok(())
    }
}

struct CountCommand{

}

impl Command for CountCommand{
    fn get_name(&self)-> &str {
        "count"
    }

    fn exec(&mut self, slice: &[String]) -> Result<()> {
        println!("counted {} args",slice.len());
        Ok(())
    }
}

struct TimesCommand{
    count: u32,
}

impl Command for TimesCommand{
    fn get_name(&self)-> &str {
        "times"
    }

    fn exec(&mut self, _slice: &[String]) -> Result<()> {
        self.count += 1;
        println!("the {} th time",self.count);
        Ok(())
    }
}

struct InspirationalQuotes{

}

impl Command for InspirationalQuotes{
    fn get_name(&self)-> &str {
        "quote"
    }

    fn exec(&mut self, _slice: &[String]) -> Result<()> {
        let quotes = vec![
        "The only way to do great work is to love what you do. - Steve Jobs",
        "Believe you can and you're halfway there. - Theodore Roosevelt",
        "Don't watch the clock; do what it does. Keep going. - Sam Levenson",
        "The future belongs to those who believe in the beauty of their dreams. - Eleanor Roosevelt",
        "The secret of getting ahead is getting started. - Mark Twain",
        "It does not matter how slowly you go as long as you do not stop. - Confucius",
        "Success is not the key to happiness. Happiness is the key to success. - Albert Schweitzer",
        "If you can dream it, you can do it. - Walt Disney",
        "The harder you work for something, the greater you'll feel when you achieve it. - Unknown",
        "Dream bigger. Do bigger. - Unknown",
        "Be yourself; everyone else is already taken. - Oscar Wilde"
    ];
        let nr = rand::thread_rng().gen_range(0..quotes.len());
        println!("{}", quotes[nr]);
        Ok(())
    }
}

struct Terminal{
    a: Vec<Box<dyn Command>>,
}

impl Terminal {
    fn new() -> Terminal{
        Terminal {
            a : Vec::new(),
        } 
    }
    fn register(&mut self, b: Box<dyn Command>) -> (){
        self.a.push(b);
    }
    fn run(&mut self) -> Result<()>{
        let commands = fs::read_to_string("text.txt")?;
        for i in commands.lines(){
            let mut command_name = String::new();
            let mut args: Vec<String> = Vec::new();
            for j in i.split_whitespace(){
                if command_name.len() == 0{
                    command_name = j.to_string();
                }
                else {
                    args.push(j.to_string());
                }
            }
            for k in &mut self.a{
                if k.get_name() ==  command_name{
                    if let Err(error) = k.exec(&args) {
                        println!("Error! {}",error);
                        break;
                    }
                } else if k.get_name() == command_name.to_lowercase(){
                    println!("Did you mean: {}?",k.get_name());
                }
            }
            if command_name == "stop"{
                return Ok(());
            }
        }
        Ok(())
    }
}


fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(InspirationalQuotes{}));

    let result = terminal.run();
    match result{
        Ok(_k) => println!("All the correctly-written commands were executed successfully"),
        Err(e) => print!("There was an error: {}",e),
    }
}
