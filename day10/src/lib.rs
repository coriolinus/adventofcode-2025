mod snoob;

use aoclib::parse;
use color_eyre::{
    eyre::{bail, eyre, Context, OptionExt, Report},
    Result,
};
use lazy_regex::{regex_captures, regex_captures_iter};
use snoob::PermutationIterator;
use std::{path::Path, str::FromStr};

type LightState = u32;

struct Machine {
    n_indicator_lights: u32,
    target_indicator_state: LightState,
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
        let n_indicator_lights = indicator_lights_in.len() as _;
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
            target_indicator_state: indicator_lights,
            buttons,
            joltage_requirements,
        })
    }
}

impl Machine {
    // this is only useful for debugging
    #[allow(dead_code)]
    fn summarize(&self) -> String {
        format!(
            "light state: {state:0width$b}\n{n_buttons} buttons\n{n_joltage} joltage requirements\n",
            state = self.target_indicator_state,
            width = self.n_indicator_lights as _,
            n_buttons = self.buttons.len(),
            n_joltage = self.joltage_requirements.len(),
        )
    }

    /// Compute the light state after pressing the selected buttons
    ///
    /// (there is never any point in pressing any button more than once, as two pushes are a noop)
    fn compute_light_state_for_buttons(&self, button_presses: LightState) -> LightState {
        let mut state = 0;
        for (idx, button) in self.buttons.iter().copied().enumerate() {
            if button_presses & 1 << idx != 0 {
                state ^= button;
            }
        }
        state
    }
}

pub fn part1(input: &Path) -> Result<()> {
    let total_presses = parse::<Machine>(input)?
        .enumerate()
        .map(|(idx, machine)| {
            PermutationIterator::new(machine.n_indicator_lights)
                .ok_or_else(|| {
                    eyre!(
                        "machine {idx} of width {} could not construct permutation iterator",
                        machine.n_indicator_lights
                    )
                })?
                .find_map(|button_presses| {
                    (machine.compute_light_state_for_buttons(button_presses)
                        == machine.target_indicator_state)
                        .then(|| button_presses.count_ones())
                })
                .ok_or_eyre(format!("no combination of buttons turned on machine {idx}"))
        })
        .sum::<Result<u32, _>>()?;

    println!("total button presses to activate machines: {total_presses}");
    Ok(())
}

pub fn part2(input: &Path) -> Result<()> {
    unimplemented!("input file: {:?}", input)
}
