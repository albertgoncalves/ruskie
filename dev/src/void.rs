pub trait OptionExt<T> {
    fn void(self);
}

impl<T> OptionExt<T> for Option<T> {
    fn void(self) {
        self.map_or_else(|| (), |_| ())
    }
}

pub trait ResultExt<T, E> {
    fn void(self);
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn void(self) {
        self.map(|_| ()).unwrap_or(())
    }
}
