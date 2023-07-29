// AiScriptに移植するためのリファレンス実装として作成したため、意図的に警告レベルをゆるく設定している。
#![warn(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery)]

use std::io::Read;

fn main() {
    println!("Ctrl+Cで終了");
    loop {
        println!("変換対象の文字列:");
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).expect("IOエラー");
        println!("--------------------------------------------------------------------------------------------------------------------------------");
        println!("変換後の文字列:");
        println!("{}", reverse(&buf));
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Status {
    // initial
    FindEmojiDelimiter,
    //
    Accumulate,
}

macro_rules! dprintln {
    ($($tt:tt)*) => {
        #[cfg(debug_assertions)]
        {
            // println!($($tt)*);
        }
    };
}

fn reverse(st: &str) -> String {
    let mut s = Status::FindEmojiDelimiter;
    let mut buf = String::new();
    let mut ret = String::new();

    let mut i = 0;
    'each: while i < st.chars().count() {
        let c = st.chars().nth(i).unwrap();
        dprintln!("{s:?} ({c}, {i}) ... {buf:?}, {ret:?}");

        if c == ':' {
            if s == Status::FindEmojiDelimiter {
                s = Status::Accumulate;
            } else {
                let x = match buf.as_str() {
                    "_a" => Some(("あ", false)),
                    "_i" => Some(("い", false)),
                    "_u" => Some(("う", false)),
                    "_e" => Some(("え", false)),
                    "_o" => Some(("お", false)),
                    "_xa" => Some(("ぁ", false)),
                    "_xi" => Some(("ぃ", false)),
                    "_xu" => Some(("ぅ", false)),
                    "_xe" => Some(("ぇ", false)),
                    "_xo" => Some(("ぉ", false)),
                    "_ka" => Some(("か", true)),
                    "_ki" => Some(("き", true)),
                    "_ku" => Some(("く", true)),
                    "_ke" => Some(("け", true)),
                    "_ko" => Some(("こ", true)),
                    "_sa" => Some(("さ", true)),
                    "_si" => Some(("し", true)),
                    "_su" => Some(("す", true)),
                    "_se" => Some(("せ", true)),
                    "_so" => Some(("そ", true)),
                    "_ta" => Some(("た", true)),
                    "_ti" => Some(("ち", true)),
                    "_tu" => Some(("つ", true)),
                    "_te" => Some(("て", true)),
                    "_to" => Some(("と", true)),
                    "_na" => Some(("な", false)),
                    "_ni" => Some(("に", false)),
                    "_nu" => Some(("ぬ", false)),
                    "_ne" => Some(("ね", false)),
                    "_no" => Some(("の", false)),
                    "_ha" => Some(("は", true)),
                    "_hi" => Some(("ひ", true)),
                    "_hu" => Some(("ふ", true)),
                    "_he" => Some(("へ", true)),
                    "_ho" => Some(("ほ", true)),
                    "_ma" => Some(("ま", false)),
                    "_mi" => Some(("み", false)),
                    "_mu" => Some(("む", false)),
                    "_me" => Some(("め", false)),
                    "_mo" => Some(("も", false)),
                    "_ya" => Some(("や", false)),
                    "_yu" => Some(("ゆ", false)),
                    "_yo" => Some(("よ", false)),
                    "_xya" => Some(("ゃ", false)),
                    "_xyu" => Some(("ゅ", false)),
                    "_xyo" => Some(("ょ", false)),
                    "_xtu" => Some(("っ", false)),
                    "_ra" => Some(("ら", false)),
                    "_ri" => Some(("り", false)),
                    "_ru" => Some(("る", false)),
                    "_re" => Some(("れ", false)),
                    "_ro" => Some(("ろ", false)),
                    "_wa" => Some(("わ", false)),
                    "_wi" => Some(("ゐ", false)),
                    "_we" => Some(("ゑ", false)),
                    "_wo" => Some(("を", false)),
                    "_nn" => Some(("ん", false)),
                    "_bou" => Some(("ー", false)),
                    "_wave_dash" => Some(("～", false)),
                    "touten" => Some(("、", false)),
                    "kuten" => Some(("。", false)),
                    "kigo_dakuten" => Some(("゛", false)),
                    "kigo_handakuten" => Some(("゜", false)),
                    _ => None,
                };

                if let Some((reversed, need_look_behind)) = x {
                    if need_look_behind {
                        // 注: ここでは独立した濁点や半濁点のことを考える必要はない。そのようなルールは下の方によって処理される。
                        let go_handaku = 'dakuten: {
                            const D: &str = ":kigo_dakuten:";

                            let d_iter = D.chars();
                            let d_cp_count = d_iter.clone().count();
                            let ci_trails = st.chars().skip(i + 1).clone().take(d_cp_count);
                            if ci_trails.clone().count() < d_cp_count {
                                // 足りないので濁点は絶対に続いてない
                                ret += reversed;
                                buf = String::new();
                                s = Status::FindEmojiDelimiter;
                                break 'dakuten false;
                            }

                            let trails = ci_trails;
                            let f = |(post_i, post_c)| {
                                dprintln!("`--- {post_c:?} ({post_i}) =? {cc:?}", cc = trails.clone().nth(post_i));
                                trails.clone().nth(post_i).map(|c| c == post_c).unwrap_or(false)
                            };

                            let followed_by_dakuten = d_iter.clone().enumerate().all(f);
                            dprintln!("D: {followed_by_dakuten}");
                            if followed_by_dakuten {
                                // 濁点が続いている
                                ret += match reversed {
                                    "か" => "が",
                                    "き" => "ぎ",
                                    "く" => "ぐ",
                                    "け" => "げ",
                                    "こ" => "ご",
                                    "さ" => "ざ",
                                    "し" => "じ",
                                    "す" => "ず",
                                    "せ" => "ぜ",
                                    "そ" => "ぞ",
                                    "た" => "だ",
                                    "ち" => "ぢ",
                                    "つ" => "づ",
                                    "て" => "で",
                                    "と" => "ど",
                                    "は" => "ば",
                                    "ひ" => "び",
                                    "ふ" => "ぶ",
                                    "へ" => "べ",
                                    "ほ" => "ぼ",
                                    _ => unreachable!(),
                                };
                                buf = String::new();
                                s = Status::FindEmojiDelimiter;
                                // FIXME: skip :kigo_dakuten:
                                i += 1 + d_cp_count;
                                continue 'each;
                            } else {
                                // 全ての文字がマッチしたわけではないので濁点ではない
                                break 'dakuten true;
                            }
                        };

                        'handakuten: {
                            if !go_handaku {
                                break 'handakuten
                            }
                            const H: &str = ":kigo_handakuten:";

                            let ci = st.chars().skip(i);
                            let di = H.chars();
                            let di_count = di.clone().count();
                            let ci_trails = ci.clone().take(di_count);
                            if ci_trails.clone().count() < di_count {
                                // 足りないので濁点ではない
                                ret += reversed;
                                buf = String::new();
                                s = Status::FindEmojiDelimiter;
                                break 'handakuten;
                            }

                            let trails = ci_trails;
                            let f = |(post_i, post_c)| {
                                trails.clone().nth(post_i).expect("precondition failed") == post_c
                            };

                            if di.clone().enumerate().all(f) {
                                // 濁点が続いている
                                match reversed {
                                    "は" => {
                                        ret += "ぱ";
                                    }
                                    "ひ" => {
                                        ret += "ぴ";
                                    }
                                    "ふ" => {
                                        ret += "ぷ";
                                    }
                                    "へ" => {
                                        ret += "ぺ";
                                    },
                                    "ぽ" => {
                                        ret += "ぽ";
                                    }
                                    other => {
                                        ret += other;
                                        ret += "゜";
                                    },
                                };
                                buf = String::new();
                                s = Status::FindEmojiDelimiter;
                                // FIXME: skip :kigo_handakuten:
                                i += 1 + di_count;
                                continue 'each;
                            } else {
                                // 全ての文字がマッチしたわけではないので濁点ではない
                                break 'handakuten;
                            }

                        }
                    } else {
                        ret += reversed;
                        buf = String::new();
                        s = Status::FindEmojiDelimiter;
                    }
                } else {
                    // skip, this is not Japanese emoji
                    ret += ":";
                    ret += buf.as_str();
                    ret += ":";
                    buf = String::new();
                    s = Status::FindEmojiDelimiter;
                }
            }
        } else if s == Status::Accumulate && ((c >= 'a' && c <= 'z') || c == '_' || c == '-') {
            // emoji name
            buf.push(c);
        } else if s == Status::Accumulate {
            ret += ":";
            ret += buf.as_str();
            // not emoji, forget it!
            buf = String::new();
            s = Status::FindEmojiDelimiter
        } else {
            ret.push(c);
        }

        i += 1;
    }

    ret
}

#[cfg(test)]
mod tests {
    use crate::reverse;

    #[test]
    fn test_1() {
        assert_eq!(reverse(""), "");
    }

    #[test]
    fn test_2() {
        assert_eq!(reverse(":_a:"), "あ");
    }

    #[test]
    fn test_3() {
        assert_eq!(reverse(":_ka:"), "か");
    }

    #[test]
    fn test_4() {
        assert_eq!(reverse(":_sa::kigo_dakuten:"), "ざ");
    }

    #[test]
    fn test_5() {
        assert_eq!(reverse(":_ha:"), "は");
    }

    #[test]
    fn test_6() {
        assert_eq!(reverse(":_ha::kigo_dakuten:"), "ば");
    }

    #[test]
    fn test_7() {
        assert_eq!(reverse(":_ha::kigo_handakuten:"), "ぱ");
    }

    #[test]
    fn test_8() {
        assert_eq!(reverse(":_a::kigo_dakuten:"), "あ゛");
    }

    #[test]
    fn test_9() {
        assert_eq!(reverse(":_a::kigo_handakuten:"), "あ゜");
    }
}
