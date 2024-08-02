// Copyright Niculici Mihai-Daniel 2023
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

fn pwd() {
    // Attempt to get the current directory
    if let Ok(path) = env::current_dir() {
        // If successful, print the current directory path
        println!("{}", path.display());
    } else {
        // If an error occurs, print an error message and exit with status code 206
        println!("{}", 206);
        std::process::exit(206);
    }
}


fn echo(args: Vec<String>) {
    // Check if there are not enough arguments
    if args.len() < 3 {
        println!("{}", 246);
        std::process::exit(246);
    } else {
        // Check if the second argument is not "-n" 
        if args[2].as_str() != "-n" {
            let mut i = 2;
            // Iterate through the arguments and print them with spaces
            while i < args.len() {
                if i == args.len() - 1 {
                    print!("{}", args[i]);
                } else {
                    print!("{} ", args[i]);
                }
                i += 1;
            }
            println!(); // Print a newline after printing the arguments
        } else {
            let mut i = 3;
            // Iterate through the arguments (skipping "-n") and print them with spaces
            while i < args.len() {
                if i == args.len() - 1 {
                    print!("{}", args[i]);
                } else {
                    print!("{} ", args[i]);
                }
                i += 1;
            }
        }
    }
}


fn cat(args: Vec<String>) {
    // Check if there are not enough arguments
    if args.len() < 3 {
        println!("{}", 236);
        std::process::exit(236);
    } else {
        let mut i = 2;
        // Iterate through the arguments starting from index 2
        while i < args.len() {
            // Attempt to read the file contents
            if let Ok(contents) = fs::read_to_string(&args[i]) {
                // Print the contents of the file
                print!("{}", contents);
            } else {
                // If there's an error reading the file, print an error message and exit
                println!("{}", 236);
                std::process::exit(236);
            }
            i += 1;
        }
    }
}


fn mkdir(args: Vec<String>) {
    // Check if there are not enough arguments
    if args.len() < 3 {
        println!("{}", 226);
        std::process::exit(226);
    } else {
        let mut i = 2;
        // Iterate through the arguments starting from index 2
        while i < args.len() {
            // Attempt to create a directory with the provided path
            if let Err(e) = fs::create_dir(&args[i]) {
                // Print any error that occurs and exit with an error status code
                println!("{}", e);
                std::process::exit(226);
            }
            i += 1;
        }
    }
}

fn mv(args: Vec<String>) {
    // Check if there are not enough arguments
    if args.len() < 4 {
        println!("{}", 216);
        std::process::exit(216);
    } else {
        // Attempt to rename the source file or directory to the target path
        if let Err(e) = fs::rename(&args[2], &args[3]) {
            // Print any error that occurs and exit with an error status code
            println!("{}", e);
            std::process::exit(216);
        }
    }
}


fn ln(args: Vec<String>) {
    // Check if there are not enough arguments
    if args.len() < 4 {
        println!("{}", 206);
        std::process::exit(206);
    }
    // Check if it's a symbolic link creation
    if args[2] != "-s" && args[2] != "--symbolic"  {
        // Check for invalid options or flags
        if args[2].chars().next() == Some('-') {
            println!("Invalid command");
            std::process::exit(255);
        }
        // Attempt to create a hard link
        if let Err(e) = fs::hard_link(&args[2], &args[3]) {
            // Print any error that occurs and exit with an error status code
            println!("{}", e);
            std::process::exit(206);
        }
    } else {
        // Attempt to create a symbolic link
        if let Err(e) = std::os::unix::fs::symlink(&args[3], &args[4]) {
            // Print any error that occurs and exit with an error status code
            println!("{}", e);
            std::process::exit(206);
        }
    }
}


fn rmdir(args: Vec<String>) {
    // Check if there are not enough arguments
    if args.len() < 3 {
        println!("{}", 196);
        std::process::exit(196);
    } else {
        let mut i = 2;
        // Iterate through the arguments starting from index 2
        while i < args.len() {
            // Attempt to remove a directory with the provided path
            if let Err(e) = fs::remove_dir(&args[i]) {
                // Print any error that occurs and exit with an error status code
                println!("{}", e);
                std::process::exit(196);
            }
            i += 1;
        }
    }
}


fn rm(args: Vec<String>) {
    // Check if there are not enough arguments
    if args.len() < 3 {
        println!("{}", 186);
        std::process::exit(186);
    } else {
        // Check if it's a directory removal
        if args[2] == "-d" || args[2] == "--dir" {
            let mut i = 3;
            // Iterate through the arguments starting from index 3
            while i < args.len() {
                // Attempt to remove a directory with the provided path
                if let Err(e) = fs::remove_dir(&args[i]) {
                    // Print any error that occurs and exit with an error status code
                    println!("{}", e);
                    std::process::exit(186);
                }
                i += 1;
            }
        } else if (args[2] == "-r" || args[2] == "-R" || args[2] == "--recursive") && args.len() < 4 {
            println!("Invalid command");
            std::process::exit(255);
        } else if args[2] == "-r" || args[2] == "-R" || args[2] == "--recursive" {
            // Handle recursive removal of files and directories
            if args[3] == "-d" || args[3] == "--dir" {
                let mut _i = 4;
                // Iterate through the arguments starting from index 4
                while _i < args.len() {
                    // Check if the file is a directory and remove it recursively
                    if let Ok(file_type) = fs::symlink_metadata(&args[_i]) {
                        if file_type.is_dir() {
                            rm_recursive(&args[_i]);
                        } else {
                            // If it's a file, remove it
                            if let Err(e) = fs::remove_file(&args[_i]) {
                                // Print any error that occurs and exit with an error status code
                                println!("{}", e);
                                std::process::exit(186);
                            }
                        }
                    }
                    _i += 1;
                }
            } else {
                let mut _i = 3;
                // Iterate through the arguments starting from index 3
                while _i < args.len() {
                    // Check if the file is a directory and remove it recursively
                    if let Ok(file_type) = fs::symlink_metadata(&args[_i]) {
                        if file_type.is_dir() {
                            rm_recursive(&args[_i]);
                        } else {
                            // If it's a file, remove it
                            if let Err(e) = fs::remove_file(&args[_i]) {
                                // Print any error that occurs and exit with an error status code
                                println!("{}", e);
                                std::process::exit(186);
                            }
                        }
                    }
                    _i += 1;
                }
            }
        } else {
            let mut i = 2;
            let mut _ok = false;
            // Iterate through the arguments starting from index 2
            while i < args.len() {
                // Check if the file is a directory or not
                if let Ok(file_type) = fs::symlink_metadata(&args[i]) {
                    if file_type.is_dir() {
                        _ok = true;
                        i += 1;
                        continue;
                    } else {
                        // If it's a file, remove it
                        if let Err(e) = fs::remove_file(&args[i]) {
                            // Print any error that occurs and exit with an error status code
                            println!("{}", e);
                            std::process::exit(186);
                        }
                    }
                } else {
                    // If there's an error reading the file, print an error message and exit
                    println!("{}", 186);
                    std::process::exit(186);
                }
                i += 1;
            }
            // Check for invalid mix of files and directories
            if _ok == true {
                println!("{}", 186);
                std::process::exit(186);
            }
        }
    }
}


fn rm_recursive(path: &String) {
    // Attempt to read the contents of the directory
    if let Ok(entries) = fs::read_dir(path) {
        // Iterate through the entries in the directory
        for entry in entries {
            if let Ok(entry) = entry {
                // Get the file type of the entry
                if let Ok(file_type) = entry.file_type() {
                    // Check if the entry is a directory
                    if file_type.is_dir() {
                        // Recursively call rm_recursive on the subdirectory
                        rm_recursive(&entry.path().display().to_string());
                    } else {
                        // If it's a file, remove it
                        if let Err(e) = fs::remove_file(&entry.path()) {
                            // Print any error that occurs and exit with an error status code
                            println!("{}", e);
                            std::process::exit(186);
                        }
                    }
                }
            }
        }
    } else {
        // If there's an error reading the directory, print an error message and exit
        println!("{}", 186);
        std::process::exit(186);
    }
    // Attempt to remove the directory itself
    if let Err(e) = fs::remove_dir(path) {
        // Print any error that occurs and exit with an error status code
        println!("{}", e);
        std::process::exit(186);
    }
}


fn ls(args: Vec<String>) {
    // Check if there are not enough arguments and the command is "ls"
    if args.len() < 3 && args[1] == "ls" {
        let path = Path::new(".");
        if path.is_dir() {
            // Iterate through the entries in the current directory and skip hidden files
            for entry in fs::read_dir(path).unwrap() {
                let entry = entry.unwrap();
                let file_name = entry.file_name();
                if file_name.to_string_lossy().chars().next() == Some('.') {
                    continue;
                }
                // Print the file or directory name
                println!("{}", file_name.to_string_lossy());
            }
        }
    } else if args.len() < 3 {
        // If there are not enough arguments, print an error message and exit
        println!("{}", 176);
        std::process::exit(176);
    } else if args[2] == "-r" || args[2] == "-R" || args[2] == "recursive" {
        // Check if it's a recursive listing with or without hidden files
        if args[3] == "-a" {
            let i = 4;
            // Check if the specified path is a directory and list its contents recursively (including hidden files)
            if let Ok(file_type) = fs::symlink_metadata(&args[i]) {
                if file_type.is_dir() {
                    ls_recursive_all(&args[i]);
                }
            }
        } else {
            let i = 3;
            // Check if the specified path is a directory and list its contents recursively (excluding hidden files)
            if let Ok(file_type) = fs::symlink_metadata(&args[i]) {
                if file_type.is_dir() {
                    ls_recursive(&args[i]);
                } else {
                    println!("{}", 176);
                    std::process::exit(176);
                }
            } else {
                println!("{}", 176);
                std::process::exit(176);
            }
        }
    } else if args[2] == "-a" {
        let i = 3;
        // Check if the specified path is a directory and list its contents (including hidden files)
        if let Ok(file_type) = fs::symlink_metadata(&args[i]) {
            if file_type.is_dir() {
                let path = Path::new(&args[i]);
                if path.is_dir() {
                    for entry in fs::read_dir(path).unwrap() {
                        let entry = entry.unwrap();
                        let file_name = entry.file_name();
                        // Print the file or directory name
                        println!("{}", file_name.to_string_lossy());
                    }
                    // Print "." and ".." for directory navigation
                    println!(".");
                    println!("..");
                }
            } else {
                // If it's not a directory, print the specified path
                println!("{}", &args[i]);
            }
        } else {
            println!("{}", 176);
            std::process::exit(176);
        }
    } else {
        let i = 2;
        // Check if the specified path is a directory and list its contents (excluding hidden files)
        if let Ok(file_type) = fs::symlink_metadata(&args[i]) {
            if file_type.is_dir() {
                let path = Path::new(&args[i]);
                if path.is_dir() {
                    for entry in fs::read_dir(path).unwrap() {
                        let entry = entry.unwrap();
                        let file_name = entry.file_name();
                        if file_name.to_string_lossy().chars().next() == Some('.') {
                            continue;
                        }
                        // Print the file or directory name
                        println!("{}", file_name.to_string_lossy());
                    }
                }
            } else {
                // If it's not a directory, print the specified path
                println!("{}", &args[i]);
            }
        } else {
            println!("{}", 176);
            std::process::exit(176);
        }
    }
}

fn ls_recursive(path: &String) {
    // Attempt to read the entries in the specified directory
    if let Ok(entries) = fs::read_dir(path) {
        println!("{}:", path);
        // Iterate through the entries in the directory and skip hidden files
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                if file_name.to_string_lossy().chars().next() == Some('.') {
                    continue;
                }
                // Print the file or directory name
                println!("{}", file_name.to_string_lossy());
            }
        }

        // Read the entries again to handle directories recursively
        if let Ok(entr) = fs::read_dir(path) {
            // Iterate through the entries in the directory
            for entry in entr {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    if file_name.to_string_lossy().chars().next() == Some('.') {
                        continue;
                    }
                    // Get the file type of the entry
                    if let Ok(file_type) = entry.file_type() {
                        // Check if the entry is a directory
                        if file_type.is_dir() {
                            // Recursively call ls_recursive on the subdirectory
                            ls_recursive(&entry.path().display().to_string());
                        }
                    }
                }
            }
        }
    } else {
        // If there's an error reading the directory, print an error message and exit
        println!("{}", 176);
        std::process::exit(176);
    }
}


fn ls_recursive_all(path: &String) {
    // Attempt to read the entries in the specified directory
    if let Ok(entries) = fs::read_dir(path) {
        println!("{}:", path);
        // Print "." and ".." for directory navigation
        println!(".");
        println!("..");
        // Iterate through the entries in the directory
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                // Print the file or directory name
                println!("{}", file_name.to_string_lossy());
            }
        }

        // Read the entries again to handle directories recursively
        if let Ok(entr) = fs::read_dir(path) {
            // Iterate through the entries in the directory
            for entry in entr {
                if let Ok(entry) = entry {
                    // Get the file type of the entry
                    if let Ok(file_type) = entry.file_type() {
                        // Check if the entry is a directory
                        if file_type.is_dir() {
                            // Recursively call ls_recursive_all on the subdirectory
                            ls_recursive_all(&entry.path().display().to_string());
                        }
                    }
                }
            }
        }
    } else {
        // If there's an error reading the directory, print an error message and exit
        println!("{}", 176);
        std::process::exit(176);
    }
}


fn cp(args: Vec<String>) {
    // Check if there are enough arguments
    if args.len() < 4 {
        println!("{}", 166);
        std::process::exit(166);
    } else {
        // Check if the copy operation should be recursive
        if args[2] == "-r" || args[2] == "-R" || args[2] == "--recursive" {
            let i = 3;
            // Check the type of the source file or directory
            if let Ok(file_type) = fs::symlink_metadata(&args[i]) {
                if file_type.is_dir() {
                    // If it's a directory, create the destination directory if it doesn't exist
                    if let Err(_e) = std::fs::create_dir(&args[i + 1]) {
                        // Extract the directory name from the source path
                        let dir_name = args[i].split("/");
                        let mut name_of_dir = String::from("");
                        for j in dir_name {
                            name_of_dir = String::from(j);
                        }
                        // Form the new directory path
                        let file = format!("{}/{}", args[i + 1], name_of_dir);
                        if let Err(e) = std::fs::create_dir(&file) {
                            println!("{}", e);
                            std::process::exit(166);
                        }
                        // Recursively copy the contents to the destination
                        cp_recursive(&args[i], &file);
                    } else {
                        // Copy the contents of the directory recursively to the destination directory
                        cp_recursive(&args[i], &args[i + 1]);
                    }
                } else {
                    // If it's a file, create the destination file and copy the content
                    let file_name = args[i].split("/");
                    let mut name_of_file = String::from("");
                    for j in file_name {
                        name_of_file = String::from(j);
                    }
                    // Form the new file path
                    let file = format!("{}/{}", args[i + 1], name_of_file);
                    // Create the new file
                    if let Err(e) = std::fs::File::create(&file) {
                        println!("{}", e);
                        std::process::exit(166);
                    }
                    // Copy the file to the destination
                    if let Err(e) = fs::copy(&args[i], &file) {
                        println!("{}", e);
                        std::process::exit(166);
                    }
                }
            }

        } else {
            // Not in recursive mode, copy single files or directories
            let mut _i = 2;
            // Check the type of the source file or directory
            if let Ok(file_type) = fs::symlink_metadata(&args[_i]) {
                if file_type.is_dir() {
                    // Directories are not allowed in non-recursive mode
                    println!("{}", 166);
                    std::process::exit(166);
                } else {
                    // If it's a file, create the destination file and copy the content
                    // Check if the destination file already exists
                    if let Err(_e) = std::fs::File::create(&args[_i + 1]) {
                        // Extract the file name from the source path
                        let file_name = args[_i].split("/");
                        let mut name_of_file = String::from("");
                        for j in file_name {
                            name_of_file = String::from(j);
                        }
                        // Form the new file path
                        let file = format!("{}/{}", args[_i + 1], name_of_file);
                        // Create the new file
                        if let Err(e) = std::fs::File::create(&file) {
                            println!("{}", e);
                            std::process::exit(166);
                        }
                        // Copy the file to the destination
                        if let Err(e) = fs::copy(&args[_i], &file) {
                            println!("{}", e);
                            std::process::exit(166);
                        }
                    } else {
                        // If the destination file doesn't exist, create it and copy the content
                        if let Err(e) = std::fs::File::create(&args[_i + 1]) {
                            println!("{}", e);
                            std::process::exit(166);
                        }
                        // Copy the file to the destination
                        if let Err(e) = fs::copy(&args[_i], &args[_i + 1]) {
                            println!("{}", e);
                            std::process::exit(166);
                        }
                    }
                }
            } else {
                // If the source doesn't exist or has an invalid type, exit with an error
                println!("{}", 166);
                std::process::exit(166);
            }
        }
    }
}


fn cp_recursive(path: &String, path_2: &String) {
    // Iterate through entries in the source directory
    for entry in fs::read_dir(&path).unwrap() {
        let entry = entry.unwrap();
        let drum = entry.path();
        
        // Check if the entry is a directory
        if drum.is_dir() {
            // Extract the directory name from the source path
            let dir_name: String = drum.display().to_string();
            let mut name_of_dir = String::from("");
            
            if let Some(last_part) = dir_name.split("/").last() {
                name_of_dir = String::from(last_part);
            }
            
            // Form the new directory path in the destination
            let file = format!("{}/{}", path_2, name_of_dir);
            
            // Create the destination directory
            if let Err(e) = std::fs::create_dir(&file) {
                println!("{}", e);
                std::process::exit(166);
            }
            
            // Recursively call cp_recursive for the subdirectory
            cp_recursive(&drum.display().to_string(), &file);
        } else {
            // If it's a file, extract the file name
            let file_name = drum.display().to_string();
            let mut name_of_file = String::from("");
            
            if let Some(last_part) = file_name.split("/").last() {
                name_of_file = String::from(last_part);
            }
            
            // Form the new file path in the destination
            let mut _file = format!("{}/{}", path_2, name_of_file);
            
            // Create the destination file
            if let Err(e) = std::fs::File::create(&_file) {
                println!("{}", e);
                std::process::exit(166);
            }
            
            // Copy the file from the source to the destination
            if let Err(e) = fs::copy(&drum, &_file) {
                println!("{}", e);
                std::process::exit(166);
            }
        }
    }
}

fn touch(args: Vec<String>) {
    if args.len() < 3 {
        // If there are not enough arguments, print an error and exit
        println!("{}", 156);
        std::process::exit(156);
    } else {
        if args[2] == "-a" {
            // If the option is "-a", update the access and modification times of the file.
            let i = 3;
            // Remove the existing file, if any
            let _ = std::fs::remove_file(&args[i]);
            // Create the file, effectively updating its timestamp
            if let Err(e) = std::fs::File::create(&args[i]) {
                println!("{}", e);
                std::process::exit(156);
            }
        } else if args[2] == "-m" {
            // If the option is "-m", update only the modification time of the file.
            let i = 3;
            // Remove the existing file, if any
            let _ = std::fs::remove_file(&args[i]);
            // Create the file, effectively updating its modification timestamp
            if let Err(e) = std::fs::File::create(&args[i]) {
                println!("{}", e);
                std::process::exit(156);
            }
        } else if args[2] == "-c" || args[2] == "--no-creat" {
            // If the option is "-c" or "--no-creat", do nothing (no creation).
            return;
        } else {
            // If no special options are provided, create or update the specified files.
            let mut i = 2;
            while i < args.len() {
                // Remove the existing file, if any
                let _ = std::fs::remove_file(&args[i]);
                // Create the file, effectively updating its timestamp
                if let Err(e) = std::fs::File::create(&args[i]) {
                    println!("{}", e);
                    std::process::exit(156);
                }
                i += 1;
            }
        }
    }
}

fn chmod(args: Vec<String>) {
    // Check if the correct number of arguments is provided
    if args.len() < 4 {
        println!("{}", 146);  // Print an error code
        std::process::exit(146);  // Exit the program with an error code
    } else if args[2].chars().next() == Some('-') {
        println!("Invalid command");  // Print an error message
        std::process::exit(255);  // Exit the program with an error code
    } else if let Ok(mut _mode) = args[2].parse::<u32>() {
        // Parse the mode as an octal value
        let file_path = &args[3];
        let temp = fs::metadata(file_path).unwrap();
        let mut _permissions = temp.permissions();
        let octal_string = format!("{}", _mode);
        let octal_value = u32::from_str_radix(&octal_string, 8).unwrap();

        // Set the mode of the permissions
        _permissions.set_mode(octal_value);

        // Apply the modified permissions to the file
        if let Err(e) = fs::set_permissions(file_path, _permissions) {
            println!("{}", e);  // Print an error message
            std::process::exit(146);  // Exit the program with an error code
        }
    } else {
        let permissions = &args[2];
        let users_for_add = permissions.split("+").collect::<Vec<&str>>();
        let users_for_del = permissions.split("-").collect::<Vec<&str>>();
        let file_path = &args[3];

        let mut _mode = fs::metadata(file_path)
            .map(|metadata| metadata.permissions().mode())
            .unwrap_or(0o644);

        if permissions.contains('+') {
            // Handle permissions to add (+rwx)
            for _i in users_for_add[1].chars() {
                match _i {
                    'r' => {
                        // Add read permission
                        if users_for_add[0].contains("a") {
                            _mode |= 0o444;
                        }
                        if users_for_add[0].contains("u") {
                            _mode |= 0o400;
                        }
                        if users_for_add[0].contains("g") {
                            _mode |= 0o040;
                        }
                        if users_for_add[0].contains("o") {
                            _mode |= 0o004;
                        }
                    }
                    'w' => {
                        // Add write permission
                        if users_for_add[0].contains("a") {
                            _mode |= 0o222;
                        }
                        if users_for_add[0].contains("u") {
                            _mode |= 0o200;
                        }
                        if users_for_add[0].contains("g") {
                            _mode |= 0o020;
                        }
                        if users_for_add[0].contains("o") {
                            _mode |= 0o002;
                        }
                    }
                    'x' => {
                        // Add execute permission
                        if users_for_add[0].contains("a") {
                            _mode |= 0o111;
                        }
                        if users_for_add[0].contains("u") {
                            _mode |= 0o100;
                        }
                        if users_for_add[0].contains("g") {
                            _mode |= 0o010;
                        }
                        if users_for_add[0].contains("o") {
                            _mode |= 0o001;
                        }
                    },
                    _ => {
                        println!("{}", 146);  // Print an error code
                        std::process::exit(146);  // Exit the program with an error code
                    }
                }
            }
        } else if permissions.contains('-') {
            // Handle permissions to remove (-rwx)
            for _i in users_for_del[1].chars() {
                match _i {
                    'r' => {
                        // Remove read permission
                        if users_for_del[0].contains("a") {
                            _mode &= !0o444;
                        }
                        if users_for_del[0].contains("u") {
                            _mode &= !0o400;
                        }
                        if users_for_del[0].contains("g") {
                            _mode &= !0o040;
                        }
                        if users_for_del[0].contains("o") {
                            _mode &= !0o004;
                        }
                    }
                    'w' => {
                        // Remove write permission
                        if users_for_del[0].contains("a") {
                            _mode &= !0o222;
                        }
                        if users_for_del[0].contains("u") {
                            _mode &= !0o200;
                        }
                        if users_for_del[0].contains("g") {
                            _mode &= !0o020;
                        }
                        if users_for_del[0].contains("o") {
                            _mode &= !0o002;
                        }
                    }
                    'x' => {
                        // Remove execute permission
                        if users_for_del[0].contains("a") {
                            _mode &= !0o111;
                        }
                        if users_for_del[0].contains("u") {
                            _mode &= !0o100;
                        }
                        if users_for_del[0].contains("g") {
                            _mode &= !0o010;
                        }
                        if users_for_del[0].contains("o") {
                            _mode &= !0o001;
                        }
                    },
                    _ => {
                        println!("{}", 146);  // Print an error code
                        std::process::exit(146);  // Exit the program with an error code
                    }
                }
            }
        }

        // Apply the modified permissions to the file
        if let Err(e) = fs::set_permissions(file_path, fs::Permissions::from_mode(_mode)) {
            println!("{}", e);  // Print an error message
            std::process::exit(146);  // Exit the program with an error code
        }
    }
}


fn grep(args: Vec<String>) {
    // Check if the correct number of arguments is provided
    if args.len() < 4 {
        println!("{}", 136);  // Print an error code
        std::process::exit(136);  // Exit the program with an error code
    } else {
        let mut i = 3;
        while i < args.len() {
            if let Ok(contents) = fs::read_to_string(&args[i]) {
                let mut _j = 0;
                for line in contents.lines() {
                    // Check if the line contains the search pattern
                    if line.contains(&args[2]) {
                        println!("{}", line);  // Print the matching line
                    }
                    _j += 1;
                }
            } else {
                println!("{}", 136);  // Print an error code
                std::process::exit(136);  // Exit the program with an error code
            }
            i += 1;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "pwd" => {
            // Print the current working directory
            pwd(); 
        }
        "echo" => {
            // Echo command implementation
            echo(args); 
        }
        "cat" => {
            // Concatenate and display file contents
            cat(args); 
        }
        "mkdir" => {
            // Create a directory
            mkdir(args); 
        }
        "mv" => {
            // Move or rename files
            mv(args); 
        }
        "ln" => {
            // Create links
            ln(args); 
        }
        "rmdir" => {
            // Remove a directory
            rmdir(args); 
        }
        "rm" => {
            // Remove files or directories
            rm(args); 
        }
        "ls" => {
            // List directory contents
            ls(args); 
        }
        "cp" => {
            // Copy files or directories
            cp(args); 
        }
        "touch" => {
            // Create or update file timestamps
            touch(args); 
        }
        "chmod" => {
            // Change file permissions
            chmod(args); 
        }
        "grep" => {
            // Search for patterns in text
            grep(args); 
        }
        _ => {
            println!("Invalid command");
            std::process::exit(255);
        }
    }
}