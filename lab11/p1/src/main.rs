use anyhow::Result;
use std::fs::File;
use std::io::Write;

struct MyWriter<W : Write>{ 
    file : W
}

impl<W : Write> MyWriter<W>{
    fn new(file : W) -> MyWriter<W>{
        MyWriter{ file }
    }
}

impl<W : Write> Write for MyWriter<W>{
    fn write(self : &mut Self,content : &[u8]) -> Result<usize, std::io::Error>{
        let mut duplicated_content = Vec::with_capacity(content.len()*2);

        for &byte in content {
            duplicated_content.push(byte);
            duplicated_content.push(byte);
        }

        self.file.write_all(&duplicated_content)?;

        Ok(duplicated_content.len())
    }
    fn flush(self : &mut Self)->Result<(), std::io::Error>{
        self.file.flush()
    }
}
fn main() -> Result<()> {
    let mut writer = MyWriter::new(File::create("a.txt")?);
    writer.write_all(b"abc")?;

    Ok(())
}