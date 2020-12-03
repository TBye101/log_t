pub mod logging_implementations {

    extern crate chrono;
    use crate::logging_abstraction::Logger;
    use contracts::*; 
    use std::{fs::File, io::Write};

    const UNINITIALIZED_FILE_HANDLE_EXCEPTION: &str = "The log file was not initialized before writing to the log.";

    pub struct FileLogger {
        ///The path to the disk stored log file.
        pub logfile_path: String,

        ///Our actual handle to manipulate the file we use for logging.
        file_handle: Option<File>
        }

        impl FileLogger {

            /// Initializes a new FileLogger, and calls the open() function.
            #[debug_requires(!file_path.trim().is_empty(), "Empty or whitespace filepaths are not allowed")]
            pub fn new_from_string(file_path: String) -> Result<FileLogger, std::io::Error> {
                let mut new_logger = FileLogger {
                    logfile_path: file_path,
                    file_handle: None };

                //I need to specify that open() is only borrowing "self" to return ownership back, instead of it moving
                match new_logger.open() {
                    Some(error) => Err(error),
                    None => return Ok(new_logger)
                }
            }

            /// Initializes a new FileLogger, and calls the open() function.
            pub fn new_from_static_string(file_path: &str) -> Result<FileLogger, std::io::Error> {
                return FileLogger::new_from_string(String::from(file_path));
            }

            ///Formats a log entry by creating a timestamp, then adding the data to it.
            #[debug_requires(!to_write.to_string().is_empty(), "We shouldn't try to write empty data to the log")]
            #[debug_ensures(!ret.trim().is_empty(), "The formatted log entry shouldn't be empty")]
            fn format_entry<T>(&self, to_write: T) -> String
                where T: std::fmt::Display
            {
                let current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
                return std::format!("[{}]: {}\n", current_time, to_write);
            }
        }

        impl Logger for FileLogger {

            fn open(&mut self) -> Option<std::io::Error>
            {
                let created_file = File::create(&self.logfile_path);
                
                match created_file {
                    Ok(created_file_handle) => self.file_handle = Some(created_file_handle),
                    Err(error) => return Some(error)
                }

                None
            }

            fn write<T: std::fmt::Display>(&self, to_write: T) -> Option<std::io::Error>
            {
                //Verify that the log file has been opened already
                if self.file_handle.is_none() {
                    return Some(std::io::Error::new(std::io::ErrorKind::NotFound, UNINITIALIZED_FILE_HANDLE_EXCEPTION));
                }
                
                let mut log_file = self.file_handle.as_ref().unwrap();
                //Format our data and timestamp together
                let to_write = self.format_entry(to_write);
                println!("{}", to_write);
                let write_result = log_file.write_all(to_write.as_bytes());
                
                if write_result.is_err() {
                    return write_result.err();
                }

                None
            }
    
            fn write_slice<T>(&self, to_write: &[&T]) -> std::option::Option<std::io::Error>
                where T: std::fmt::Display
            {
                //Write each item to our file destination
                for item in to_write {
                    let write_result = self.write(item);
                    if write_result.is_some() {
                        return write_result;
                    }
                }

                None
            }
        }
}