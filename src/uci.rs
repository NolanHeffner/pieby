
use std::io;

/* fn main(engine_name: &str, engine_authors: &str, engine_version: &str) {
    let engine: UCIEngine = UCIEngine::new(engine_name, engine_authors, engine_version);
    engine.UCIEngine::UCILoop();
}
 */

pub struct UCIEngine {
    engine_name: &'static str,
    engine_authors: &'static str,
    engine_version: &'static str,
    options: Options,
}

pub struct Options {
    hash: i32,
    threads: i8,
}

impl UCIEngine {
    pub fn new(engine_name: &'static str, engine_authors: &'static str, engine_version: &'static str) -> UCIEngine {
        UCIEngine {
            engine_name,
            engine_authors,
            engine_version,
            options: Options {
                hash: 0,
                threads: 0,
            },
        }
    }
    pub fn uci_loop(&self) {
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
                    println!("uciok");
                }
                    // Tell engine to use the uci (universal chess interface), this will be send once as a first command after program boot to tell the engine to switch to uci mode. After receiving the uci command the engine must identify itself with the "id" command and sent the "option" commands to tell the GUI which engine settings the engine supports if any. After that the engine should sent "uciok" to acknowledge the uci mode. If no uciok is sent within a certain time period, the engine task will be killed by the GUI.
                "quit" | "stop" => {
                    break;
                }
                "debug" => println!("Debug mode yet to be implemented lmao"), // WIP
                    // Switch the debug mode of the engine on and off. In debug mode the engine should sent additional infos to the GUI, e.g. with the "info string" command, to help debugging, e.g. the commands that the engine has received etc. This mode should be switched off by default and this command can be sent any time, also when the engine is thinking.
                "isready" => println!("readyok"), // NOT OK
                    // This is used to synchronize the engine with the GUI. When the GUI has sent a command or multiple commands that can take some time to complete, this command can be used to wait for the engine to be ready again or to ping the engine to find out if it is still alive. E.g. this should be sent after setting the path to the tablebases as this can take some time. This command is also required once before the engine is asked to do any search to wait for the engine to finish initializing. This command must always be answered with "readyok" and can be sent also when the engine is calculating in which case the engine should also immediately answer with "readyok" without stopping the search.
                "setoption" => {
                    /*
                    match tokens[1] {
                        
                    }*/
                },
                "register" => {},
                "ucinewgame" => {},
                "position" => {},
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