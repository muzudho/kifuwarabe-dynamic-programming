use crate::computer_player::FEATURE_V_H_B_S_LEN;
use crate::FILE_LEN;
use crate::{
    computer_player::{Bestmove, Learning, Search, WayValue},
    log::LogExt,
    Engine, ResultChannel, SearchInfo, EVALUATION_FILE_NAME,
};
use casual_logger::Log;
use rand::Rng;

impl Default for Learning {
    fn default() -> Self {
        Learning {}
    }
}
impl Learning {
    pub fn learn(&mut self, engine: &mut Engine) {
        let old_info_enabled = engine.pos.info_enabled;
        engine.pos.info_enabled = false;

        for game in 0..60 {
            Log::print_info(&format!("Game={}", game));
            engine.enter("pos");
            engine.enter("xfen");
            Log::print_info(&format!("PV_JSON={}", engine.pos.pv_json()));
            for retry_way in 0..10 {
                Log::print_info(&format!("RetryWay={}", retry_way));
                engine.enter("go");
                let bestmove = &engine.bestmove;
                if let Some(bestmove) = bestmove {
                    if let Some(chosen_file) = bestmove.file {
                        engine.enter(&format!("do {}", chosen_file));
                        engine.enter("pos");
                        engine.enter("xfen");
                        Log::print_info(&format!("PV_JSON={}", engine.pos.pv_json()));
                        if let Some(_) = engine.game_result {
                            // Game end.
                            break;
                        }
                    } else {
                        // Not found file. Retry.
                        // 列が未指定。 リトライ。
                        Log::print_info(&format!("Not found file. retry_way={}", retry_way));
                    }
                } else {
                    // Resign. Retry.
                    // 投了。 リトライ。
                    Log::print_info(&format!("Resign. retry_way={}", retry_way));
                }
            }
            Log::print_info(&"[Finished]");
            // Points are distributed during rewind.
            // 巻き戻し中にポイントが分配されます。
            engine.enter("pos");
            engine.enter("xfen");
            engine.enter("undo");
            while engine.undone {
                engine.enter("pos");
                engine.enter("xfen");
                // Learning.
                engine.enter("uh");
                // Undo.
                engine.enter("undo");
            }
            engine.enter("pos");
            engine.enter("xfen");
            Log::print_info("Save.");
            engine.evaluation.save(EVALUATION_FILE_NAME);
        }
        engine.pos.info_enabled = old_info_enabled;
    }

    /// uh...  
    /// うーん……。  
    pub fn uh(&mut self, engine: &mut Engine) {
        let old_info_enabled = engine.pos.info_enabled;
        engine.pos.info_enabled = false;

        self.uh_by_result_channel(engine, ResultChannel::Win);
        self.uh_by_result_channel(engine, ResultChannel::Draw);

        engine.pos.info_enabled = old_info_enabled;
        engine.evaluation.save(EVALUATION_FILE_NAME);
    }

    /// uh...  
    /// うーん……。  
    pub fn uh_by_result_channel(&mut self, engine: &mut Engine, result_channel: ResultChannel) {
        let mut search = Search::default();
        search.start_pieces_num = engine.pos.pieces_num;

        let mut search_info = SearchInfo::default();
        let files_way = [
            {
                let mut bestmove = Bestmove::default();
                search.node_exit(
                    &mut engine.pos,
                    &engine.evaluation,
                    &result_channel,
                    'a',
                    &mut search_info,
                    &mut bestmove,
                    0,
                );
                bestmove
            },
            {
                let mut bestmove = Bestmove::default();
                search.node_exit(
                    &mut engine.pos,
                    &engine.evaluation,
                    &result_channel,
                    'b',
                    &mut search_info,
                    &mut bestmove,
                    0,
                );
                bestmove
            },
            {
                let mut bestmove = Bestmove::default();
                search.node_exit(
                    &mut engine.pos,
                    &engine.evaluation,
                    &result_channel,
                    'c',
                    &mut search_info,
                    &mut bestmove,
                    0,
                );
                bestmove
            },
            {
                let mut bestmove = Bestmove::default();
                search.node_exit(
                    &mut engine.pos,
                    &engine.evaluation,
                    &result_channel,
                    'd',
                    &mut search_info,
                    &mut bestmove,
                    0,
                );
                bestmove
            },
            {
                let mut bestmove = Bestmove::default();
                search.node_exit(
                    &mut engine.pos,
                    &engine.evaluation,
                    &result_channel,
                    'e',
                    &mut search_info,
                    &mut bestmove,
                    0,
                );
                bestmove
            },
            {
                let mut bestmove = Bestmove::default();
                search.node_exit(
                    &mut engine.pos,
                    &engine.evaluation,
                    &result_channel,
                    'f',
                    &mut search_info,
                    &mut bestmove,
                    0,
                );
                bestmove
            },
            {
                let mut bestmove = Bestmove::default();
                search.node_exit(
                    &mut engine.pos,
                    &engine.evaluation,
                    &result_channel,
                    'g',
                    &mut search_info,
                    &mut bestmove,
                    0,
                );
                bestmove
            },
        ];

        // The number of files for which points can be obtained.
        // 点数を得られる列数。
        let mut obtainer = [false; FILE_LEN];
        let mut obtainer_count = 0;
        for file in 0..FILE_LEN {
            match result_channel {
                ResultChannel::Win => match files_way[file].pred_result {
                    WayValue::PossiblyWin | WayValue::Win => {
                        obtainer[file] = true;
                        obtainer_count += 1;
                    }
                    _ => {}
                },
                ResultChannel::Draw => match files_way[file].pred_result {
                    WayValue::PossiblyDraw | WayValue::Draw => {
                        obtainer[file] = true;
                        obtainer_count += 1;
                    }
                    _ => {}
                },
            }
        }
        let co_obtainer_count = FILE_LEN as u16 - obtainer_count;

        if obtainer_count < 1 {
            Log::print_info(&format!(
                "Result channel={:?} Obtainer nothing.",
                result_channel
            ));
            return;
        } else if co_obtainer_count < 1 {
            Log::print_info(&format!(
                "Result channel={:?} Co-obtainer nothing.",
                result_channel
            ));
            return;
        }

        // It can move the evaluation value.
        // 評価値が移動できます。
        let tensor_before_give = engine.evaluation.ways_weight(&engine.pos, &result_channel);
        let mut obtainer_files = Vec::<usize>::new();
        let give_values = [
            {
                let file_ch = 'a';
                let old =
                    engine
                        .evaluation
                        .get_values_by_file(&engine.pos, file_ch, &result_channel);
                let gives =
                    engine
                        .evaluation
                        .give_value_by_file(&engine.pos, file_ch, &result_channel, 4);
                let new_ =
                    engine
                        .evaluation
                        .get_values_by_file(&engine.pos, file_ch, &result_channel);
                Log::print_info(&format!(
                    "{} old=|{}|{}|{}|{}| gives={} new=|{}|{}|{}|{}|",
                    file_ch,
                    old[0],
                    old[1],
                    old[2],
                    old[3],
                    gives,
                    new_[0],
                    new_[1],
                    new_[2],
                    new_[3],
                ));
                gives
            },
            {
                let file_ch = 'b';
                let old =
                    engine
                        .evaluation
                        .get_values_by_file(&engine.pos, file_ch, &result_channel);
                let gives =
                    engine
                        .evaluation
                        .give_value_by_file(&engine.pos, file_ch, &result_channel, 4);
                let new_ =
                    engine
                        .evaluation
                        .get_values_by_file(&engine.pos, file_ch, &result_channel);
                Log::print_info(&format!(
                    "{} old=|{}|{}|{}|{}| gives={} new=|{}|{}|{}|{}|",
                    file_ch,
                    old[0],
                    old[1],
                    old[2],
                    old[3],
                    gives,
                    new_[0],
                    new_[1],
                    new_[2],
                    new_[3],
                ));
                gives
            },
            {
                let file_ch = 'c';
                let old =
                    engine
                        .evaluation
                        .get_values_by_file(&engine.pos, file_ch, &result_channel);
                let gives =
                    engine
                        .evaluation
                        .give_value_by_file(&engine.pos, file_ch, &result_channel, 4);
                let new_ =
                    engine
                        .evaluation
                        .get_values_by_file(&engine.pos, file_ch, &result_channel);
                Log::print_info(&format!(
                    "{} old=|{}|{}|{}|{}| gives={} new=|{}|{}|{}|{}|",
                    file_ch,
                    old[0],
                    old[1],
                    old[2],
                    old[3],
                    gives,
                    new_[0],
                    new_[1],
                    new_[2],
                    new_[3],
                ));
                gives
            },
            {
                let file_ch = 'd';
                let old =
                    engine
                        .evaluation
                        .get_values_by_file(&engine.pos, file_ch, &result_channel);
                let gives =
                    engine
                        .evaluation
                        .give_value_by_file(&engine.pos, file_ch, &result_channel, 4);
                let new_ =
                    engine
                        .evaluation
                        .get_values_by_file(&engine.pos, file_ch, &result_channel);
                Log::print_info(&format!(
                    "{} old=|{}|{}|{}|{}| gives={} new=|{}|{}|{}|{}|",
                    file_ch,
                    old[0],
                    old[1],
                    old[2],
                    old[3],
                    gives,
                    new_[0],
                    new_[1],
                    new_[2],
                    new_[3],
                ));
                gives
            },
            {
                let file_ch = 'e';
                let old =
                    engine
                        .evaluation
                        .get_values_by_file(&engine.pos, file_ch, &result_channel);
                let gives =
                    engine
                        .evaluation
                        .give_value_by_file(&engine.pos, file_ch, &result_channel, 4);
                let new_ =
                    engine
                        .evaluation
                        .get_values_by_file(&engine.pos, file_ch, &result_channel);
                Log::print_info(&format!(
                    "{} old=|{}|{}|{}|{}| gives={} new=|{}|{}|{}|{}|",
                    file_ch,
                    old[0],
                    old[1],
                    old[2],
                    old[3],
                    gives,
                    new_[0],
                    new_[1],
                    new_[2],
                    new_[3],
                ));
                gives
            },
            {
                let file_ch = 'f';
                let old =
                    engine
                        .evaluation
                        .get_values_by_file(&engine.pos, file_ch, &result_channel);
                let gives =
                    engine
                        .evaluation
                        .give_value_by_file(&engine.pos, file_ch, &result_channel, 4);
                let new_ =
                    engine
                        .evaluation
                        .get_values_by_file(&engine.pos, file_ch, &result_channel);
                Log::print_info(&format!(
                    "{} old=|{}|{}|{}|{}| gives={} new=|{}|{}|{}|{}|",
                    file_ch,
                    old[0],
                    old[1],
                    old[2],
                    old[3],
                    gives,
                    new_[0],
                    new_[1],
                    new_[2],
                    new_[3],
                ));
                gives
            },
            {
                let file_ch = 'g';
                let old =
                    engine
                        .evaluation
                        .get_values_by_file(&engine.pos, file_ch, &result_channel);
                let gives =
                    engine
                        .evaluation
                        .give_value_by_file(&engine.pos, file_ch, &result_channel, 4);
                let new_ =
                    engine
                        .evaluation
                        .get_values_by_file(&engine.pos, file_ch, &result_channel);
                Log::print_info(&format!(
                    "{} old=|{}|{}|{}|{}| gives={} new=|{}|{}|{}|{}|",
                    file_ch,
                    old[0],
                    old[1],
                    old[2],
                    old[3],
                    gives,
                    new_[0],
                    new_[1],
                    new_[2],
                    new_[3],
                ));
                gives
            },
        ];
        let gives_total = {
            let mut sum = 0;
            for file in 0..FILE_LEN {
                sum += give_values[file];
            }
            sum
        };
        let obtain_point = gives_total / obtainer_count;
        let rest_point = gives_total % obtainer_count;
        let mut take1_values = [
            {
                if obtainer[0] {
                    obtain_point
                } else {
                    0
                }
            },
            {
                if obtainer[1] {
                    obtain_point
                } else {
                    0
                }
            },
            {
                if obtainer[2] {
                    obtain_point
                } else {
                    0
                }
            },
            {
                if obtainer[3] {
                    obtain_point
                } else {
                    0
                }
            },
            {
                if obtainer[4] {
                    obtain_point
                } else {
                    0
                }
            },
            {
                if obtainer[5] {
                    obtain_point
                } else {
                    0
                }
            },
            {
                if obtainer[6] {
                    obtain_point
                } else {
                    0
                }
            },
        ];
        {
            for file in 0..FILE_LEN {
                if obtainer[file] {
                    obtainer_files.push(file);
                }
            }
            for _i in 0..rest_point {
                take1_values
                    [obtainer_files[rand::thread_rng().gen_range(0, obtainer_count) as usize]] += 1;
            }
        }
        let takes_total = {
            let mut sum = 0;
            for file in 0..FILE_LEN {
                sum += take1_values[file];
            }
            sum
        };

        let mut text = String::new();
        text.push_str(&format!(
            "Result channel={:?}
",
            result_channel
        ));
        Log::print_info(&text);

        let tensor_before_take = engine.evaluation.ways_weight(&engine.pos, &result_channel);
        let rest_values = [
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'a',
                    &result_channel,
                    take1_values[0],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'b',
                    &result_channel,
                    take1_values[1],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'c',
                    &result_channel,
                    take1_values[2],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'd',
                    &result_channel,
                    take1_values[3],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'e',
                    &result_channel,
                    take1_values[4],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'f',
                    &result_channel,
                    take1_values[5],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'g',
                    &result_channel,
                    take1_values[6],
                )
            },
        ];

        // Refund.
        // 還付。
        let refund_total = {
            let mut sum = 0;
            for val in &rest_values {
                sum += val;
            }
            sum
        };
        let mut refund1_values;
        let mut refunder_files = Vec::<usize>::new();
        {
            let refund_point = refund_total / co_obtainer_count;
            let refund_rest_point = refund_total % co_obtainer_count;
            refund1_values = [
                {
                    if !obtainer[0] {
                        refund_point
                    } else {
                        0
                    }
                },
                {
                    if !obtainer[1] {
                        refund_point
                    } else {
                        0
                    }
                },
                {
                    if !obtainer[2] {
                        refund_point
                    } else {
                        0
                    }
                },
                {
                    if !obtainer[3] {
                        refund_point
                    } else {
                        0
                    }
                },
                {
                    if !obtainer[4] {
                        refund_point
                    } else {
                        0
                    }
                },
                {
                    if !obtainer[5] {
                        refund_point
                    } else {
                        0
                    }
                },
                {
                    if !obtainer[6] {
                        refund_point
                    } else {
                        0
                    }
                },
            ];
            {
                for file in 0..FILE_LEN {
                    if !obtainer[file] {
                        refunder_files.push(file);
                    }
                }
                for _i in 0..refund_rest_point {
                    refund1_values[refunder_files
                        [rand::thread_rng().gen_range(0, co_obtainer_count) as usize]] += 1;
                }
            }
        }
        let tensor_before_refund = engine.evaluation.ways_weight(&engine.pos, &result_channel);
        let lost_values = [
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'a',
                    &result_channel,
                    refund1_values[0],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'b',
                    &result_channel,
                    refund1_values[1],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'c',
                    &result_channel,
                    refund1_values[2],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'd',
                    &result_channel,
                    refund1_values[3],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'e',
                    &result_channel,
                    refund1_values[4],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'f',
                    &result_channel,
                    refund1_values[5],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'g',
                    &result_channel,
                    refund1_values[6],
                )
            },
        ];
        let lost_value = {
            let mut sum = 0;
            for val in &lost_values {
                sum += val;
            }
            sum
        };
        if 0 < lost_value {
            panic!(Log::print_fatal(&format!(
                "(Err.459) Learn fail. lost_value={}",
                lost_value
            )))
        }

        let mut text = String::new();
        text.push_str(&format!(
            "Gives total={} Takes total={} Refund total={}
",
            gives_total, takes_total, refund_total
        ));
        text.push_str(&format!(
            "obtainer_files={:?}
",
            obtainer_files
        ));
        text.push_str(&format!(
            "refunder_files={:?}
",
            refunder_files
        ));

        text.push_str(&format!(
            "\
[Learn]
     | Feature number      | Current evaluation        | Choice way    | Give                           | Take                                | Refund
File | Vert Hori Baro Sini | Vert Hori Baro Sini Total | File   Result | Val  Vert Hori Baro Sini Total | Val  Rest Vert Hori Baro Sini Total | Val  Vert Hori Baro Sini Total
---- + ---- ---- ---- ---- + ---- ---- ---- ---- ----- + ------ ------ + ---- ---- ---- ---- ---- ----- + ---- ---- ---- ---- ---- ---- ----- + ---- ---- ---- ---- ---- -----
"
        ));
        text.push_str(&self.score_line_by_file(
            0,
            'a',
            &engine.evaluation.ways_feat(&engine.pos),
            &tensor_before_give,
            &tensor_before_take,
            &tensor_before_refund,
            &engine.evaluation.ways_weight(&engine.pos, &result_channel),
            &files_way,
            &give_values,
            &take1_values,
            &rest_values,
            &refund1_values,
        ));
        text.push_str(&self.score_line_by_file(
            1,
            'b',
            &engine.evaluation.ways_feat(&engine.pos),
            &tensor_before_give,
            &tensor_before_take,
            &tensor_before_refund,
            &engine.evaluation.ways_weight(&engine.pos, &result_channel),
            &files_way,
            &give_values,
            &take1_values,
            &rest_values,
            &refund1_values,
        ));
        text.push_str(&self.score_line_by_file(
            2,
            'c',
            &engine.evaluation.ways_feat(&engine.pos),
            &tensor_before_give,
            &tensor_before_take,
            &tensor_before_refund,
            &engine.evaluation.ways_weight(&engine.pos, &result_channel),
            &files_way,
            &give_values,
            &take1_values,
            &rest_values,
            &refund1_values,
        ));
        text.push_str(&self.score_line_by_file(
            3,
            'd',
            &engine.evaluation.ways_feat(&engine.pos),
            &tensor_before_give,
            &tensor_before_take,
            &tensor_before_refund,
            &engine.evaluation.ways_weight(&engine.pos, &result_channel),
            &files_way,
            &give_values,
            &take1_values,
            &rest_values,
            &refund1_values,
        ));
        text.push_str(&self.score_line_by_file(
            4,
            'e',
            &engine.evaluation.ways_feat(&engine.pos),
            &tensor_before_give,
            &tensor_before_take,
            &tensor_before_refund,
            &engine.evaluation.ways_weight(&engine.pos, &result_channel),
            &files_way,
            &give_values,
            &take1_values,
            &rest_values,
            &refund1_values,
        ));
        text.push_str(&self.score_line_by_file(
            5,
            'f',
            &engine.evaluation.ways_feat(&engine.pos),
            &tensor_before_give,
            &tensor_before_take,
            &tensor_before_refund,
            &engine.evaluation.ways_weight(&engine.pos, &result_channel),
            &files_way,
            &give_values,
            &take1_values,
            &rest_values,
            &refund1_values,
        ));
        text.push_str(&self.score_line_by_file(
            6,
            'g',
            &engine.evaluation.ways_feat(&engine.pos),
            &tensor_before_give,
            &tensor_before_take,
            &tensor_before_refund,
            &engine.evaluation.ways_weight(&engine.pos, &result_channel),
            &files_way,
            &give_values,
            &take1_values,
            &rest_values,
            &refund1_values,
        ));
        Log::print_info(&text);
    }

    fn score_line_by_file(
        &self,
        file: usize,
        file_ch: char,
        tensor_of_feat_number: &[[Option<u8>; FEATURE_V_H_B_S_LEN]; FILE_LEN],
        tensor_before_give: &[[u8; FEATURE_V_H_B_S_LEN]; FILE_LEN],
        tensor_before_take: &[[u8; FEATURE_V_H_B_S_LEN]; FILE_LEN],
        tensor_before_refund: &[[u8; FEATURE_V_H_B_S_LEN]; FILE_LEN],
        new_tensor: &[[u8; FEATURE_V_H_B_S_LEN]; FILE_LEN],
        files_way: &[Bestmove; FILE_LEN],
        give_values: &[u16],
        take1_values: &[u16],
        rest_values: &[u16],
        refund_values: &[u16],
    ) -> String {
        format!(
            "{: >4} | {: >4} {: >4} {: >4} {: >4} | {: >4} {: >4} {: >4} {: >4} {: >5} | {: <6} {: <6} | {: >4} {: >4} {: >4} {: >4} {: >4} {: >5} | {: >4} {: >4} {: >4} {: >4} {: >4} {: >4} {: >5} | {: >4} {: >4} {: >4} {: >4} {: >4} {: >5}
",
            file_ch,
            Learning::none_zero_to_point(tensor_of_feat_number[file][0]),
            Learning::none_zero_to_point(tensor_of_feat_number[file][1]),
            Learning::none_zero_to_point(tensor_of_feat_number[file][2]),
            Learning::none_zero_to_point(tensor_of_feat_number[file][3]),
            Learning::zero_to_point(tensor_before_give[file][0] as u16),
            Learning::zero_to_point(tensor_before_give[file][1] as u16),
            Learning::zero_to_point(tensor_before_give[file][2] as u16),
            Learning::zero_to_point(tensor_before_give[file][3] as u16),
            Learning::zero_to_point(tensor_before_give[file][0] as u16 + tensor_before_give[file][1] as u16 + tensor_before_give[file][2] as u16 + tensor_before_give[file][3] as u16),
            //
            if let Some(file) = files_way[file].file {
                file.to_string()
            } else {
                "resign".to_string()
            },
            &format!("{:?}", files_way[file].pred_result),
            //
            Learning::zero_to_point(give_values[file]),
            Learning::zero_to_point(tensor_before_take[file][0]as u16),
            Learning::zero_to_point(tensor_before_take[file][1]as u16),
            Learning::zero_to_point(tensor_before_take[file][2]as u16),
            Learning::zero_to_point(tensor_before_take[file][3]as u16),
            Learning::zero_to_point(tensor_before_take[file][0] as u16 + tensor_before_take[file][1] as u16 + tensor_before_take[file][2] as u16 + tensor_before_take[file][3] as u16),
            //
            Learning::zero_to_point(take1_values[file]as u16),
            Learning::zero_to_point(rest_values[file]as u16),
            Learning::zero_to_point(tensor_before_refund[file][0]as u16),
            Learning::zero_to_point(tensor_before_refund[file][1]as u16),
            Learning::zero_to_point(tensor_before_refund[file][2]as u16),
            Learning::zero_to_point(tensor_before_refund[file][3]as u16),
            Learning::zero_to_point(tensor_before_refund[file][0] as u16 + tensor_before_refund[file][1] as u16 + tensor_before_refund[file][2] as u16 + tensor_before_refund[file][3] as u16),
            //
            Learning::zero_to_point(refund_values[file]as u16),
            Learning::zero_to_point(new_tensor[file][0]as u16),
            Learning::zero_to_point(new_tensor[file][1]as u16),
            Learning::zero_to_point(new_tensor[file][2]as u16),
            Learning::zero_to_point(new_tensor[file][3]as u16),
            Learning::zero_to_point(new_tensor[file][0] as u16 + new_tensor[file][1] as u16 + new_tensor[file][2] as u16 + new_tensor[file][3] as u16),
        )
    }

    fn none_zero_to_point(num: Option<u8>) -> String {
        if let Some(num) = num {
            if num == 0 {
                ".".to_string()
            } else {
                num.to_string()
            }
        } else {
            ".".to_string()
        }
    }

    fn zero_to_point(num: u16) -> String {
        if num == 0 {
            ".".to_string()
        } else {
            num.to_string()
        }
    }
}
