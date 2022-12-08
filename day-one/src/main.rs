#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate prettytable;

use pest::Parser;

#[derive(Parser)]
#[grammar = "./list.pest"]
struct ListParser;

fn main() {
    let list = ListParser::parse(Rule::list, include_str!("./list.txt"))
        .unwrap_or_else(|e| panic!("{}", e));

    let mut elfs = list
        .map(|elf| {
            elf.into_inner()
                .map(|meal| meal.as_str().parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

    elfs.sort_by(|a, b| b.cmp(a));
    table!(
        ["challenge", "kcal"],
        ["most", elfs.first().unwrap()],
        ["top 3", &elfs[1..3].iter().sum::<i32>()]
    )
    .printstd();
}
