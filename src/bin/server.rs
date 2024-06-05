use std::io;

struct Command {
    command: &'static str,
    arguments: Vec<&'static str>,
    description: &'static str,
}

// CLI Menu for controlling background clients that have connected

fn help_menu(commands: &[Command]) {
    // Display menu help message upon running
    println!("{}", "-".repeat(40));
    println!("Rust c2 server test");
    println!("{}", "-".repeat(40));

    println!("\tCommands:");
    for cmd in commands {
        let mut arg_string: String = "".to_string();
        for arg in &cmd.arguments {
            let asd = format!("*{}*", arg);
            arg_string += &asd.as_str();
        }

        println!(
            "\t\t{} {}\n\t\t\t\t{}",
            cmd.command, arg_string, cmd.description
        );
    }
}

fn main() {
    println!("Hello, world!");

    // define array of commands
    let commands = [
        Command {
            command: "help",
            arguments: Vec::new(),
            description: "Displays this help menu",
        },
        Command {
            command: "clients",
            arguments: Vec::new(),
            description: "Retrieve list of connected clients",
        },
        Command {
            command: "select",
            arguments: Vec::from(["client id"]),
            description: "select client by id",
        },
        Command {
            command: "getinfo",
            arguments: Vec::new(),
            description: "Retrieves info from selected client",
        },
    ];

    help_menu(&commands);
    // end help menu

    // accept user input

    let mut input = String::new();
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read stdin");
        println!("{}", input);
    }
}
