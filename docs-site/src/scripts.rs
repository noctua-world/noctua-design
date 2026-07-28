//! The fourteen scripts, and the sample each is rendered with.
//!
//! `NOCTUA.md §7.3.1` is the reason Noto was chosen over anything else. This
//! table is what turns that reason into something a test can check: every one of
//! these renders, in the default configuration, with no tofu.
//!
//! The samples are real sentences rather than lorem: a script's shaping only
//! shows in real text. Arabic needs joined forms, Devanagari needs conjuncts,
//! Thai needs the absence of spaces to matter.

/// One script, its sample, and what it proves.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Sample {
    /// The script's English name.
    pub name: &'static str,
    /// The BCP 47 tag for `lang`, so the browser shapes and hyphenates correctly.
    pub lang: &'static str,
    /// `rtl` where the script runs right to left.
    pub dir: &'static str,
    /// A real sentence in the script.
    pub text: &'static str,
    /// What this particular sample exercises, for the reader of the page.
    pub proves: &'static str,
    /// Whether Noctua Iosevka can set it — the fallback must fire when it cannot.
    pub monospaceable: bool,
}

/// Every script the platform must render.
pub(crate) const SAMPLES: [Sample; 14] = [
    Sample {
        name: "Latin",
        lang: "en",
        dir: "ltr",
        text: "The quick brown fox jumps over the lazy dog.",
        proves: "the base case, and every letter of the alphabet",
        monospaceable: true,
    },
    Sample {
        name: "Latin, Portuguese",
        lang: "pt-BR",
        dir: "ltr",
        text: "Uma ação em português: coração, ambiguïdade, José até vovô.",
        proves: "stacked diacritics — the tilde, cedilla, circumflex and acute this project needs",
        monospaceable: true,
    },
    Sample {
        name: "Greek",
        lang: "el",
        dir: "ltr",
        text: "Η γρήγορη καφέ αλεπού πηδάει πάνω από το τεμπέλικο σκυλί.",
        proves: "Greek, which Iosevka does cover — the fallback must NOT fire",
        monospaceable: true,
    },
    Sample {
        name: "Cyrillic",
        lang: "ru",
        dir: "ltr",
        text: "Съешь же ещё этих мягких французских булок, да выпей чаю.",
        proves: "Cyrillic, likewise covered — the fallback must not fire",
        monospaceable: true,
    },
    Sample {
        name: "Arabic",
        lang: "ar",
        dir: "rtl",
        text: "نص حكيم له سر قاطع وذو شأن عظيم مكتوب على ثوب أخضر ومغلف بجلد أزرق.",
        proves: "right-to-left shaping, and initial/medial/final letter forms",
        monospaceable: false,
    },
    Sample {
        name: "Hebrew",
        lang: "he",
        dir: "rtl",
        text: "דג סקרן שט בים מאוכזב ולפתע מצא חברה.",
        proves: "right-to-left without joining forms",
        monospaceable: false,
    },
    Sample {
        name: "Devanagari",
        lang: "hi",
        dir: "ltr",
        text: "ऋषियों को सताने वाले दुष्ट राक्षसों के राजा रावण का सर्वनाश।",
        proves: "conjunct consonants and marks above and below the baseline",
        monospaceable: false,
    },
    Sample {
        name: "Thai",
        lang: "th",
        dir: "ltr",
        text: "เป็นมนุษย์สุดประเสริฐเลิศคุณค่ากว่าบรรดาฝูงสัตว์เดรัจฉาน",
        proves: "a script with no spaces, where line breaking is the font's problem",
        monospaceable: false,
    },
    Sample {
        name: "Ethiopic",
        lang: "am",
        dir: "ltr",
        text: "ሰማይ አይታረስ ንጉሥ አይከሰስ።",
        proves: "a syllabary with no case",
        monospaceable: false,
    },
    Sample {
        name: "Georgian",
        lang: "ka",
        dir: "ltr",
        text: "ვეპხის ტყაოსანი შოთა რუსთაველის პოემაა.",
        proves: "a unicameral alphabet with tall ascenders and deep descenders",
        monospaceable: false,
    },
    Sample {
        name: "Armenian",
        lang: "hy",
        dir: "ltr",
        text: "Բոլոր մարդիկ ծնվում են ազատ և հավասար իրենց արժանապատվությամբ։",
        proves: "its own punctuation, which is not Latin punctuation",
        monospaceable: false,
    },
    Sample {
        name: "Japanese",
        lang: "ja",
        dir: "ltr",
        text: "いろはにほへと ちりぬるを 和歌 東京都は日本の首都です。",
        proves: "three writing systems on one line — hiragana, katakana and kanji",
        monospaceable: false,
    },
    Sample {
        name: "Korean",
        lang: "ko",
        dir: "ltr",
        text: "다람쥐 헌 쳇바퀴에 타고파. 한국어는 한글로 씁니다.",
        proves: "composed syllable blocks",
        monospaceable: false,
    },
    Sample {
        name: "Simplified Chinese",
        lang: "zh-Hans",
        dir: "ltr",
        text: "天地玄黄，宇宙洪荒。中文是世界上使用人数最多的语言。",
        proves: "dense ideographs, and full-width punctuation",
        monospaceable: false,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    /// The utility every text component uses must agree with this table, or the
    /// page documents one behaviour and the components implement another.
    #[test]
    fn the_table_agrees_with_the_script_utility() {
        for sample in SAMPLES {
            let expected = if sample.monospaceable {
                noctua_design::Script::Monospaceable
            } else {
                noctua_design::Script::Proportional
            };
            assert_eq!(
                noctua_design::dominant_script(sample.text),
                expected,
                "{} is classified differently by the utility than by this table",
                sample.name
            );
        }
    }

    /// Three of the fourteen are the ones Iosevka covers. If that count changes,
    /// the fallback's whole premise has changed with it.
    #[test]
    fn exactly_four_samples_are_monospaceable() {
        // Latin, Portuguese Latin, Greek, Cyrillic.
        assert_eq!(SAMPLES.iter().filter(|s| s.monospaceable).count(), 4);
    }

    #[test]
    fn every_right_to_left_sample_says_so() {
        for sample in SAMPLES {
            let rtl = sample.lang == "ar" || sample.lang == "he";
            assert_eq!(sample.dir == "rtl", rtl, "{}", sample.name);
        }
    }
}
