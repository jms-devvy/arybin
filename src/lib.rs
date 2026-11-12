use std::{collections::HashMap, hash::Hash};

//use crate::PathResult::LayerStruct;

pub mod btree {
    use std::{collections::HashMap, hash::Hash};

    #[derive(Clone, Debug)]
    enum PathResult {
        CharLeafStruct(CharLeafStruct),
        LayerStruct(Box<LayerStruct>),
    }
    #[derive(Clone, Debug)]
    struct Tree {
        start_path: LayerStruct,
    }

    #[derive(Debug)]
    enum NestedVec<T> {
        Value(T),
        List(Vec<NestedVec<T>>),
    }

    impl Tree {
        fn map_recurr(
            &self,
            key_path_temp_hashmap: &mut HashMap<char, Vec<i64>>,
            temp_layer_count: &mut i64,
            layer: LayerStruct,
            mut path_construct: Vec<i64>,
        ) {
            //path_construct should be passed as a .clone()
            *temp_layer_count = *temp_layer_count + 1;
            match &layer.zero_goes_to {
                PathResult::CharLeafStruct(leaf) => {
                    path_construct.push(0);

                    key_path_temp_hashmap.insert(leaf.char_val, path_construct.clone());
                    path_construct.pop();
                    println!("found leaf {} at {:?}0", leaf.char_val, path_construct);
                }
                PathResult::LayerStruct(inlayer) => {
                    println!("pushed 0");
                    path_construct.push(0);

                    self.map_recurr(
                        key_path_temp_hashmap,
                        temp_layer_count,
                        (**inlayer).clone(),
                        path_construct.clone(),
                    );
                }
            }

            match &layer.one_goes_to {
                PathResult::CharLeafStruct(leaf) => {
                    path_construct.push(1);

                    key_path_temp_hashmap.insert(leaf.char_val, path_construct.clone());
                    path_construct.pop();

                    println!("found leaf {} at {:?}1", leaf.char_val, path_construct);
                }
                PathResult::LayerStruct(inlayer) => {
                    path_construct.push(1);
                    self.map_recurr(
                        key_path_temp_hashmap,
                        temp_layer_count,
                        (**inlayer).clone(),
                        path_construct.clone(),
                    );
                }
            }
        } //function end indent

        pub fn map(&self) -> TreeSearchResultStruct {
            let mut key_path_temp_hashmap: HashMap<char, Vec<i64>> = HashMap::new();
            let mut temp_layer_count: i64 = 0;
            let mut path_construct: Vec<i64> = Vec::new();
            self.map_recurr(
                &mut key_path_temp_hashmap,
                &mut temp_layer_count,
                self.start_path.clone(),
                path_construct.clone(),
            );
            return TreeSearchResultStruct {
                key_path_map: key_path_temp_hashmap,
                map_depth: temp_layer_count,
            };
        }
    }

    #[derive(Clone, Debug)]
    pub struct LayerStruct {
        zero_goes_to: PathResult,
        one_goes_to: PathResult,
    }
    #[derive(Clone, Debug)]
    pub struct TreeSearchResultStruct {
        key_path_map: HashMap<char, Vec<i64>>,
        map_depth: i64,
    }
    #[derive(Clone, Debug)]
    pub struct CharLeafStruct {
        char_val: char,
        amount: i64,
    }
}

pub mod huffman {
    //WIP
    use std::{collections::HashMap, hash::Hash};

    fn get_freqs(input_string: String, num_freq_hashmap: &mut HashMap<char, i64>) {
        let collected: Vec<char> = input_string.chars().collect();
        //get frequency
        for i in collected {
            if let Some(bleh) = num_freq_hashmap.get_mut(&i) {
                *bleh += 1;
            } else {
                num_freq_hashmap.insert(i, 1);
            }
        }
    }
}
