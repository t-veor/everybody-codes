use std::collections::HashMap;

use utils::{grid::OrthoDir, read_input_files};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Action {
    Inc,
    Dec,
    Noop,
}

impl Action {
    fn parse(s: &str) -> Self {
        match s {
            "+" => Action::Inc,
            "-" => Action::Dec,
            "=" | "S" => Action::Noop,
            _ => unreachable!("Unknown action {s:?}"),
        }
    }
}

#[derive(Debug, Clone)]
struct Plan {
    actions: Vec<Action>,
}

impl Plan {
    fn parse(input: &str) -> Self {
        Self {
            actions: input.split(',').map(Action::parse).collect(),
        }
    }

    fn simulate(&self, steps: i64, initial_power: i64) -> i64 {
        let mut power = initial_power;
        let mut total = 0;

        for i in 0..steps {
            match self.actions[i as usize % self.actions.len()] {
                Action::Inc => power += 1,
                Action::Dec => power = (power - 1).max(0),
                Action::Noop => (),
            };

            total += power
        }

        total
    }

    fn all_perms_with_budget(
        inc_budget: usize,
        dec_budget: usize,
        noop_budget: usize,
        mut visitor: impl FnMut(Self),
    ) {
        fn inner(
            plan_so_far: &mut Vec<Action>,
            inc_budget: usize,
            dec_budget: usize,
            noop_budget: usize,
            visitor: &mut impl FnMut(Plan),
        ) {
            if (inc_budget, dec_budget, noop_budget) == (0, 0, 0) {
                return visitor(Plan {
                    actions: plan_so_far.clone(),
                });
            }

            if inc_budget > 0 {
                plan_so_far.push(Action::Inc);
                inner(
                    plan_so_far,
                    inc_budget - 1,
                    dec_budget,
                    noop_budget,
                    visitor,
                );
                plan_so_far.pop();
            }

            if dec_budget > 0 {
                plan_so_far.push(Action::Dec);
                inner(
                    plan_so_far,
                    inc_budget,
                    dec_budget - 1,
                    noop_budget,
                    visitor,
                );
                plan_so_far.pop();
            }

            if noop_budget > 0 {
                plan_so_far.push(Action::Noop);
                inner(
                    plan_so_far,
                    inc_budget,
                    dec_budget,
                    noop_budget - 1,
                    visitor,
                );
                plan_so_far.pop();
            }
        }

        inner(
            &mut Vec::new(),
            inc_budget,
            dec_budget,
            noop_budget,
            &mut visitor,
        );
    }
}

struct Track {
    terrain: Vec<Action>,
}

impl Track {
    fn parse(input: &str) -> Self {
        let grid: HashMap<(isize, isize), char> = input
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c != ' ')
                    .map(move |(x, c)| ((x as isize, y as isize), c))
            })
            .collect();
        let mut terrain = Vec::new();

        assert_eq!(grid.get(&(0, 0)), Some(&'S'));

        // Walk around the track.
        let mut coords = (0, 0);
        let mut dir = OrthoDir::East;

        loop {
            // Determine the direction of the next track piece
            dir = dir.rotate_ccw();
            for _ in 0..4 {
                if grid.contains_key(&dir.step(coords)) {
                    break;
                }
                dir = dir.rotate_cw();
            }

            coords = dir.step(coords);

            terrain.push(Action::parse(&grid.get(&coords).unwrap().to_string()));

            if grid.get(&coords) == Some(&'S') {
                break;
            }
        }

        Self { terrain }
    }

    fn get_and_step(
        &self,
        plan: &Plan,
        indices: (usize, usize),
        power: i64,
    ) -> ((usize, usize), i64) {
        let (terrain_idx, plan_idx) = indices;

        let new_power = match (self.terrain[terrain_idx], plan.actions[plan_idx]) {
            (Action::Inc, _) => power + 1,
            (Action::Dec, _) => (power - 1).max(0),
            (Action::Noop, Action::Inc) => power + 1,
            (Action::Noop, Action::Dec) => (power - 1).max(0),
            (Action::Noop, Action::Noop) => power,
        };

        let next_terrain_idx = (terrain_idx + 1) % self.terrain.len();
        let next_plan_idx = (plan_idx + 1) % plan.actions.len();

        ((next_terrain_idx, next_plan_idx), new_power)
    }

    fn simulate(&self, plan: &Plan, rounds: usize, initial_power: i64) -> i64 {
        let mut total = 0;
        let mut power = initial_power;

        let mut indices = (0, 0);

        for _ in 0..rounds {
            for _ in 0..self.terrain.len() {
                (indices, power) = self.get_and_step(plan, indices, power);
                total += power;
            }
        }

        total
    }
}

fn part1(input: &str) -> String {
    let mut plans: Vec<_> = input
        .trim()
        .lines()
        .map(|line| {
            let (name, plan) = line.split_once(':').unwrap();
            (name, Plan::parse(plan))
        })
        .collect();

    plans.sort_by_cached_key(|(_name, plan)| -plan.simulate(10, 10));

    plans
        .iter()
        .map(|(name, _plan)| *name)
        .collect::<Vec<_>>()
        .join("")
}

fn part2(input: &str) -> String {
    let mut plans: Vec<_> = input
        .trim()
        .lines()
        .map(|line| {
            let (name, plan) = line.split_once(':').unwrap();
            (name, Plan::parse(plan))
        })
        .collect();

    let track = Track::parse(
        "
S-=++=-==++=++=-=+=-=+=+=--=-=++=-==++=-+=-=+=-=+=+=++=-+==++=++=-=-=--
-                                                                     -
=                                                                     =
+                                                                     +
=                                                                     +
+                                                                     =
=                                                                     =
-                                                                     -
--==++++==+=+++-=+=-=+=-+-=+-=+-=+=-=+=--=+++=++=+++==++==--=+=++==+++-
",
    );

    plans.sort_by_cached_key(|(_name, plan)| -track.simulate(plan, 10, 10));

    plans
        .iter()
        .map(|(name, _plan)| *name)
        .collect::<Vec<_>>()
        .join("")
}

fn part3(input: &str) -> i64 {
    let rival_plan = Plan::parse(input.trim().strip_prefix("A:").unwrap());
    let track = Track::parse(
        "
S+= +=-== +=++=     =+=+=--=    =-= ++=     +=-  =+=++=-+==+ =++=-=-=--
- + +   + =   =     =      =   == = - -     - =  =         =-=        -
= + + +-- =-= ==-==-= --++ +  == == = +     - =  =    ==++=    =++=-=++
+ + + =     +         =  + + == == ++ =     = =  ==   =   = =++=
= = + + +== +==     =++ == =+=  =  +  +==-=++ =   =++ --= + =
+ ==- = + =   = =+= =   =       ++--          +     =   = = =--= ==++==
=     ==- ==+-- = = = ++= +=--      ==+ ==--= +--+=-= ==- ==   =+=    =
-               = = = =   +  +  ==+ = = +   =        ++    =          -
-               = + + =   +  -  = + = = +   =        +     =          -
--==++++==+=+++-= =-= =-+-=  =+-= =-= =--   +=++=+++==     -=+=++==+++-
",
    );

    let score_to_beat = track.simulate(&rival_plan, 2024, 10);

    let mut winning_plans = 0;
    // This is slow and could be sped up with some form of cycle detection, but
    // geez I don't want to think about doing the maths, especially with the
    // condition that the power can't drop below 0!
    Plan::all_perms_with_budget(5, 3, 3, |plan| {
        let score = track.simulate(&plan, 2024, 10);
        if score > score_to_beat {
            winning_plans += 1;
        }
    });

    winning_plans
}

fn main() {
    let [p1, p2, p3] = read_input_files!();

    println!("{}", part1(&p1));
    println!("{}", part2(&p2));
    println!("{}", part3(&p3));
}
