use clap::Parser;
use std::{io, fs, path::PathBuf,collections::HashMap};
//TODO: implement mod file_tree :ODOT//
#[derive(Parser)]
#[command(name = "YUNODO")]
#[command(version = "0.5.0")]
#[command(about = "parse file tree for //TODO: comments", long_about = "parses a directory of files for substrings of //TODO: and outputs all instances in a parsable format")]
struct Cli {
    #[arg(short, long, value_name = "PATH")]
    path: Option<PathBuf>,
    #[arg(short, long, value_name = "FORMAT")]
    format: Option<String>,
    #[arg(short, long)]
    debug: Option<bool>,
}
fn main() {
    let cli = Cli::parse();
    if let Some(path) = cli.path.as_deref() {
        let path_string = path.display().to_string();
        let mut output_csv_item: String = String::new();
        match read_files_in_directory(&path_string.as_str()) {
            Ok(files_content) => {
                for (filename, lines) in files_content {
                    for (line_number, line) in lines.iter().enumerate() {
                        // Check if the line starts with "//" and not within strings or other signatures
                        if !line.contains("//") {
                            continue;
                        }

                        // Variables to track if we are within a string, comment, or signature
                        let mut in_string = false;
                        let mut in_comment = false;
                        let mut _in_signature = false;

                        // Iterate through each character in the line
                        for (i, c) in line.chars().enumerate() {
                            match c {
                                '"' => in_string = !in_string,
                                '/' if !in_string => {
                                    if i + 1 < line.len() && line.chars().nth(i + 1).unwrap() == '/' {
                                        // Check if it's a comment
                                        in_comment = true;
                                        break;
                                    } else if i + 1 < line.len() && line.chars().nth(i + 1).unwrap() == '*' {
                                        in_comment = true;
                                    }
                                }
                                '*' if i + 1 < line.len() && line.chars().nth(i + 1).unwrap() == '/' && in_comment => {
                                    // Check for end of block comment
                                    in_comment = false;
                                    break;
                                }
                                _ => (),
                            }
                        }

                        if in_comment {
                            // Extract TODO comments
                            let v: Vec<&str> = line.split("//TODO:").collect();
                            if let Some(last_part) = v.last() {
                                if let Some(end_index) = last_part.find(":ODOT//") {
                                    let extracted = &last_part[..end_index];
                                    // Format output CSV item
                                    if !output_csv_item.is_empty(){
                                        //OUTPUT_CSV_TO_APPEND, PATH, FILENAME, LINE_NUMBER, EXTRACTED_COMMENT
                                         output_csv_item = format!("{},{},{},{},{}", output_csv_item, path_string, filename, line_number + 1, extracted);
                                    } else {
                                        //PATH, FILENAME, LINE_NUMBER, EXTRACTED_COMMENT
                                         output_csv_item = format!("{},{},{},{}",path_string, filename, line_number + 1, extracted);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
        if let Some(format) = cli.format.as_deref() {
            //TODO: write match to match format and select which output :ODOT//
            match format {
                "md"|"MD" => out_as_md_table(output_csv_item.clone()),
                "json"|"JSON" => out_as_json_object(output_csv_item.clone()),
                "yaml"|"YAML" => out_as_yaml_file(output_csv_item.clone()),
                "toml"|"TOML" => out_as_toml_file(output_csv_item.clone()),
                _ => println!("That's not a supported format")
            }
        }
    }
}

/// Reads files in a given directory and its subdirectories.
///
/// # Arguments
///
/// * `dir_path` - A string slice representing the path to the directory.
///
/// # Returns
///
/// A Result containing a vector of tuples where each tuple contains the filename and
/// a vector of lines in the file, or an io::Error if an error occurs during file I/O.
fn read_files_in_directory(dir_path: &str) -> io::Result<Vec<(String, Vec<String>)>> {
    let mut files_content = Vec::new();
    let paths = fs::read_dir(dir_path)?;

    for path in paths {
        let entry = path?;
        let path = entry.path();
        if path.is_file() {
            // If the entry is a file, read its content
            let filename = path.file_name().unwrap().to_string_lossy().into_owned();
            let content = fs::read_to_string(&path)?;
            let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
            files_content.push((filename, lines));
        } else if path.is_dir() {
            // If the entry is a directory, recursively call the function to read its content
            let subdir_path = path.to_string_lossy().into_owned();
            let subdir_content = read_files_in_directory(&subdir_path)?;
            files_content.extend(subdir_content);
        }
    }

    Ok(files_content)
}
fn out_as_md_table(input_csv:String){
    //TODO: iterate the input_csv and split into vec @ comma then pop vec backfilling table appending formatted values to new vec to be iterated and displayed :ODOT//
    let mut split_input: Vec<&str> = input_csv.split(',').collect();
    //Formatting Values
    let headers = String::from("| File Path | File Name | Line Number | Comment |");
    let divider = String::from("|:----------|:---------:|:-----------:|:--------|");

    //Output Vector
    let mut table:Vec<String> = Vec::new();
    table.push(headers);
    table.push(divider);
    split_and_print(&mut split_input, &mut table);
    for line in table {
        println!("{}",line);
    }
}
fn split_csv(vec: &mut Vec<&str>,split:usize) -> Vec<String> {
    let mut rows: Vec<String> = Vec::new();
    if !vec.is_empty() {
        let mut vec2 = vec.split_off(split);
        let row = vec.join(",");
        rows.push(row);
        let mut remaining_rows = split_csv(&mut vec2,split);
        rows.append(&mut remaining_rows);
    }
    rows
}

fn split_and_print(vec: &mut Vec<&str>,table: &mut Vec<String>) {
    if vec.is_empty() {
        return;
    }

    let mut vec2 = vec.split_off(4);
    let comment = vec.pop();
    let line = vec.pop();
    let name = vec.pop();
    let path = vec.pop();
    let spacer = '|';
    let formatted_row = format!("{} {} {} {} {} {} {} {} {}",spacer,path.unwrap(),spacer,name.unwrap(),spacer,line.unwrap(),spacer,comment.unwrap(),spacer);
    table.push(formatted_row);
    split_and_print(&mut vec2, table);
}
fn out_as_json_object(input_csv:String){
    let object_open_char = "{".to_string();    
    let object_close_char = "}".to_string();
    let depth_counter = 0;
    
    let mut split_input: Vec<&str> = input_csv.split(',').collect();
    let rows:Vec<String> = split_csv(&mut split_input,4);
    
    let mut output:Vec<String> = Vec::new();
    output.push(object_open_char);
    for row in rows {
        let mut cols: Vec<_> = row.split(',').collect();
        let obj_open = "    {";
        let obj_close = "    },";
        let comment = format!("        \"todo_comment\":\"{}\",", cols.pop().unwrap());
        let line_number = format!("        \"line_number\":\"{}\",", cols.pop().unwrap());
        let file_name = format!("        \"file_name\":\"{}\",", cols.pop().unwrap());
        let file_path = format!("        \"file_path\":\"{}\",", cols.pop().unwrap());
        output.push(obj_open.to_string());
        output.push(file_path.clone());
        output.push(file_name.clone());
        output.push(line_number.clone());
        output.push(comment.clone());
        output.push(obj_close.to_string());
    }
    output.push(object_close_char);
    for line in output {
        println!("{}",line)
    }
}
fn out_as_toml_file(input_csv: String) {
    // Split input CSV into rows
    let mut split_input: Vec<&str> = input_csv.split(',').collect();

    // Remove any empty elements
    split_input.retain(|&x| x.trim() != "");

    // Define data structures to store todos and headers
    let mut todos: HashMap<String, Vec<(String, String)>> = HashMap::new();

    // Parse each row of the CSV
    while !split_input.is_empty() {
        let path = split_input.remove(0).trim().to_string();
        let file = split_input.remove(0).trim().to_string();
        let line = split_input.remove(0).trim().to_string();
        let comment = split_input.remove(0).trim().to_string();
        let header = format!("{}{}", path, file);

        // Store todo item under the header
        todos.entry(header.clone()).or_insert(Vec::new()).push((line.clone(), comment.clone()));
    }

    // Write todos to TOML file
    let mut toml_output = String::new();
    for (header, todo_list) in todos {
        // Write header
        toml_output.push_str(&format!("[{}]\n", header));

        // Write todo items under this header
        for (i, (line, comment)) in todo_list.clone().into_iter().enumerate() {
            toml_output.push_str(&format!("[[todo]]\n"));
            toml_output.push_str(&format!("line = {}\n", line));
            toml_output.push_str(&format!("comment = \"{}\"\n", comment));
            if i < todo_list.len() - 1 {
                toml_output.push_str("\n"); // Separate todo items with a newline
            }
        }
    }
    println!("{}", toml_output);
}
fn out_as_yaml_file(input_csv: String) {
    // Split input CSV into rows
    let mut split_input: Vec<&str> = input_csv.split(',').collect();

    // Remove any empty elements
    split_input.retain(|&x| x.trim() != "");

    // Define data structures to store todos and headers
    let mut todos: HashMap<String, Vec<(String, String)>> = HashMap::new();

    // Parse each row of the CSV
    while !split_input.is_empty() {
        let path = split_input.remove(0).trim().to_string();
        let file = split_input.remove(0).trim().to_string();
        let line = split_input.remove(0).trim().to_string();
        let comment = split_input.remove(0).trim().to_string();
        let header = format!("{}{}", path, file);

        // Store todo item under the header
        todos.entry(header.clone()).or_insert(Vec::new()).push((line.clone(), comment.clone()));
    }

    // Write todos to YAML file
    let mut yaml_output = String::new();
    for (header, todo_list) in todos {
        // Write header
        yaml_output.push_str(&format!("\"{}\":\n", header));

        // Write todo items under this header
        for (line, comment) in todo_list {
            yaml_output.push_str("    \"item\":\n");
            yaml_output.push_str(&format!("        \"line_number\": \"{}\"\n", line));
            yaml_output.push_str(&format!("        \"comment\": \"{}\"\n", comment));
        }
    }

    // Print YAML output to terminal
    println!("{}", yaml_output);
}
