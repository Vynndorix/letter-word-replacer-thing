use std::fs;

fn main() -> std::io::Result<()> {
    let filename = "your_file.txt";
    let contents = fs::read_to_string(filename)?;

    let modified_contents = contents
        .replace("e", "penis");
        //        ^ replaces every e with penis

    fs::write(filename, modified_contents)?;

    Ok(())

    //put a whole movie script or just a couple regular sentences or paragraph in your file then update the filename variable. Put the bee movie in the file.
    //just test it out :D 
    //it will replace every e with penis

}
