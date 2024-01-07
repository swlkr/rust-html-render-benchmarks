use html_node::{html, text};

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
    let page = html! {
        <table>
            {
                rows.into_iter().map(|cols| html! {
                    <tr>
                        {
                            cols.into_iter().map(|col| html! {
                                <td>{text!("{col}")}</td>
                            })
                        }
                    </tr>
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
    let page = html! {
        <html>
            <head>
                <title>{text!("{}", teams.year)}</title>
            </head>
            <body>
                <h1>CSL {text!("{}", teams.year)}</h1>
                <ul>
                    {
                        teams.teams.iter().enumerate().map(|(idx, team)| html! {
                            <li class={if idx == 0 { "champion" } else { "" }}>
                                <b>{text!("{}", team.name)}</b>: {text!("{}", team.score)}
                            </li>
                        })
                    }
                </ul>
            </body>
        </html>
    };
    page.to_string()
}

struct Teams {
    year: u16,
    teams: Vec<Team>,
}

struct Team {
    name: String,
    score: u8,
}
