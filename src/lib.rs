mod genre;
mod movie;
mod person;
mod show;
mod user;
mod worker;

pub trait Random {
    fn random() -> Self;
}

pub trait RandomVec<T>
where
    T: Random,
{
    fn random_vec(len: usize) -> Vec<T>;
}
