use std::{io, fmt}; 
use std::io::{Write, Read};
use git2::{Repository, Revwalk, Oid, BranchType}; 
use chrono::prelude::*; 
//1:28:34 / 3:34:04
//https://www.youtube.com/watch?v=YFzF1AHYjes

type Result<T, E = Error> =  std::result::Result<T,E>; 

fn get_branches(repo: &Repository) -> Result<Vec<Branch>, Error>{
    let mut branches = 
        repo.branches(Some(BranchType::Local))?
        .map(|branch| -> Result<_, Error> {
            let (branch,_) = branch?;  
            let name = String::from_utf8(branch.name_bytes()?.to_vec())?;

            let commit = branch.get().peel_to_commit()?; 
            
            let time = commit.time(); 
            let offset = chrono::Duration::minutes(i64::from(time.offset_minutes())); 
            let time = NaiveDateTime::from_timestamp(time.seconds(), 0) + offset;
            
            Ok(Branch{
                id : commit.id(), 
                time: time, 
                name: name,
            })
    }).collect::<Result<_>>()?; //the _ is there cuz Rust can infer the return type from the return type of func
    

    Ok(branches)
}

fn main()->Result<(), Error> {
    let repo = Repository::open_from_env()?; 
    let mut stdout = io::stdout(); 



    let mut rev_walk = repo.revwalk()?; 
        rev_walk.push_head()?; 
        rev_walk.set_sorting(git2::Sort::TIME | git2::Sort::REVERSE)?; 

        for rev in rev_walk{
            // let rev = rev?; 
            let commit = repo.find_commit(rev?)?; 
            // let message = match commit.message_raw(){
            //     None => break,
            //     Some(msg) => msg, 
            // }; 

            let message = commit.summary_bytes().unwrap_or_else(|| commit.message_bytes());


            write!(stdout,"{}", String::from_utf8_lossy(message))?;  
            write!(stdout, "\n")?;
        } 

    Ok(())
}

#[derive(Debug)]
struct Branch{
    id: Oid, 
    time: NaiveDateTime, 
    name: String, //allocated on the heap. 
}

// fn main() -> Result<(), Error> {
   
//     crossterm::terminal::enable_raw_mode()?; 
    
//     let mut stdout = io::stdout(); 
//     let mut stdin = io::stdin().bytes(); 

//     loop{
//         write!(stdout, "Type something > ")?;
//         stdout.flush()?; 

//         let byte = match stdin.next(){
//             Some(val) => val?, //need the question mark here cuz its a double unwrap, we need to unwrap the option that is returned by stdin.next()
//             None => break, 
//         };

//         let c = char::from(byte); 

//         if c == 'q' {break;}
//         write!(stdout, "You typed {}\n\r", c)?; 
//         stdout.flush()?; 

//     }//loop

//     crossterm::terminal::disable_raw_mode()?; 
//     Ok(())
// }

#[derive(Debug)]
enum Error {
    CrosstermError(crossterm::ErrorKind), 
    IoError(io::Error),
    GitError(git2::Error), 
    FromUtf8Error(std::string::FromUtf8Error), 
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
            Error::GitError(error_val) =>  write!(f, "{}", error_val), 
            Error::FromUtf8Error(x) => write!(f, "{}", x), 
        }
    }
}//this function allows to actually print what the error is. 

impl std::error::Error for Error{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self{
            Error::CrosstermError(x) => Some(x), 
            Error::IoError(x) => Some(x),  
            Error::GitError(x) => Some(x), 
            Error::FromUtf8Error(x) => Some(x), 
        }
    }
}

impl From<git2::Error> for Error{
    fn from(error: git2::Error) -> Error{
        Error::GitError(error)
    }
}

impl From<std::string::FromUtf8Error> for Error{
    fn from(error: std::string::FromUtf8Error) -> Error{
        Error::FromUtf8Error(error)
    }
}

