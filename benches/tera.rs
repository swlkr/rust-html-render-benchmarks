use lazy_static::lazy_static;
use serde::Serialize;
use tera::{Context, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/tera_*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html"]);
        tera
    };
}

fn main() {
    divan::main()
}

const SIZE: usize = 100;

#[divan::bench]
fn big_table(bencher: divan::Bencher) {
    let mut table = Vec::with_capacity(SIZE);
    for _ in 0..SIZE {
        let mut inner = Vec::with_capacity(SIZE);
        for i in 0..SIZE {
            inner.push(i);
        }
        table.push(inner);
    }
    let big_table = BigTable { table };

    bencher.bench(|| {
        TEMPLATES
            .render(
                "tera_big_table.html",
                &Context::from_serialize(&big_table).unwrap(),
            )
            .unwrap()
    });
}

#[derive(Serialize)]
struct BigTable {
    table: Vec<Vec<usize>>,
}

#[divan::bench]
fn teams(bencher: divan::Bencher) {
    let teams = Teams {
        year: 2015,
        teams: vec![
            Team {
                name: "Jiangsu".into(),
                score: 43,
            },
            Team {
                name: "Beijing".into(),
                score: 27,
            },
            Team {
                name: "Guangzhou".into(),
                score: 22,
            },
            Team {
                name: "Shandong".into(),
                score: 12,
            },
        ],
    };

    bencher.bench(|| {
        TEMPLATES
            .render("tera_teams.html", &Context::from_serialize(&teams).unwrap())
            .unwrap()
    })
}

#[derive(Serialize)]
struct Teams {
    year: u16,
    teams: Vec<Team>,
}

#[derive(Serialize)]
struct Team {
    name: String,
    score: u8,
}
