#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Content {
    title: String,
    img_url: String,
    content_url: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Review {
    content: String,
    ratings: Vec<Rating>,
}

struct Select {
    title: String,
    options: Vec<String>,
    value: String,
}

struct Score {
    title: String,
    min: f64,
    max: f64,
    value: f64,
}

struct Text {
    title: String,
    value: String,
}

struct Number {
    title: String,
    value: i64,
}

struct Toggle {
    title: String,
    value: bool,
}

enum Rating {
    Select(Select),
    Score(Score),
    Text(Text),
    Number(Number),
    Toggle(Toggle),
}
