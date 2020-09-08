use crate::computer_player::{Evaluation, NOUGHT_AND_CROSS_LEN, WIN_AND_DRAW_LEN};
use casual_logger::Log;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

impl Evaluation {
    /// Save to a file.  
    /// ファイルへ保存します。  
    pub fn save(&self, file_name: &str) {
        let mut text = String::new();
        // Open the file.
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(Path::new(file_name))
            // TODO error handling.
            .unwrap();

        // 1
        for state in self.features_1_to_7[0].iter() {
            self.push_val(&state, &mut text);
        }
        text.push_str("\r\n");
        // 2
        for state in self.features_1_to_7[1].iter() {
            self.push_val(&state, &mut text);
        }
        text.push_str("\r\n");
        // 3
        for state in self.features_1_to_7[2].iter() {
            self.push_val(&state, &mut text);
        }
        text.push_str("\r\n");
        // 4
        for state in self.features_1_to_7[3].iter() {
            self.push_val(&state, &mut text);
        }
        text.push_str("\r\n");
        // 5
        for state in self.features_1_to_7[4].iter() {
            self.push_val(&state, &mut text);
        }
        text.push_str("\r\n");
        // 6
        for state in self.features_1_to_7[5].iter() {
            self.push_val(&state, &mut text);
        }
        text.push_str("\r\n");
        // 7
        for state in self.features_1_to_7[6].iter() {
            self.push_val(&state, &mut text);
        }
        text.push_str("\r\n");
        // 8
        for state in self.features_8_to_13[0].iter() {
            self.push_val(&state, &mut text);
        }
        text.push_str("\r\n");
        // 9
        for state in self.features_8_to_13[1].iter() {
            self.push_val(&state, &mut text);
        }
        text.push_str("\r\n");
        // 10
        for state in self.features_8_to_13[2].iter() {
            self.push_val(&state, &mut text);
        }
        text.push_str("\r\n");
        // 11
        for state in self.features_8_to_13[3].iter() {
            self.push_val(&state, &mut text);
        }
        text.push_str("\r\n");
        // 12
        for state in self.features_8_to_13[4].iter() {
            self.push_val(&state, &mut text);
        }
        text.push_str("\r\n");
        // 13
        for state in self.features_8_to_13[5].iter() {
            self.push_val(&state, &mut text);
        }
        text.push_str("\r\n");
        // 14
        for state in self.features_14_19_20_25[0].iter() {
            self.push_val(&state, &mut text);
        }
        text.push_str("\r\n");
        // 15
        for state in self.features_15_18_21_24[0].iter() {
            self.push_val(&state, &mut text);
        }
        text.push_str("\r\n");
        // 16
        for state in self.features_16_17_22_23[0].iter() {
            self.push_val(&state, &mut text);
        }
        text.push_str("\r\n");
        // 17
        for state in self.features_16_17_22_23[1].iter() {
            self.push_val(&state, &mut text);
        }
        text.push_str("\r\n");
        // 18
        for state in self.features_15_18_21_24[1].iter() {
            self.push_val(&state, &mut text);
        }
        text.push_str("\r\n");
        // 19
        for state in self.features_14_19_20_25[1].iter() {
            self.push_val(&state, &mut text);
        }
        text.push_str("\r\n");
        // 20
        for state in self.features_14_19_20_25[2].iter() {
            self.push_val(&state, &mut text);
        }
        text.push_str("\r\n");
        // 21
        for state in self.features_15_18_21_24[2].iter() {
            self.push_val(&state, &mut text);
        }
        text.push_str("\r\n");
        // 22
        for state in self.features_16_17_22_23[2].iter() {
            self.push_val(&state, &mut text);
        }
        text.push_str("\r\n");
        // 23
        for state in self.features_16_17_22_23[3].iter() {
            self.push_val(&state, &mut text);
        }
        text.push_str("\r\n");
        // 24
        for state in self.features_15_18_21_24[3].iter() {
            self.push_val(&state, &mut text);
        }
        text.push_str("\r\n");
        // 25
        for state in self.features_14_19_20_25[3].iter() {
            self.push_val(&state, &mut text);
        }
        text.push_str("\r\n");

        // Write.
        let mut file_buf = BufWriter::new(file);
        // write_all method required to use 'use std::io::Write;'.
        if let Err(why) = file_buf.write_all(text.as_bytes()) {
            panic!("couldn't write evaluation. : {}", why);
        }
    }

    pub fn push_val(
        &self,
        state: &[[u8; NOUGHT_AND_CROSS_LEN]; WIN_AND_DRAW_LEN],
        text: &mut String,
    ) {
        for result_channel in 0..WIN_AND_DRAW_LEN {
            for turn in 0..NOUGHT_AND_CROSS_LEN {
                text.push_str(&format!("{},", state[result_channel][turn]));
            }
        }
    }

    /// Load from a file.  
    /// TODO ファイルから読み込みます。  
    pub fn load(&mut self, file_name: &str) {
        // Open the file.
        let file = match OpenOptions::new().read(true).open(Path::new(file_name)) {
            Ok(file) => file,
            Err(why) => {
                // Ignored.
                Log::warn(&format!("{}", why));
                return;
            }
        };

        // Read.
        let reader = BufReader::new(file);
        let mut state = 0;
        let mut token_index;
        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for (feature_from0, line) in reader.lines().enumerate() {
            match line {
                Ok(line) => {
                    token_index = 0;
                    let tokens: Vec<&str> = line.split(",").collect();
                    let feature_index = feature_from0 + 1;
                    match feature_index {
                        1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                            for result_channel in 0..WIN_AND_DRAW_LEN {
                                for turn in 0..NOUGHT_AND_CROSS_LEN {
                                    self.features_1_to_7[feature_index - 1][state]
                                        [result_channel][turn] = match tokens[token_index].parse() {
                                        Ok(x) => x,
                                        Err(why) => panic!("couldn't read evaluation. : {}", why),
                                    };
                                    token_index += 1;
                                }
                            }
                        }
                        8 | 9 | 10 | 11 | 12 | 13 => {
                            for result_channel in 0..WIN_AND_DRAW_LEN {
                                for turn in 0..NOUGHT_AND_CROSS_LEN {
                                    self.features_8_to_13[feature_index - 8][state]
                                        [result_channel][turn] = match tokens[token_index].parse() {
                                        Ok(x) => x,
                                        Err(why) => panic!("couldn't read evaluation. : {}", why),
                                    };
                                    token_index += 1;
                                }
                            }
                        }
                        14 | 19 | 20 | 25 => {
                            for result_channel in 0..WIN_AND_DRAW_LEN {
                                for turn in 0..NOUGHT_AND_CROSS_LEN {
                                    let array_index = match feature_index {
                                        14 => 0,
                                        19 => 1,
                                        20 => 2,
                                        25 => 3,
                                        _ => {
                                            panic!("couldn't read evaluation. : {}", feature_index)
                                        }
                                    };
                                    self.features_14_19_20_25[array_index][state][result_channel]
                                        [turn] = match tokens[token_index].parse() {
                                        Ok(x) => x,
                                        Err(why) => panic!("couldn't read evaluation. : {}", why),
                                    };
                                    token_index += 1;
                                }
                            }
                        }
                        15 | 18 | 21 | 24 => {
                            for result_channel in 0..WIN_AND_DRAW_LEN {
                                for turn in 0..NOUGHT_AND_CROSS_LEN {
                                    let array_index = match feature_index {
                                        15 => 0,
                                        18 => 1,
                                        21 => 2,
                                        24 => 3,
                                        _ => {
                                            panic!("couldn't read evaluation. : {}", feature_index)
                                        }
                                    };
                                    self.features_15_18_21_24[array_index][state][result_channel]
                                        [turn] = match tokens[token_index].parse() {
                                        Ok(x) => x,
                                        Err(why) => panic!("couldn't read evaluation. : {}", why),
                                    };
                                    token_index += 1;
                                }
                            }
                        }
                        16 | 17 | 22 | 23 => {
                            for result_channel in 0..WIN_AND_DRAW_LEN {
                                for turn in 0..NOUGHT_AND_CROSS_LEN {
                                    let array_index = match feature_index {
                                        16 => 0,
                                        17 => 1,
                                        22 => 2,
                                        23 => 3,
                                        _ => {
                                            panic!("couldn't read evaluation. : {}", feature_index)
                                        }
                                    };
                                    self.features_16_17_22_23[array_index][state][result_channel]
                                        [turn] = match tokens[token_index].parse() {
                                        Ok(x) => x,
                                        Err(why) => panic!("couldn't read evaluation. : {}", why),
                                    };
                                    token_index += 1;
                                }
                            }
                        }
                        _ => panic!("Invalid feature. feature_index={}", feature_index),
                    }
                    state = 0;
                }
                Err(why) => panic!("couldn't read evaluation. : {}", why),
            }
        }
    }
}
