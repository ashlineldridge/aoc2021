use anyhow::{bail, Context, Result};
use lazy_static::lazy_static;
use std::{collections::{HashMap, HashSet}, fmt::Display, io::{self, Read}, str::FromStr};
use std::hash::{Hash, Hasher};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let entries = read_entries(&input)?;

    part1(&entries)?;
    part2(&entries)?;

    Ok(())
}

fn part1(entries: &[Entry]) -> Result<()> {
    let looking_for = vec![1, 4, 7, 8].into_iter().collect::<HashSet<u32>>();

    let mut count = 0;
    for entry in entries {
        let decoder = Decoder::build(&entry.samples)?;
        for pattern in &entry.outputs {
            let digit = decoder.decode(pattern)?;
            if looking_for.contains(&digit) {
                count += 1;
            }
        }
    }

    println!("Part 1 answer: {}", count);

    Ok(())
}

fn part2(entries: &[Entry]) -> Result<()> {
    let mut sum = 0;
    for entry in entries {
        let decoder = Decoder::build(&entry.samples)?;
        let mut output = 0;
        for pattern in &entry.outputs {
            let digit = decoder.decode(pattern)?;
            output = output * 10 + digit;
        }

        sum += output;
    }

    println!("Part 2 answer: {}", sum);

    Ok(())
}

fn read_entries(input: &str) -> Result<Vec<Entry>> {
    input.lines().map(|line| line.parse()).collect()
}

type Segment = char;
type Digit = u32;
type SegmentMapping = HashMap<Segment, Segment>;
type DivergentSegmentMapping = HashMap<Segment, SegmentSet>;

#[derive(Clone, Eq)]
pub struct SegmentSet(HashSet<Segment>);

impl SegmentSet {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn as_hashset(&self) -> &HashSet<Segment> {
        &self.0
    }
}

impl FromStr for SegmentSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments = s.chars().collect();
        Ok(SegmentSet(segments))
    }
}

impl Display for SegmentSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut v = self.0.iter().collect::<Vec<_>>();
        v.sort_unstable();

        f.write_str(&String::from_iter(v.into_iter()))
    }
}

impl PartialEq for SegmentSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hash for SegmentSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

lazy_static! {
    // All possible segments.
    static ref ALL_SEGMENTS: HashSet<Segment> = "abcdefg"
        .chars().collect();

    // Internal (non-pub) vector of pattern-digit pairs ordered by pattern
    // length. This structure is used for building the following structures.
    static ref SEGMENT_SET_DIGIT_PAIRS: Vec<(SegmentSet, Digit)> = vec![
        ("cf", 1),
        ("acf", 7),
        ("bcdf", 4),
        ("acdeg", 2),
        ("acdfg", 3),
        ("abdfg", 5),
        ("abcefg", 0),
        ("abdefg", 6),
        ("abcdfg", 9),
        ("abcdefg", 8),
    ].into_iter().map(|(p, d)| (SegmentSet::from_str(p).unwrap(), d)).collect();

    // Map of digits keyed by pattern.
    pub static ref DIGITS_BY_SEGMENT_SET: HashMap<SegmentSet, Digit> = SEGMENT_SET_DIGIT_PAIRS
        .iter()
        .cloned()
        .collect();

    // Map of patterns keyed by their length.
    pub static ref SEGMENT_SETS_BY_LENGTH: HashMap<usize, Vec<SegmentSet>> = SEGMENT_SET_DIGIT_PAIRS
        .iter()
        .fold(HashMap::new(), |mut m, (ss, _)| {
            let patterns = m.entry(ss.len()).or_default();
            patterns.push(ss.clone());
            m
        });
}

struct Decoder {
    mapping: SegmentMapping,
}

impl Decoder {

    /// Building the decoder works by reducing a map of each incorrectly wired segment to all
    /// possible segments (i.e., 1-to-many) to map of each incorrectly wired segment to the
    /// correct corresponding segment (i.e., 1-to-1).
    fn build(samples: &[SegmentSet]) -> Result<Decoder> {
        // Create a vector of samples ordered by ascending length.
        let mut samples = samples.to_vec();
        samples.sort_unstable_by_key(|ss| ss.len());

        // Mapping of input segment to possible output segments. We'll aim to reduce the
        // set of possible output segments to one for each input in the loop below.
        let mut dsm: DivergentSegmentMapping = ALL_SEGMENTS
            .iter()
            .map(|s| (*s, SegmentSet(ALL_SEGMENTS.clone())))
            .collect();

        for sample in &samples {
            let possible_matches = SEGMENT_SETS_BY_LENGTH
                .get(&sample.len())
                .context(format!("invalid pattern: {}", sample))?;

            for possible_match in possible_matches {
                let mut dsm_clone = dsm.clone();

                for sample_segment in sample.as_hashset() {
                    let v = dsm_clone
                        .get(sample_segment)
                        .context(format!("invalid segment: {}", sample_segment))?;

                    let reduced_segment_set =
                        SegmentSet(possible_match.as_hashset().intersection(v.as_hashset()).cloned().collect());

                    dsm_clone.insert(*sample_segment, reduced_segment_set);
                }

                if Self::is_valid(&dsm_clone) {
                    dsm = dsm_clone;
                    break;
                }
            }

            Self::reduce(&mut dsm);

            if let Some(mapping) = Self::try_converge(&dsm) {
                return Ok(Decoder { mapping });
            }
        }

        bail!("could not converge signal patterns");
    }

    fn try_converge(dsm: &DivergentSegmentMapping) -> Option<SegmentMapping> {
        let mut mapping = SegmentMapping::new();
        for (s, ss) in dsm {
            if ss.len() != 1 {
                return None;
            }
            mapping.insert(*s, *ss.as_hashset().iter().next().unwrap());
        }

        Some(mapping)
    }

    fn reduce(dsm: &mut DivergentSegmentMapping) {
        while Self::reduce_once(dsm) {}
    }

    fn reduce_once(dsm: &mut DivergentSegmentMapping) -> bool {
        let mut semi_converged: HashSet<SegmentSet> = HashSet::new();
        for possible_matches in dsm.values() {
            let count = dsm.values().filter(|v| *v == possible_matches).count();
            if count == possible_matches.len() {
                semi_converged.insert(possible_matches.clone());
            }
        }

        let mut modified = false;
        for possible_matches in dsm.values_mut() {
            if semi_converged.contains(possible_matches) {
                continue;
            }

            for semi_converged_set in &semi_converged {
                let reduced_matches = SegmentSet(possible_matches
                    .as_hashset()
                    .difference(semi_converged_set.as_hashset())
                    .cloned()
                    .collect());

                if *possible_matches != reduced_matches {
                    *possible_matches = reduced_matches;
                    modified = true;
                }
            }
        }

        modified
    }

    fn is_valid(dsm: &DivergentSegmentMapping) -> bool {
        for possible_matches in dsm.values() {
            if possible_matches.is_empty() {
                return false;
            }

            if possible_matches.len() == 1 {
                let count = dsm.values().filter(|v| *v == possible_matches).count();
                if count > 1 {
                    return false;
                }
            }
        }

        true
    }

    fn decode(&self, encoded_segment_set: &SegmentSet) -> Result<Digit> {
        let mut decoded_segment_set = HashSet::new();
        for segment in encoded_segment_set.as_hashset() {
            let decoded_segment = self
                .mapping
                .get(segment)
                .context(format!("invalid segment: {}", segment))?;

            decoded_segment_set.insert(*decoded_segment);
        }

        let decoded_segment_set = SegmentSet(decoded_segment_set);

        DIGITS_BY_SEGMENT_SET
            .get(&decoded_segment_set)
            .cloned()
            .context(format!("could not decode pattern: {}", encoded_segment_set))
    }
}

struct Entry {
    samples: Vec<SegmentSet>,
    outputs: Vec<SegmentSet>,
}

impl FromStr for Entry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (inputs, outputs) = s.split_once(" | ").context("bad input")?;
        let samples = inputs.split_terminator(' ').map(SegmentSet::from_str).collect::<Result<_>>()?;
        let outputs = outputs.split_terminator(' ').map(SegmentSet::from_str).collect::<Result<_>>()?;

        Ok(Entry { samples, outputs })
    }
}
