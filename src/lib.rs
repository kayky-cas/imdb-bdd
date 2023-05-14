use std::marker::PhantomData;

pub mod genre;
pub mod movie;
pub mod person;
pub mod show;
pub mod user;
pub mod worker;

pub trait Random {
    fn random() -> Self;
}

pub struct RandomVec<T>
where
    T: Random,
{
    marker: PhantomData<T>,
}

impl<T> RandomVec<T>
where
    T: Random,
{
    pub fn random_vec(len: usize) -> Vec<T> {
        let mut list = Vec::new();

        for _ in 0..len {
            list.push(T::random())
        }

        list
    }
}

struct Imdb {
    people: Vec<person::Person>,
    movies: Vec<movie::Movie>,
    shows: Vec<show::Show>,
    users: Vec<user::User>,
}

impl Random for Imdb {
    fn random() -> Self {
        Self {
            people: RandomVec::<person::Person>::random_vec(50),
            movies: RandomVec::<movie::Movie>::random_vec(30),
            shows: RandomVec::<show::Show>::random_vec(30),
            users: RandomVec::<user::User>::random_vec(10),
        }
    }
}
