use order_app::*;
use essential_app_utils as utils;
use essential_app_utils::{
    compile::compile_pint_project,
    db::{new_dbs, Dbs},
};
use essential_node as node;
use essential_types::{
  convert::word_4_from_u8_32,
  ContentAddress, PredicateAddress, Word};

#[tokio::test]
async fn test() {
  let order = compile_pint_project(concat!(env!("CARGO_MANIFEST_DIR"), "/..").into())
    .await
    .unwrap();

  let order_address = essential_hash::contract_addr::from_contract(&order);
  let order_pred_address = PredicateAddress {
    contract: order_address.clone(),
    predicate: essential_hash::content_addr(&order.predicates[0]),
  };
  let create_bid_pred_address = PredicateAddress {
    contract: order_address.clone(),
    predicate: essential_hash::content_addr(&order.predicates[1]),
  };

  let tokenA = compile_pint_project(concat!(env!("CARGO_MANIFEST_DIR"), "/../tokenA").into())
    .await
    .unwrap();
  
    let tokenA_address = essential_hash::contract_addr::from_contract(&tokenA);
  let mintA_pred_address = PredicateAddress {
    contract: tokenA_address.clone(),
    predicate: essential_hash::content_addr(&tokenA.predicates[0]),
  };
  let sendA_pred_address = PredicateAddress {
    contract: tokenA_address.clone(),
    predicate: essential_hash::content_addr(&tokenA.predicates[1]),
  };

  let tokenB = compile_pint_project(concat!(env!("CARGO_MANIFEST_DIR"), "/../tokenB").into())
    .await
    .unwrap();

  let tokenB_address = essential_hash::contract_addr::from_contract(&tokenB);
  let mintB_pred_address = PredicateAddress {
    contract: tokenB_address.clone(),
    predicate: essential_hash::content_addr(&tokenB.predicates[0]),
  };
  let sendB_pred_address = PredicateAddress {
    contract: tokenB_address.clone(),
    predicate: essential_hash::content_addr(&tokenB.predicates[1]),
  };

  let dbs = new_dbs().await;
  essential_app_utils::deploy::deploy_contract(&dbs.builder, &order)
    .await
    .unwrap();

  essential_app_utils::deploy::deploy_contract(&dbs.builder, &tokenA)
    .await
    .unwrap();

  essential_app_utils::deploy::deploy_contract(&dbs.builder, &tokenB)
    .await
    .unwrap();

  let key = get_key(PRICE_KEY);
  let key_val = read_key(&dbs.node, &order_address, &key).await;
  assert_eq!(key_val, 0);

  initialize(&dbs, create_bid_pred_address.clone(), tokenA_address.clone(), tokenB_address.clone()).await;

  let o = utils::builder::build_default(&dbs).await.unwrap();
  println!("{:?}", o.failed);
  assert_eq!(o.succeeded.len(), 5);
  assert!(o.failed.is_empty());

  let key_val = read_key(&dbs.node, &order_address, &key).await;
  assert_eq!(key_val, 3000);
}

async fn read_key(
  conn: &node::db::ConnectionPool,
  address: &ContentAddress,
  key: &CustomKey,
) -> Word {
  let r = utils::node::query_state_head(conn, address, &key.0)
      .await
      .unwrap();
  extract_state(QueryState(r)).unwrap()
}

async fn initialize(dbs: &Dbs, predicate_address: PredicateAddress, tokenA_address: ContentAddress, tokenB_address: ContentAddress) {
  // let key = counter_key();
  // let current_count = dbs
  //     .node
  //     .query_state(predicate_address.contract.clone(), key.0)
  //     .await
  //     .unwrap();
  let solution = create_bid(predicate_address, 2, 3000, tokenA_address, tokenB_address);

  utils::builder::submit(&dbs.builder, solution)
      .await
      .unwrap();
}