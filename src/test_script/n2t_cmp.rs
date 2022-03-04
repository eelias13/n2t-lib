use super::OutType;

#[derive(Debug, Clone, PartialEq)]
pub struct N2tCmp {
    table: Vec<Vec<OutType>>,
    names: Vec<String>,
}

impl N2tCmp {
    pub fn new(table: Vec<Vec<OutType>>, names: Vec<&str>) -> Self {
        let names = names.iter().map(|&v| v.to_string()).collect();
        Self { table, names }
    }

    pub fn from_code(code: &str, head: Vec<OutType>) -> Result<Self, String> {
        let mut table = Vec::new();
        for _ in head.iter() {
            table.push(Vec::new());
        }

        let mut lines: Vec<String> = code
            .split("\n")
            .filter(|s| s.contains("|"))
            .map(|s| s.to_string().replace(" ", "").replace("\t", ""))
            .collect();

        let names: Vec<String> = lines[0]
            .split("|")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        lines.remove(0);
        if names.len() != table.len() {
            return Err(format!(
                "head len {} dose not match names len {}",
                table.len(),
                names.len()
            ));
        }

        for line in lines {
            let values: Vec<String> = line
                .split("|")
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();

            if values.len() != table.len() {
                return Err(format!(
                    "table len {} mismatch with values len {}",
                    table.len(),
                    values.len()
                ));
            }

            for i in 0..table.len() {
                let mut is_clock = false;
                let mut val = values[i].clone();
                if values[i].chars().last() == Some('+') {
                    is_clock = true;
                    val = val[0..val.len() - 1].to_string();
                }

                let out_type = match head[i] {
                    OutType::Clock(_) => OutType::Clock((
                        if let Ok(res) = val.parse() {
                            res
                        } else {
                            return Err(format!("{} is not a number", val));
                        },
                        is_clock,
                    )),
                    OutType::Binary(_) => OutType::Binary(parse_bool(val)?),
                    OutType::Decimal(_) => OutType::Decimal(if let Ok(res) = val.parse() {
                        res
                    } else {
                        return Err(format!("{} is not a number", val));
                    }),
                };

                table[i].push(out_type);
            }
        }

        Ok(Self { table, names })
    }
}

pub fn parse_bool(val: String) -> Result<Vec<bool>, String> {
    let mut res = Vec::new();
    for c in val.chars() {
        res.push(match c {
            '0' => false,
            '1' => true,
            _ => return Err(format!("unexpected cahr '{}' expected '0' or '1'", c)),
        });
    }
    Ok(res)
}
