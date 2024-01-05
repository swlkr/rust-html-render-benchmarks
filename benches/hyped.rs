use hyped::*;

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

    bencher.bench(|| big_table_render(&table))
}

fn big_table_render(rows: &Vec<Vec<usize>>) -> String {
    render(table(
        rows.iter()
            .map(|row| tr(row.iter().map(|col| td(*col)).collect::<Vec<_>>()))
            .collect::<Vec<_>>(),
    ))
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

    bencher.bench(|| teams_render(&teams))
}

fn teams_render(teams: &Teams) -> String {
    render((html((
        head(title(teams.year)),
        body((
            h1(("CSL ", teams.year)),
            ul(teams
                .teams
                .iter()
                .enumerate()
                .map(|(idx, team)| {
                    li((b(team.name.clone()), ": ", team.score)).class(if idx == 0 {
                        "champion"
                    } else {
                        ""
                    })
                })
                .collect::<Vec<_>>()),
        )),
    )),))
}

struct Teams {
    year: u16,
    teams: Vec<Team>,
}

struct Team {
    name: String,
    score: u8,
}
