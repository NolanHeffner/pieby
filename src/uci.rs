


/* fn main(engine_name: &str, engine_authors: &str, engine_version: &str) {
    let engine: UCIEngine = UCIEngine::new(engine_name, engine_authors, engine_version);
    engine.UCIEngine::UCILoop();
}
 */

use std::io;

use crate::options::{get_option, set_option, OptionValue};

pub struct UCIEngine {
    engine_name: &'static str,
    engine_authors: &'static str,
    engine_version: &'static str,
    options: Options,
}

impl UCIEngine {

    pub fn new(engine_name: &'static str, engine_authors: &'static str, engine_version: &'static str) -> UCIEngine {
        UCIEngine {
            engine_name,
            engine_authors,
            engine_version,
            options: Options {
                hash: 0,
                threads: 1,
            },
        }
    }
    
    pub fn uci_loop(&mut self) {
        // Instantiate new search object below before loop
        

        // Loop starts
        loop {
            // Get messages input
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let tokens = input.trim().split(' ').collect::<Vec<&str>>();
            
            // Interpret messages
            match tokens[0] {
                "uci" => {
                    println!("id name {} {}", self.engine_name, self.engine_version);
                    println!("id author {}", self.engine_authors);
                    println!("option name Hash type spin min 8 max 65536 default 16");
                    println!("option name Threads type spin min 1 max 255 default 1");
                    println!("uciok");
                }
                    // Tell engine to use the uci (universal chess interface), this will be send once as a first command after program boot to tell the engine to switch to uci mode. After receiving the uci command the engine must identify itself with the "id" command and sent the "option" commands to tell the GUI which engine settings the engine supports if any. After that the engine should sent "uciok" to acknowledge the uci mode. If no uciok is sent within a certain time period, the engine task will be killed by the GUI.
                "setoption" => {

                    if tokens[1] == "name" && tokens[3] == "value" {
                        let name = tokens[2];
                        let is_float_value = tokens[4].contains(".");
                        if is_float_value {
                            let mut fail = false;
                            let value = tokens[4].parse::<f32>().unwrap_or_else(|_e| {
                                println!("Failed to parse option value.");
                                fail = true;
                                0.0
                            });
                            if !fail {set_option(name, OptionValue::from(value));}
                        } else {
                            let mut fail = false;
                            let value = tokens[4].parse::<u64>().unwrap_or_else(|_e| {
                                println!("Failed to parse option value.");
                                fail = true;
                                0
                            });
                            if !fail {set_option(name, OptionValue::from(value));}
                        }
                    }

                /*  let option_name = tokens[3];
                    let num_options_selected = (tokens.len() - 3) / 2;
                    match option_name {
                        "Hash" => {
                            for idx in 0..num_options_selected {
                                let field = tokens[2 * idx + 1];
                                let field_value = tokens[2 * idx + 2];
                                match field {
                                    "type" => {
                                        println!("Only one type of hash is implemented.");
                                    }
                                    "value" => {
                                        self.options.hash = field_value.parse::<u32>().unwrap_or_else(|_e| {
                                            println!("Failed to set new hash, value was unable to be parsed into u32.");
                                            self.options.hash
                                        });
                                    }
                                    _ => ()
                                }
                            }
                        }
                        "Threads" => {
                            for idx in 0..num_options_selected {
                                let field = tokens[2 * idx + 1];
                                let field_value = tokens[2 * idx + 2];
                                match field {
                                    "value" => {
                                        let option_value = tokens[5];
                                        self.options.threads = option_value.parse::<u8>().unwrap_or_else(|_e| {
                                            println!("Failed to set new hash, value was unable to be parsed into u8.");
                                            self.options.threads
                                        });
                                    }
                                    _ => ()
                                }
                            }
                        }
                        _ => {println!("Invalid option name, there is no option of type {option_name}");}
                    }*/
                }
                "quit" => {
                    break;
                }
                "stop" => {
                    // Send stop signal to search function
                }
                "debug" => println!("Debug mode yet to be implemented lmao"), // WIP
                    // Switch the debug mode of the engine on and off. In debug mode the engine should sent additional infos to the GUI, e.g. with the "info string" command, to help debugging, e.g. the commands that the engine has received etc. This mode should be switched off by default and this command can be sent any time, also when the engine is thinking.
                "isready" => println!("readyok"), // NOT OK
                    // This is used to synchronize the engine with the GUI. When the GUI has sent a command or multiple commands that can take some time to complete, this command can be used to wait for the engine to be ready again or to ping the engine to find out if it is still alive. E.g. this should be sent after setting the path to the tablebases as this can take some time. This command is also required once before the engine is asked to do any search to wait for the engine to finish initializing. This command must always be answered with "readyok" and can be sent also when the engine is calculating in which case the engine should also immediately answer with "readyok" without stopping the search.
                "register" => {println!("No registration needed.")},
                "ucinewgame" => {
                    // Reset ttables
                },
                "position" => {
                    match tokens[1] {
                        // "fen" => 
                        _ => println!("Error: Invalid specification for command 'position'."),
                    }
                },
                "go" => {
                    /*
                    match tokens[1] {
                    
                    }*/
                },
                "ponderhit" => {},
                _ => println!("command ignored"),
            }
        }
    }
}