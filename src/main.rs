#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::path::Path;
use std::process::Command;


// use std::any::type_name;

// fn print_type_of<T>(_: &T) {
//     println!("{}", type_name::<T>());
// }

fn main() {
    
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();

        io::stdin().read_line(&mut command).unwrap();

        let args: Vec<String> = command.split_whitespace().map(String::from).collect();

        let first_arg = args.first().map(String::as_str);

        let key = "PATH";

        match first_arg {
            Some("exit") => break,
            Some("type") => {
                if let Some(arg) = args.get(1) {
                    if find_type(arg) == "builtin" {
                        println!("{arg} is a shell builtin")
                    } else {
                        match env::var_os(key){
                            Some(paths) => {
                                let mut found: bool = false;
                                for path in env::split_paths(&paths){
                                    let full_path = path.join(arg);
                                    if is_file_executable(&full_path) {
                                        found = true;
                                        println!("{arg} is {}", full_path.display());
                                        break;
                                    }

                                }
                                if !found{
                                    println!("{arg}: not found");
                                }
                                
                            }
                            None => println!("{arg}: not found"),
                        }
                    }
                }
            },
            Some("echo") => println!("{}", args[1..].join(" ")),
            Some("pwd") => {
                let cwd = env::current_dir().unwrap();
                println!("{}", cwd.display());
            }
            Some ("cd") => {
                if let Some(dir) = args.get(1){
                    if dir == "~"{
                        if let Some(home) = env::var_os("HOME"){
                            if let Err(_) = env::set_current_dir(&home){
                                println!("cd: {}: No such file or directory", dir);
                            }
                        } else {
                            println!("cd: HOME not set");
                        }
                        
                    } else {
                        let path = Path::new(dir);

                        if let Err(_) = env::set_current_dir(path){
                            println!("cd: {}: No such file or directory", dir);
                        }
                    }
                    
                } else {
                    println!("cd: missing argument");
                }
            }
            Some(v) => {
                if find_type(v) != "builtin" {
                    match env::var_os(key){
                        Some(paths) => {
                            let mut status: bool = false;
                            for path in env::split_paths(&paths){
                                let full_path = path.join(v);
                                if is_file_executable(&full_path){
                                    Command::new(v).args(&args[1..]).status().unwrap();
                                    status = true;
                                    break;
                                }
                            }
                            if !status {
                                println!("{v}: command not found")
                            }
                        },
                        None => println!("{v}: not found"),
                    }

                }
            },
            None => break,
        }
        
        io::stdout().flush().unwrap();
    
        
        
    }

    fn find_type(arg: &str) -> &str {
        let builtin_array = ["echo", "exit", "type", "pwd", "cd"];
        if builtin_array.contains(&arg) {
            return "builtin"
        } else {
            return "eh"
        }
    }

    fn is_file_executable(path: &Path) -> bool {
        if path.exists(){
            if let Ok(metadata) = std::fs::metadata(path){
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    return metadata.permissions().mode() & 0o111 != 0; 
                      
                }
            }
        }
        false
    }
}
