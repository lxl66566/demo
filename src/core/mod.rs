pub mod sub1;
pub mod sub2;

pub trait Process {
    fn f(&self);
}

pub trait Takable {
    fn take(&self);
}

impl<T> Takable for T
where
    T: Process,
{
    fn take(&self) {
        self.f();
    }
}
