use std::fs::{ self, File };
use std::io::Write;
use std::path::Path;
use clap::{ Arg, ColorChoice };
const LOGO: &str =
    r#"
______     __                                       __
/      \   |  \                                     |  \
|  $$$$$$\ _| $$_     ______   ______   __   __   __ | $$____    ______    ______    ______   __    __
| $$___\$$|   $$ \   /      \ |      \ |  \ |  \ |  \| $$    \  /      \  /      \  /      \ |  \  |  \
\$$    \  \$$$$$$  |  $$$$$$\ \$$$$$$\| $$ | $$ | $$| $$$$$$$\|  $$$$$$\|  $$$$$$\|  $$$$$$\| $$  | $$
_\$$$$$$\  | $$ __ | $$   \$$/      $$| $$ | $$ | $$| $$  | $$| $$    $$| $$   \$$| $$   \$$| $$  | $$
|  \__| $$  | $$|  \| $$     |  $$$$$$$| $$_/ $$_/ $$| $$__/ $$| $$$$$$$$| $$      | $$      | $$__/ $$
\$$    $$   \$$  $$| $$      \$$    $$ \$$   $$   $$| $$    $$ \$$     \| $$      | $$       \$$    $$
 \$$$$$$     \$$$$  \$$       \$$$$$$$  \$$$$$\$$$$  \$$$$$$$   \$$$$$$$ \$$       \$$       _\$$$$$$$
                                                                                            |  \__| $$
                                                                                             \$$    $$
                                                                                              \$$$$$$
"#;
fn main() {
    handle_args();
}
fn handle_args() {
    println!("\n{}", LOGO);
    let mut cmd = clap::Command
        ::new("IDK")
        .color(ColorChoice::Auto)
        .version("0.1-beta")
        .about("A toy object-oriented programming language")
        .subcommand(clap::Command::new("build").about("Build the current project directory"))
        .subcommand(
            clap::Command
                ::new("new")
                .about("Create a new empty project folder")
                .arg(Arg::new("name").required(true).help("The name of the new project"))
        );
    let matches = cmd.clone().get_matches();
    if let Some(matches) = matches.subcommand_matches("new") {
        let msg = format!(
            "🎉 Congratulations, you successfully created the project, please use cd ./{}, and then use idk build to build the project!",
            matches.get_one::<String>("name").unwrap()
        );
        // println!("STuff sefgywef fiewbf ,{}", msg);
        create_project_folder(matches.get_one::<String>("name").unwrap());
    }
    //  else if let Some(_) = matches.subcommand_matches("build") {
    //     let mut curr_path = "./src".to_string();
    //     let mut path_flag = true;
    //     let home_dir = home_dir().unwrap().into_os_string().into_string().unwrap();
    //     let mut files: Vec<String> = vec![
    //         format!("{}/.strawberry/std/Object.st", home_dir),
    //         format!("{}/.strawberry/std/Integer.st", home_dir),
    //         format!("{}/.strawberry/std/String.st", home_dir),
    //         format!("{}/.strawberry/std/Bool.st", home_dir),
    //         format!("{}/.strawberry/std/Void.st", home_dir)
    //     ];
    //     while path_flag {
    //         path_flag = false;
    //         if let Ok(paths) = fs::read_dir(&curr_path) {
    //             // paths.
    //             for path in paths {
    //                 if let Ok(d) = &path {
    //                     let path_name = d.path().to_str().unwrap().to_string();
    //                     if metadata(&path_name).unwrap().is_dir() {
    //                         path_flag = true;
    //                         curr_path = path_name.to_string();
    //                     } else {
    //                         files.insert(0, path_name);
    //                     }
    //                 }
    //             }
    //         } else {
    //             let err = format!(
    //                 "❌ Failed to build because the current directory is not a strawberry project, try strawberry new example"
    //             );
    //             println!("{}", err.red());
    //         }
    //     }
    //     compile(files);
    // }
    else {
        let _ = cmd.print_long_help();
    }
}
fn create_project_folder(name: &str) {
    let path = Path::new(name);
    if path.exists() {
        println!("Error: {} already exists", name);
        return;
    }
    fs::create_dir(path).expect("Failed to create project folder");
    fs::create_dir(path.join("src")).expect("Failed to create project src folder");
    fs::create_dir(path.join("build")).expect("Failed to create project build folder");
    let mut file = File::create(path.join("src/main.idk")).expect("Failed to create main.st");
    file.write(
        b"\tFn main() -> Int { \n\t\tprint(\"Hello world!\"); \n\t\treturn 0; \n\t};"
    ).unwrap();
}
