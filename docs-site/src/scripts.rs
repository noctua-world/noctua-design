//! The fourteen scripts, and the sample each is rendered with.
//!
//! `NOCTUA.md §7.3.1` is the reason Noto was chosen over anything else. This
//! table is what turns that reason into something a test can check: every one of
//! these renders, in the default configuration, with no tofu.
//!
//! The samples are real sentences rather than lorem: a script's shaping only
//! shows in real text. Arabic needs joined forms, Devanagari needs conjuncts,
//! Thai needs the absence of spaces to matter.

use noctua_design_i18n::Key;

/// One script, its sample, and what it proves.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Sample {
    /// A stable name for the DOM and for a test, in every locale.
    ///
    /// The markup used to carry the English name, so a spec selecting on it
    /// passed in `/en/` and matched nothing in `/pt-BR/`.
    pub id: &'static str,
    /// The script's name, as a message. "Greek" is a word in English and
    /// "Grego" is the same word in Portuguese; neither is an identifier.
    pub name: Key,
    /// The BCP 47 tag for `lang`, so the browser shapes and hyphenates correctly.
    pub lang: &'static str,
    /// `rtl` where the script runs right to left.
    pub dir: &'static str,
    /// A real sentence in the script.
    pub text: &'static str,
    /// What this particular sample exercises, for the reader of the page.
    pub proves: Key,
    /// Whether Noctua Iosevka can set it — the fallback must fire when it cannot.
    pub monospaceable: bool,

    /// Whether its glyphs are one em wide, like the browser's tofu box.
    ///
    /// Declared rather than pattern-matched. The browser suite used to exempt
    /// CJK from its width check with a regex over the script's ENGLISH NAME —
    /// so the exemption evaporated the moment the name became a message, and
    /// the suite called a correctly rendered Chinese line tofu. A property of
    /// a writing system belongs in the table that describes it.
    pub ideographic: bool,
}

/// Every script the platform must render.
pub(crate) const SAMPLES: [Sample; 14] = [
    Sample {
        id: "latin",
        name: Key::TypeScriptLatinName,
        lang: "en",
        dir: "ltr",
        text: "The quick brown fox jumps over the lazy dog.",
        proves: Key::TypeScriptLatinProves,
        monospaceable: true,
        ideographic: false,
    },
    Sample {
        id: "latin-portuguese",
        name: Key::TypeScriptLatinPortugueseName,
        lang: "pt-BR",
        dir: "ltr",
        text: "Uma ação em português: coração, ambiguïdade, José até vovô.",
        proves: Key::TypeScriptLatinPortugueseProves,
        monospaceable: true,
        ideographic: false,
    },
    Sample {
        id: "greek",
        name: Key::TypeScriptGreekName,
        lang: "el",
        dir: "ltr",
        text: "Η γρήγορη καφέ αλεπού πηδάει πάνω από το τεμπέλικο σκυλί.",
        proves: Key::TypeScriptGreekProves,
        monospaceable: true,
        ideographic: false,
    },
    Sample {
        id: "cyrillic",
        name: Key::TypeScriptCyrillicName,
        lang: "ru",
        dir: "ltr",
        text: "Съешь же ещё этих мягких французских булок, да выпей чаю.",
        proves: Key::TypeScriptCyrillicProves,
        monospaceable: true,
        ideographic: false,
    },
    Sample {
        id: "arabic",
        name: Key::TypeScriptArabicName,
        lang: "ar",
        dir: "rtl",
        text: "نص حكيم له سر قاطع وذو شأن عظيم مكتوب على ثوب أخضر ومغلف بجلد أزرق.",
        proves: Key::TypeScriptArabicProves,
        monospaceable: false,
        ideographic: false,
    },
    Sample {
        id: "hebrew",
        name: Key::TypeScriptHebrewName,
        lang: "he",
        dir: "rtl",
        text: "דג סקרן שט בים מאוכזב ולפתע מצא חברה.",
        proves: Key::TypeScriptHebrewProves,
        monospaceable: false,
        ideographic: false,
    },
    Sample {
        id: "devanagari",
        name: Key::TypeScriptDevanagariName,
        lang: "hi",
        dir: "ltr",
        text: "ऋषियों को सताने वाले दुष्ट राक्षसों के राजा रावण का सर्वनाश।",
        proves: Key::TypeScriptDevanagariProves,
        monospaceable: false,
        ideographic: false,
    },
    Sample {
        id: "thai",
        name: Key::TypeScriptThaiName,
        lang: "th",
        dir: "ltr",
        text: "เป็นมนุษย์สุดประเสริฐเลิศคุณค่ากว่าบรรดาฝูงสัตว์เดรัจฉาน",
        proves: Key::TypeScriptThaiProves,
        monospaceable: false,
        ideographic: false,
    },
    Sample {
        id: "ethiopic",
        name: Key::TypeScriptEthiopicName,
        lang: "am",
        dir: "ltr",
        text: "ሰማይ አይታረስ ንጉሥ አይከሰስ።",
        proves: Key::TypeScriptEthiopicProves,
        monospaceable: false,
        ideographic: false,
    },
    Sample {
        id: "georgian",
        name: Key::TypeScriptGeorgianName,
        lang: "ka",
        dir: "ltr",
        text: "ვეპხის ტყაოსანი შოთა რუსთაველის პოემაა.",
        proves: Key::TypeScriptGeorgianProves,
        monospaceable: false,
        ideographic: false,
    },
    Sample {
        id: "armenian",
        name: Key::TypeScriptArmenianName,
        lang: "hy",
        dir: "ltr",
        text: "Բոլոր մարդիկ ծնվում են ազատ և հավասար իրենց արժանապատվությամբ։",
        proves: Key::TypeScriptArmenianProves,
        monospaceable: false,
        ideographic: false,
    },
    Sample {
        id: "japanese",
        name: Key::TypeScriptJapaneseName,
        lang: "ja",
        dir: "ltr",
        text: "いろはにほへと ちりぬるを 和歌 東京都は日本の首都です。",
        proves: Key::TypeScriptJapaneseProves,
        monospaceable: false,
        ideographic: true,
    },
    Sample {
        id: "korean",
        name: Key::TypeScriptKoreanName,
        lang: "ko",
        dir: "ltr",
        text: "다람쥐 헌 쳇바퀴에 타고파. 한국어는 한글로 씁니다.",
        proves: Key::TypeScriptKoreanProves,
        monospaceable: false,
        ideographic: true,
    },
    Sample {
        id: "simplified-chinese",
        name: Key::TypeScriptSimplifiedChineseName,
        lang: "zh-Hans",
        dir: "ltr",
        text: "天地玄黄，宇宙洪荒。中文是世界上使用人数最多的语言。",
        proves: Key::TypeScriptSimplifiedChineseProves,
        monospaceable: false,
        ideographic: true,
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
                sample.id
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
            assert_eq!(sample.dir == "rtl", rtl, "{}", sample.id);
        }
    }
}
