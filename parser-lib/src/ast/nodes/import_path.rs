// SPDX-License-Identifier: Apache-2.0
use crate::{
    ast::{
        Identifier, ImportPath, SourceBuffer,
        derive::Derive,
        prettyprint::{PrettyPrintable, printout_accumulator::PrintoutAccumulator},
    },
    grammar::Rule,
};

impl Derive for ImportPath {
    fn from_parse_tree(p: pest::iterators::Pair<'_, Rule>, source: &SourceBuffer) -> Self {
        assert!(p.as_rule() == Rule::import_path);
        let loc = From::from(&p.as_span());
        let inner = p.into_inner();
        let entries:Vec<Identifier> = inner
            .map(|x| Identifier::from_parse_tree(x, source))
            .collect();
        let name = entries
            .iter()
            .map(|x| x.value.clone())
            .collect::<Vec<_>>()
            .join(".");
        Self {
            loc: source.pointer(loc),
            entries,
            name,
        }
    }
}

impl PrettyPrintable for ImportPath {
    fn prettyprint(&self, buffer: PrintoutAccumulator) -> PrintoutAccumulator {
        buffer.write_separated_list(&self.entries, ".")
    }
}
