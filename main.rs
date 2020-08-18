use std::{io, fmt}; 
use std::io::{Write, Read};

fn main() -> Result<(), Error> {
   
    crossterm::terminal::enable_raw_mode()?; 
    
    let mut stdout = io::stdout(); 
    let mut stdin = io::stdin().bytes(); 

    loop{
        write!(stdout, "Type something > ")?;
        stdout.flush()?; 

        let byte = match stdin.next(){
            Some(val) => val?, //need the question mark here cuz its a double unwrap, we need to unwrap the option that is returned by stdin.next()
            None => break, 
        };

        let c = char::from(byte); 

        if c == 'q' {break;}
        write!(stdout, "You typed {}\n\r", c)?; 
        stdout.flush()?; 

    }//loop

    crossterm::terminal::disable_raw_mode()?; 
    Ok(())
}

#[derive(Debug)]
enum Error {
    CrosstermError(crossterm::ErrorKind), 
    IoError(io::Error),
}//our own umbrella Error type. 

impl From<crossterm::ErrorKind> for Error{
    fn from(input_error: crossterm::ErrorKind) -> Error{
        Error::CrosstermError(input_error) 
    }
}

impl From<io::Error> for Error{
    fn from (input_error: io::Error) -> Self{
        Error::IoError(input_error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Error::CrosstermError(inner) => write!(f, "{}", inner), 
            Error::IoError(inner) => write!(f, "{}", inner), 
        }
    }
}

impl std::error::Error for Error{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self{
            Error::CrosstermError(x) => Some(x), 
            Error::IoError(x) => Some(x),  
        }
    }
}