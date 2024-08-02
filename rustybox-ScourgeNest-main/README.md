# <p style="text-align: center;">Copyright Niculici Mihai-Daniel 2023</p>


# <p style="text-align: center;">Tema 1 PCLP 4</p>

# <p style="text-align: center;">Rustybox</p>

* This program interprets command-line parameters to execute various file and system operations.
* It operates as a versatile command-line utility, handling commands like ls, rm, cp, chmod, and grep. These commands trigger actions such as directory listing, file removal, copying, permission modification, and text searching.
* The program carefully validates and processes these arguments, ensuring that the requested actions are carried out accurately.
 * Additionally, it handles error scenarios, providing meaningful error codes to enhance user feedback, making it a powerful and robust tool for file and system management tasks.

# Valid commands are:
        - pwd - Print the current working directory
        - echo [option] [arguments] - outputs text or variables to the console
        - cat [name_of_file] - Concatenate and display file contents
        - mkdir [name_of_dir] - Create a directory
        - mv [source] [destination] - Move or rename files
        - ln [optiune] [source] [name_of_link] - Create links
        - rmdir [name_of_dir] - Remove a directory
        - rm [options] [files/dirs] - Remove files or directories
        - ls [options] [director] - List directory contents
        - cp [option] [source] [destination] - Copy files or directories
        - touch [options] [file] - Create or update file timestamps
        - chmod [permissions] [file/dir] - Change file/dir permissions
        - grep [-i] [regex] [name_of_file] - Search for patterns in text


## <p style="text-align: center;">Commands explained</p>

* `pwd` command retrieves and prints the current directory path or exits with code 206 on failure, offering insights into the working directory
#
* `echo` prints text, excluding "-n," with spaces in between. If "-n" is present, it omits the newline after printing the text. Handles argument count and errors.
#
* `cat` reads and prints text from files specified as arguments. It checks argument count, reads file contents, and handles errors by printing an error message and exiting.
#
* `mkdir` creates directories specified as arguments. It checks argument count, attempts to create directories with provided paths, prints errors if any occur, and exits with an error code.
#
* `mv` renames a source file or directory to a target path. It checks argument count, attempts the renaming operation, prints errors if any occur, and exits with an error code if needed.
#
* `ln` handles hard and symbolic link creation. It checks argument count and options, attempts link creation, and prints errors or exits with an error status code as needed.
#
* `rmdir` removes directories. It checks argument count, attempts removal for provided paths, and prints errors or exits with an error status code when needed.
#
* `rm` function removes files or directories based on provided arguments. It handles various flags and options like recursive removal and checks for errors during the process. If needed, it prints error messages and exits with an error status code.
#
* `ls` function lists the contents of directories based on provided arguments, supporting various options like recursive listing and excluding hidden files. It handles errors and exits with status codes when needed.
#
* `cp` function copies files and directories based on the provided arguments. It supports recursive copying and handles various scenarios, including creating directories if they don't exist in the destination. The code ensures proper error handling and exits with status codes when necessary.
#
* `touch` function is responsible for updating the access and modification times of files. It takes different options into account and acts accordingly. When executed with specific options, it may either create new files or modify the timestamps of existing ones. It also handles errors and exits with appropriate status codes when necessary.
#
* `chmod` function updates file permissions. It reads an octal mode or +/- permissions and applies changes to files. Handles errors and exits appropriately.
#
* `grep` function searches for a pattern in text files. Given a pattern and file(s), it reads each file, searches for the pattern, and prints matching lines. Handles errors and exits accordingly.
#
* When an invalid command is encountered, the program exits returns -1.