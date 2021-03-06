#[macro_use]
extern crate itertools;

use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum ItemType {
    Generator,
    Microchip,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
struct Item {
    what: ItemType,
    compatible: &'static str,
    floor: u32,
}

impl Item {
    fn new(what: ItemType, compatible: &'static str, floor: u32) -> Item {
        Item {
            what: what,
            compatible: compatible,
            floor: floor,
        }
    }
}

fn valid_floor(items: &[&Item]) -> bool {
    let generators = items.iter()
        .filter(|&item| item.what == ItemType::Generator)
        .map(|item| item.compatible)
        .collect::<HashSet<_>>();
    if generators.is_empty() {
        return true;
    }

    let chips = items.iter()
        .filter(|&item| item.what == ItemType::Microchip)
        .map(|item| item.compatible)
        .collect::<HashSet<_>>();
    if chips.is_empty() {
        return true;
    }

    chips.is_subset(&generators)
}

fn valid_layout(items: &Vec<Item>) -> bool {
    [0, 1, 2, 3]
        .iter()
        .all(|&i| valid_floor(&items.iter().filter(|&item| item.floor == i).collect::<Vec<_>>()))
}

fn steps(current_floor: u32, layout: &Vec<Item>) -> Vec<(Vec<Item>, u32)> {
    let mut possible_layouts = Vec::new();

    // items on current floor
    let items = layout.iter()
        .enumerate()
        .filter(|&(_, ref item)| item.floor == current_floor)
        .collect::<Vec<_>>();

    let mut try_single = true;

    // possible double up moves
    if current_floor != 3 {
        for (i1, i2) in items.iter().tuple_combinations() {
            let &(i1, _) = i1;
            let &(i2, _) = i2;
            let new = layout.iter()
                .enumerate()
                .map(|(j, ref item)| if j == i1 || j == i2 {
                    Item::new(item.what, item.compatible, item.floor + 1)
                } else {
                    Item::new(item.what, item.compatible, item.floor)
                })
                .collect::<Vec<Item>>();
            if valid_layout(&new) {
                try_single = false;
                possible_layouts.push((new, current_floor + 1));
            }
        }
    }

    let items_on_0 = layout.iter().filter(|&item| item.floor == 0).count() != 0;
    let items_on_1 = layout.iter().filter(|&item| item.floor == 1).count() != 0;
    let try_down = current_floor != 0 || current_floor == 1 && items_on_0 || current_floor == 2 && (items_on_0 || items_on_1);

    for &(i, _) in &items {
        // down
        if try_down {
            let new = layout.iter()
                .enumerate()
                .map(|(j, ref item)| if i == j {
                    Item::new(item.what, item.compatible, item.floor - 1)
                } else {
                    Item::new(item.what, item.compatible, item.floor)
                })
                .collect::<Vec<Item>>();
            if valid_layout(&new) {
                possible_layouts.push((new, current_floor - 1));
            }
        }

        // only move one thing up if could not move double items
        if current_floor != 3 && try_single {
            let new = layout.iter()
                .enumerate()
                .map(|(j, ref item)| if i == j {
                    Item::new(item.what, item.compatible, item.floor + 1)
                } else {
                    Item::new(item.what, item.compatible, item.floor)
                })
                .collect::<Vec<Item>>();
            if valid_layout(&new) {
                possible_layouts.push((new, current_floor + 1));
            }
        }
    }



    possible_layouts
}

fn solve(start_layout: Vec<Item>) {
    //let mut layouts = steps(0, &start_layout);
    let mut layouts = vec![(start_layout.clone(), 0)];
    let mut cache: HashSet<(Vec<Item>, u32)> = HashSet::new();
    cache.insert((start_layout, 0));
    let mut stepcount = 0;
    let done = |items: &Vec<Item>| items.iter().all(|i| i.floor == 3);
    loop {
        stepcount += 1;
        let mut next = Vec::new();
        for (layout, floor) in layouts {
            let new_steps = steps(floor, &layout);
            for new_layout in new_steps {
                if done(&new_layout.0) {
                    println!("result: {}", stepcount);
                    return;
                }
                if !cache.contains(&new_layout) {
                    cache.insert(new_layout.clone());
                    next.push(new_layout);
                }
            }
        }
        layouts = next;
    }
}

fn main() {
    use ItemType::*;

    let part1_layout = vec![Item::new(Generator, "polonium", 0),
                            Item::new(Generator, "thulium", 0),
                            Item::new(Microchip, "thulium", 0),
                            Item::new(Generator, "promethium", 0),
                            Item::new(Generator, "ruthenium", 0),
                            Item::new(Microchip, "ruthenium", 0),
                            Item::new(Generator, "cobalt", 0),
                            Item::new(Microchip, "cobalt", 0),
                            Item::new(Microchip, "polonium", 1),
                            Item::new(Microchip, "promethium", 1)];

    let part2_layout = vec![Item::new(Generator, "polonium", 0),
                            Item::new(Generator, "thulium", 0),
                            Item::new(Microchip, "thulium", 0),
                            Item::new(Generator, "promethium", 0),
                            Item::new(Generator, "ruthenium", 0),
                            Item::new(Microchip, "ruthenium", 0),
                            Item::new(Generator, "cobalt", 0),
                            Item::new(Microchip, "cobalt", 0),
                            Item::new(Microchip, "elerium", 0),
                            Item::new(Generator, "elerium", 0),
                            Item::new(Microchip, "dilithium", 0),
                            Item::new(Generator, "dilithium", 0),
                            Item::new(Microchip, "polonium", 1),
                            Item::new(Microchip, "promethium", 1)];
    solve(part1_layout);
    solve(part2_layout);
}
