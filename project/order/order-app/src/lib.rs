use anyhow::bail;
use essential_types::{
    solution::{Mutation, Solution, SolutionData},
    convert::word_4_from_u8_32,
    PredicateAddress, Value, Word, ContentAddress, Key
};
use pint_abi;

pub const ORDER_AMOUNT_KEY: Word = 0;
pub const PRICE_KEY: Word = 1;
pub const TOKEN_A_KEY: Word = 2;
pub const TOKEN_B_KEY: Word = 3;
//const OWNER_KEY: Word = 4;

#[derive(Clone)]
pub struct CustomKey(pub Vec<Word>);

#[derive(Clone)]
pub struct QueryState(pub Option<Value>);

pub fn get_key(key: Word) -> CustomKey {
    CustomKey(vec![key])
}

pub fn extract_state(state: QueryState) -> anyhow::Result<Word> {
    match state.0 {
        Some(state) => match &state[..] {
            [] => Ok(0),
            [state] => Ok(*state),
            _ => bail!("Expected single word, got: {:?}", state),
        },
        None => Ok(0),
    }
}

pub fn create_bid(predicate: PredicateAddress, new_bid: Word, new_price: Word, new_A: ContentAddress, new_B: ContentAddress) -> Solution {
    Solution {
        data: vec![SolutionData {
            predicate_to_solve: predicate,
            decision_variables: vec![vec![new_bid], vec![new_price], word_4_from_u8_32(new_A.clone().0).to_vec(), word_4_from_u8_32(new_B.clone().0).to_vec()],
            transient_data: Default::default(),
            state_mutations: vec![Mutation {
                key: vec![ORDER_AMOUNT_KEY],
                value: vec![new_bid],
            },
            Mutation {
                key: vec![PRICE_KEY],
                value: vec![new_price],
            },
            Mutation {
                key: vec![TOKEN_A_KEY],
                value: word_4_from_u8_32(new_A.clone().0).to_vec(),
            },
            Mutation {
                key: vec![TOKEN_B_KEY],
                value: word_4_from_u8_32(new_A.clone().0).to_vec(),
            }],
        }],
    }
}