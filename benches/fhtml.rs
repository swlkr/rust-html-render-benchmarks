use std::fmt::Write;

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
    let page = fhtml::format! {
        <table>
            {
                rows.iter().fold(String::new(), |mut f, cols| {
                    let _ = fhtml::write! { f,
                        <tr>
                            {
                                cols.iter().fold(String::new(), |mut f, col| {
                                    let _ = fhtml::write! { f,
                                        <td>{col}</td>
                                    };
                                    f
                                })
                            }
                        </tr>
                    };
                    f
                })
            }
        </table>
    };
    page.to_string()
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
    fhtml::format! {
        <html>
            <head>
                <title>{teams.year}</title>
            </head>
            <body>
                <h1>"CSL " {teams.year}</h1>
                <ul>
                    {
                        teams.teams.iter().enumerate().fold(String::new(), |mut f, (idx, team)| {
                            let _ = fhtml::write! { f,
                                <li class={if idx == 0 { "champion" } else { "" }}>
                                    <b>{team.name}</b> ": " {team.score}
                                </li>
                            };
                            f
                        })
                    }
                </ul>
            </body>
        </html>
    }
}

struct Teams {
    year: u16,
    teams: Vec<Team>,
}

struct Team {
    name: String,
    score: u8,
}
