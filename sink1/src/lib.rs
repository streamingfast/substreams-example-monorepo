use std::str::FromStr;

use acme_shared::pb::acme::v1::usdt;
use substreams::scalar::BigDecimal;
use substreams::Hex;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;

fn db_transfers_out(transfers: &usdt::Transfers, tables: &mut DatabaseChangeTables) {
    transfers.items.iter().for_each(|evt| {
        tables
            .create_row(
                "transfer",
                [
                    ("evt_tx_hash", evt.evt_tx_hash.to_string()),
                    ("evt_index", evt.evt_index.to_string()),
                ],
            )
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string())
            .set("value", BigDecimal::from_str(&evt.value).unwrap());
    });
}

#[substreams::handlers::map]
fn db_out(transfers: usdt::Transfers) -> Result<DatabaseChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = DatabaseChangeTables::new();
    db_transfers_out(&transfers, &mut tables);
    Ok(tables.to_database_changes())
}
