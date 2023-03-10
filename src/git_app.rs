use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct GitApp {
    pub selected_gits_app: GitApps,
    #[serde(skip)]
    pub choosen: ChoosenTagSymbols,
    #[serde(skip)]
    pub choosen_other: ChoosenTagSymbols,
    pub owner: String,
    pub base_url: String,
    pub repo: String,
}

impl Default for GitApp {
    fn default() -> Self {
        Self {
            selected_gits_app: GitApps::TagDiffWeb,
            choosen: ChoosenTagSymbols::default(),
            choosen_other: ChoosenTagSymbols::default(),
            base_url: "https://github.com".to_owned(),
            owner: "".to_owned(),
            repo: "".to_owned(),
        }
    }
}

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum GitApps {
    TagDiffWeb,
    TagDiffCommand,
}

#[derive(Clone, PartialEq)]
pub struct TagSymbol {
    pub order: u16,
    pub name: String,
    pub symbol: String,
}

impl TagSymbol {
    pub fn new(order: u16, name: &str, symbol: &str) -> TagSymbol {
        TagSymbol {
            order,
            name: name.to_owned(),
            symbol: symbol.to_owned(),
        }
    }
}

#[derive(Default, Clone)]
pub struct ChoosenTagSymbols {
    pub symbols: HashMap<String, TagSymbol>,
}

impl ChoosenTagSymbols {
    pub fn choose(&mut self, choose: TagSymbol) {
        self.symbols.insert(choose.name.clone(), choose);
    }

    pub fn get_choosen_symbols(&mut self) -> Vec<TagSymbol> {
        let mut values: Vec<TagSymbol> = self.symbols.values().cloned().collect();
        values.sort_by(|a, b| a.order.cmp(&b.order));
        values.into_iter().collect()
    }

    pub fn get_choosen_symbol_chain(&self) -> String {
        let mut values: Vec<TagSymbol> = self.symbols.values().cloned().collect();
        values.sort_by(|a, b| a.order.cmp(&b.order));
        let strings: Vec<String> = values.into_iter().map(|tuple| tuple.symbol).collect();
        strings.concat()
    }

    pub fn take_over(&mut self, other: &ChoosenTagSymbols) {
        for symbol in other.clone().get_choosen_symbols() {
            self.symbols.insert(symbol.name.clone(), symbol);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_return_symbols_in_correct_order() {
        let mut chooser = ChoosenTagSymbols::default();
        chooser.choose(TagSymbol::new(1, "prefix", "something"));
        chooser.choose(TagSymbol::new(2, "delitmiter", "/"));
        chooser.choose(TagSymbol::new(3, "version", "someversion"));

        let tag = chooser.get_choosen_symbol_chain();
        assert_eq!(tag, "something/someversion")
    }

    #[test]
    fn should_take_over_symbols() {
        let mut chooser = ChoosenTagSymbols::default();
        chooser.choose(TagSymbol::new(1, "prefix", "something"));
        chooser.choose(TagSymbol::new(2, "delitmiter", "/"));
        chooser.choose(TagSymbol::new(3, "version", "someversion"));

        let mut other_chooser = ChoosenTagSymbols::default();
        other_chooser.take_over(&chooser);
        assert_eq!(
            other_chooser.get_choosen_symbol_chain(),
            chooser.get_choosen_symbol_chain()
        )
    }
}
