mod abi;
use acme_shared::pb::acme::v1::usdt;
use hex_literal::hex;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

substreams_ethereum::init!();

const USDT_TRACKED_CONTRACT: [u8; 20] = hex!("dac17f958d2ee523a2206206994597c13d831ec7");

fn map_usdt_transfers(blk: &eth::Block, transfers: &mut usdt::Transfers) {
    transfers.items.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == USDT_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::usdt_contract::events::Transfer::match_and_decode(log)
                        {
                            return Some(usdt::Transfer {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                from: event.from,
                                to: event.to,
                                value: event.value.to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
}

#[substreams::handlers::map]
fn map_transfers(blk: eth::Block) -> Result<usdt::Transfers, substreams::errors::Error> {
    let mut transfers = usdt::Transfers::default();
    map_usdt_transfers(&blk, &mut transfers);
    substreams::skip_empty_output();
    Ok(transfers)
}
