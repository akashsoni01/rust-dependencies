#[derive(Debug, Clone)]
pub struct Repo {
    pub name: String,
}

impl Repo {
    pub fn mock() -> Self {
        Repo {
            name: "mock-repo".to_string(),
        }
    }
}

pub struct GitHub {
    fetch_repos: Box<dyn FnOnce(Box<dyn Fn(Vec<Repo>)>)>,
}

impl GitHub {
    pub fn mock() -> Self {
        GitHub {
            fetch_repos: Box::new(|callback| {
                callback(vec![Repo::mock()]);
            }),
        }
    }
}


//////////////////////////////////////////////
// Trait for Vec<Repo> mocks
pub trait RepoVecMock {
    fn mock() -> Vec<Repo>;
}

impl RepoVecMock for Vec<Repo> {
    fn mock() -> Vec<Repo> {
        vec![Repo::mock()]
    }
}

//////////////////////////////////////////////

fn main() {
    // Example usage
    let repos = Vec::<Repo>::mock();
    println!("Repos: {:?}", repos);

    let github = GitHub::mock();
    (github.fetch_repos)(Box::new(|repos| {
        println!("Fetched repos: {:?}", repos);
    }));
}
