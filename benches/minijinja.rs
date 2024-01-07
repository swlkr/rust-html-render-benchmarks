use minijinja::{context, Environment};
use serde::Serialize;
fn main() {
    divan::main()
}

const SIZE: usize = 100;
static BIG_TABLE: &'static str = r#"
<table>
{% for row in table %}
<tr>{% for col in row %}<td>{{ col|escape }}</td>{% endfor %}</tr>
{% endfor %}
</table>
"#;

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
    let mut env = Environment::new();
    env.add_template("big_table", BIG_TABLE).unwrap();
    let template = env.get_template("big_table").unwrap();
    let big_table = BigTable { table };

    bencher.bench(|| template.render(&big_table).unwrap())
}

#[derive(Serialize)]
struct BigTable {
    table: Vec<Vec<usize>>,
}

static TEAMS: &'static str = r#"
<html>
  <head>
    <title>{{ year }}</title>
  </head>
  <body>
    <h1>CSL {{ year }}</h1>
    <ul>
    {% for team in teams %}
      <li class="{% if loop.index0 == 0 %}champion{% endif %}">
      <b>{{ team.name }}</b>: {{ team.score }}
      </li>
    {% endfor %}
    </ul>
  </body>
</html>
"#;

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
    let mut env = Environment::new();
    env.add_template("teams", TEAMS).unwrap();
    let template = env.get_template("teams").unwrap();

    bencher.bench(|| template.render(&teams).unwrap())
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
