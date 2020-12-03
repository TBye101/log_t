pub mod logging_abstraction {
    pub trait Logger {
        
        ///Initializes the channel of communication to whatever destination the logger sends data to.
        fn open(&mut self) -> Option<std::io::Error>;

        ///Writes the a slice's data to the destination of the logger.
        fn write_slice<T: std::fmt::Display>(&self, to_write: &[&T]) -> Option<std::io::Error>;
        
        ///Writes the object's data to the destination of the logger.
        fn write<T: std::fmt::Display>(&self, to_write: T) -> Option<std::io::Error>;
    }
}