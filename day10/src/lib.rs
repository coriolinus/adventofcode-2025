use aoclib::parse;
use color_eyre::{
    eyre::{bail, Context, OptionExt, Report},
    Result,
};
use lazy_regex::{regex_captures, regex_captures_iter};
use std::{path::Path, str::FromStr};

type LightState = u32;

struct Machine {
    n_indicator_lights: usize,
    indicator_lights: LightState,
    buttons: Vec<LightState>,
    joltage_requirements: Vec<u32>,
}

impl FromStr for Machine {
    type Err = Report;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (_all, indicator_lights_in, button_schematics, joltages_in) =
            regex_captures!(r"^\[([.#]*)\] ((?:\([\d,]+\) ?)+) \{([\d,]+)\}$", s,)
                .ok_or_eyre("no match for machine regex")?;

        // indicator lights
        let indicator_lights_in = indicator_lights_in.as_bytes();
        let n_indicator_lights = indicator_lights_in.len();
        let mut indicator_lights = 0;
        for (idx, light) in indicator_lights_in.iter().enumerate() {
            match light {
                b'#' => indicator_lights |= 1 << idx,
                b'.' => (), // noop
                _ => bail!("unexpected indicator light: {}", *light as char),
            }
        }

        // buttons
        let mut buttons = Vec::new();
        for capture in regex_captures_iter!(r"\(([\d,]+)\)", button_schematics) {
            let index_group = capture
                .get(1)
                .expect("this RE cannot fail to get group 1 if it matches at all")
                .as_str();
            let mut button_state = 0;
            for index in index_group.split(',') {
                let index = index
                    .parse::<usize>()
                    .wrap_err(format!("failed to parse index '{index}' as usize"))?;
                button_state |= 1 << index;
            }
            buttons.push(button_state);
        }

        // joltages
        let joltage_requirements = joltages_in
            .split(',')
            .map(|s| s.parse().wrap_err(format!("failed to parse joltage '{s}'")))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            n_indicator_lights,
            indicator_lights,
            buttons,
            joltage_requirements,
        })
    }
}

impl Machine {
    fn summarize(&self) -> String {
        format!(
            "light state: {state:0width$b}\n{n_buttons} buttons\n{n_joltage} joltage requirements\n",
            state = self.indicator_lights,
            width = self.n_indicator_lights,
            n_buttons = self.buttons.len(),
            n_joltage = self.joltage_requirements.len(),
        )
    }
}

pub fn part1(input: &Path) -> Result<()> {
    for machine in parse::<Machine>(input)? {
        println!("{}", machine.summarize());
    }
    todo!("now do something")
}

pub fn part2(input: &Path) -> Result<()> {
    unimplemented!("input file: {:?}", input)
}
