#[derive(Debug, Clone)]
pub struct HexagramData {
    pub gate: u8,
    pub binary: u8,
    pub name: &'static str,
    pub description: &'static str,
    pub character: char, 
}

// --- 1. THE RAVE MANDALA ORDER (Counter-Clockwise from Gate 41) ---
pub const RAVE_MANDALA: [u8; 64] = [
    41, 19, 13, 49, 30, 55, 37, 63, 22, 36, 25, 17, 21, 51, 42, 3, 
    27, 24, 2, 23, 8, 20, 16, 35, 45, 12, 15, 52, 39, 53, 62, 56, 
    31, 33, 7, 4, 29, 59, 40, 64, 47, 6, 46, 18, 48, 57, 32, 50, 28, 
    44, 1, 43, 14, 34, 9, 5, 26, 11, 10, 58, 38, 54, 61, 60 
];

// --- 2. BINARY LOOKUP (Gate # -> Binary 0-63) ---
pub fn gate_to_binary(gate: u8) -> u8 {
    match gate {
        1 => 63, 2 => 0, 3 => 17, 4 => 34, 5 => 23, 6 => 58, 7 => 2, 8 => 16,
        9 => 55, 10 => 59, 11 => 7, 12 => 3, 13 => 61, 14 => 60, 15 => 8, 16 => 12,
        17 => 25, 18 => 38, 19 => 33, 20 => 14, 21 => 42, 22 => 37, 23 => 4, 24 => 1,
        25 => 51, 26 => 39, 27 => 19, 28 => 30, 29 => 18, 30 => 41, 31 => 28, 32 => 26,
        33 => 6, 34 => 54, 35 => 10, 36 => 40, 37 => 53, 38 => 56, 39 => 22, 40 => 11,
        41 => 32, 42 => 21, 43 => 31, 44 => 27, 45 => 13, 46 => 29, 47 => 24, 48 => 36,
        49 => 49, 50 => 46, 51 => 43, 52 => 48, 53 => 52, 54 => 45, 55 => 35, 56 => 20,
        57 => 50, 58 => 57, 59 => 44, 60 => 15, 61 => 5, 62 => 9, 63 => 47, 64 => 23,
        _ => 0,
    }
}

// --- 3. THE COMPLETE DICTIONARY ---
pub fn get_hexagram_by_gate(gate: u8) -> HexagramData {
    let binary = gate_to_binary(gate);
    
    let (name, desc, char) = match gate {
        1 => ("The Creative", "Force. The Primal Power.", '䷀'),
        2 => ("The Receptive", "Flux. The Primal Field.", '䷁'),
        3 => ("Difficulty at Beginning", "Ordering. Sprouting.", '䷂'),
        4 => ("Youthful Folly", "Formulization. The Answer.", '䷃'),
        5 => ("Waiting", "Fixed Rhythms. Patience.", '䷄'),
        6 => ("Conflict", "Friction. The Diplomat.", '䷅'),
        7 => ("The Army", "The Role of the Self.", '䷆'),
        8 => ("Holding Together", "Contribution. Solidarity.", '䷇'),
        9 => ("Taming Power of Small", "Focus. Applied Details.", '䷈'),
        10 => ("Treading", "Behavior. The Self.", '䷉'),
        11 => ("Peace", "Ideas. Harmony.", '䷊'),
        12 => ("Standstill", "Caution. The Stop.", '䷋'),
        13 => ("Fellowship", "The Listener. Secrets.", '䷌'),
        14 => ("Possession in Great Measure", "Power Skills. Wealth.", '䷍'),
        15 => ("Modesty", "Extremes. The Love of Humanity.", '䷎'),
        16 => ("Enthusiasm", "Skills. Identification.", '䷏'),
        17 => ("Following", "Opinions. The Eye.", '䷐'),
        18 => ("Work on What Has Been Spoiled", "Correction. Integrity.", '䷑'),
        19 => ("Approach", "Wanting. Fuel of Sensitivity.", '䷒'),
        20 => ("Contemplation", "The Now. Presence.", '䷓'),
        21 => ("Biting Through", "Control. The Hunter.", '䷔'),
        22 => ("Grace", "Openness. Social Charm.", '䷕'),
        23 => ("Splitting Apart", "Assimilation. Explanation.", '䷖'),
        24 => ("Return", "Rationalization. The Reset.", '䷗'),
        25 => ("Innocence", "Spirit of Self. Universal Love.", '䷘'),
        26 => ("Taming Power of Great", "The Egoist. Sales.", '䷙'),
        27 => ("Nourishment", "Caring. Preservation.", '䷚'),
        28 => ("Preponderance of Great", "The Game Player. Risk.", '䷛'),
        29 => ("The Abyssal", "Perseverance. Saying Yes.", '䷜'),
        30 => ("The Clinging", "Feelings. The Fates.", '䷝'),
        31 => ("Influence", "Leading. The Alpha.", '䷞'),
        32 => ("Duration", "Continuity. The Conservative.", '䷟'),
        33 => ("Retreat", "Privacy. The Witness.", '䷠'),
        34 => ("Power of the Great", "Power. The Display.", '䷡'),
        35 => ("Progress", "Change. The Experience.", '䷢'),
        36 => ("Darkening of the Light", "Crisis. Inexperience.", '䷣'),
        37 => ("The Family", "Friendship. The Bargain.", '䷤'),
        38 => ("Opposition", "The Fighter. Struggle.", '䷥'),
        39 => ("Obstruction", "Provocation. The Tease.", '䷦'),
        40 => ("Deliverance", "Aloneness. Liberation.", '䷧'),
        41 => ("Decrease", "Contraction. Fantasy.", '䷨'),
        42 => ("Increase", "Growth. Conclusion.", '䷩'),
        43 => ("Break-through", "Insight. The Inner Voice.", '䷪'),
        44 => ("Coming to Meet", "Alertness. Patterns.", '䷫'),
        45 => ("Gathering Together", "The Gatherer. Royalty.", '䷬'),
        46 => ("Pushing Upward", "Determination. The Body.", '䷭'),
        47 => ("Oppression", "Realization. The Abstract.", '䷮'),
        48 => ("The Well", "Depth. The Solution.", '䷯'),
        49 => ("Revolution", "Principles. Rejection.", '䷰'),
        50 => ("The Cauldron", "Values. The Guardian.", '䷱'),
        51 => ("The Arousing", "Shock. Initiation.", '䷲'),
        52 => ("Keeping Still", "Stillness. Focus.", '䷳'),
        53 => ("Development", "Beginnings. Pressure.", '䷴'),
        54 => ("The Marrying Maiden", "Ambition. The Drive.", '䷵'),
        55 => ("Abundance", "Spirit. Melancholy.", '䷶'),
        56 => ("The Wanderer", "Stimulation. The Story.", '䷷'),
        57 => ("The Gentle", "Intuition. The Spleen.", '䷸'),
        58 => ("The Joyous", "Vitality. The Joy of Life.", '䷹'),
        59 => ("Dispersion", "Sexuality. Breaking Barriers.", '䷺'),
        60 => ("Limitation", "Acceptance. Mutation.", '䷻'),
        61 => ("Inner Truth", "Mystery. Inner Thinking.", '䷼'),
        62 => ("Preponderance of Small", "Details. Facts.", '䷽'),
        63 => ("After Completion", "Doubts. Logic.", '䷾'),
        64 => ("Before Completion", "Confusion. Abstraction.", '䷿'),
        _ => ("Unknown", "Void.", '?'),
    };

    HexagramData { gate, binary, name, description: desc, character: char }
}

pub fn get_opposite_gate(gate: u8) -> u8 {
    if let Some(idx) = RAVE_MANDALA.iter().position(|&g| g == gate) {
        let opposite_idx = (idx + 32) % 64;
        RAVE_MANDALA[opposite_idx]
    } else {
        0
    }
}
