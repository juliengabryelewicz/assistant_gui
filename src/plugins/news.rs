use std::io::BufReader;
use rss::Channel;
use std::error::Error;

fn get_news_from_newspaper(url_newspaper:&str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::blocking::get(url_newspaper)?;
    let channel = Channel::read_from(BufReader::new(content)).unwrap();
    Ok(channel)
}

pub fn get_news(newspaper:Newspaper) -> Result<Channel, Box<dyn Error>> {
    match newspaper{
        Newspaper::Lefigaro => get_news_from_newspaper("https://www.lefigaro.fr/rss/figaro_actualites.xml"),
        Newspaper::Lemonde => get_news_from_newspaper("https://www.lemonde.fr/rss/une.xml"),
        Newspaper::Marianne => get_news_from_newspaper("https://www.marianne.net/rss.xml"),
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Newspaper {
    Lefigaro,
    Lemonde,
    Marianne,
}

impl Newspaper {
    pub const ALL: [Newspaper; 3] = [
        Newspaper::Lefigaro,
        Newspaper::Lemonde,
        Newspaper::Marianne,
    ];
}

impl Default for Newspaper {
    fn default() -> Newspaper {
        Newspaper::Lefigaro
    }
}

impl std::fmt::Display for Newspaper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Newspaper::Lefigaro => "Le Figaro",
                Newspaper::Lemonde => "Le Monde",
                Newspaper::Marianne => "Marianne",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_news() {
        assert!(!get_news(Newspaper::Lefigaro).is_err())
    }

    #[test]
    fn test_get_news_from_newspaper() {
        assert!(!get_news_from_newspaper("https://www.lefigaro.fr/rss/figaro_actualites.xml").is_err())
    }
}