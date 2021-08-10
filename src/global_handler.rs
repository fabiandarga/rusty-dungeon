use crate::errors::Error;

pub struct GlobalHandler<'a> {
    pub quit_fn: &'a mut dyn FnMut() -> Result<(), Error> ,
}

impl<'a> GlobalHandler<'a> {
    pub fn quit(&mut self) -> Result<(), Error> {
        (self.quit_fn)()?;
        Ok(())
    }
}
