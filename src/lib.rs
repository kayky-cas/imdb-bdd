use media::Media;
use movie::Movie;
use person::Person;
use rand::Rng;
use show::Show;
use std::marker::PhantomData;
use user::User;
use work::Work;

mod genre;
mod media;
mod movie;
mod person;
mod ratting;
mod show;
mod user;
mod work;

const PEOPLE_QUANTITY: usize = 100;
const MOVIES_QUANTITY: usize = 30;
const SHOWS_QUANTITY: usize = 30;
const USERS_QUANTITY: usize = 300;

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
    people: Vec<Person>,
    movies: Vec<Movie>,
    shows: Vec<Show>,
    users: Vec<User>,
}

impl Imdb {
    fn create_random_works_show(&mut self, person_idx: usize, role: work::Role) {
        let person = &mut self.people[person_idx];
        let mut show_idx = rand::thread_rng().gen_range(0..self.shows.len());

        while self.shows[show_idx].release < person.birth_date {
            show_idx = rand::thread_rng().gen_range(0..self.movies.len());
        }

        match &role {
            work::Role::Actor(_) => {
                let mut episodes = Vec::with_capacity(
                    rand::thread_rng().gen_range(1..self.shows[show_idx].episodes_quantity()),
                );

                while episodes.len() < episodes.capacity() {
                    let episode_idx =
                        rand::thread_rng().gen_range(0..self.shows[show_idx].episodes_quantity());

                    if !episodes.contains(&episode_idx) {
                        self.shows[show_idx].episodes[episode_idx]
                            .append_worker(Work::for_media(&person, role.clone()));

                        let episode = &self.shows[show_idx].episodes[episode_idx];
                        person.append_work(Work::for_person(
                            episode.id,
                            episode.title.clone(),
                            Media::Show(episode.number),
                            role.clone(),
                        ));

                        episodes.push(episode_idx);
                    }
                }
            }
            _ => {
                for episode in &mut self.shows[show_idx].episodes {
                    episode.append_worker(Work::for_media(&person, role.clone()));
                    person.append_work(Work::for_person(
                        episode.id,
                        episode.title.clone(),
                        Media::Show(episode.number),
                        role.clone(),
                    ));
                }
            }
        };
    }

    fn create_random_works_movie(&mut self, person_idx: usize, role: work::Role) {
        let person = &mut self.people[person_idx];

        let mut movie_idx = rand::thread_rng().gen_range(0..self.movies.len());

        while self.movies[movie_idx].release < person.birth_date {
            movie_idx = rand::thread_rng().gen_range(0..self.movies.len());
        }

        let movie = &self.movies[movie_idx];

        person.append_work(Work::for_person(
            movie.id,
            movie.title.clone(),
            Media::Movie,
            role.clone(),
        ));
        self.movies[movie_idx].append_worker(Work::for_media(&person, role.clone()));
    }

    fn create_random_works(&mut self) {
        for person_idx in 0..self.people.len() {
            let role_type = work::RoleType::random();

            for _ in 0..rand::thread_rng().gen_range(0..=7) {
                let role = work::Role::random(&role_type);
                let media = media::Media::random();

                match media {
                    Media::Movie => self.create_random_works_movie(person_idx, role),
                    Media::Show(_) => self.create_random_works_show(person_idx, role),
                }
            }
        }
    }

    fn create_random_ratting(&mut self) {}
}

impl Random for Imdb {
    fn random() -> Self {
        let mut imdb = Self {
            people: RandomVec::random_vec(PEOPLE_QUANTITY),
            movies: RandomVec::random_vec(MOVIES_QUANTITY),
            shows: RandomVec::random_vec(SHOWS_QUANTITY),
            users: RandomVec::random_vec(USERS_QUANTITY),
        };

        imdb.create_random_works();

        imdb
    }
}
