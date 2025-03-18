use std::{error::Error, fs::File, io, path::Path, process};

use serde::Deserialize;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CheckingRecord {
    #[serde(rename = "Details")]
    details: String,

    #[serde(rename = "Posting Date")]
    posting_date: String,

    #[serde(rename = "Description")]
    description: String,

    #[serde(rename = "Amount")]
    amount: String,

    #[serde(rename = "Type")]
    r#type: String,

    #[serde(rename = "Balance")]
    balance: String,

    #[serde(rename = "Check or Slip #")]
    check_number: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CreditCardRecord {
    #[serde(rename = "Card")]
    card: String,

    #[serde(rename = "Transaction Date")]
    transaction_date: String,

    #[serde(rename = "Post Date")]
    post_date: String,

    #[serde(rename = "Description")]
    description: String,

    #[serde(rename = "Category")]
    category: String,

    #[serde(rename = "Type")]
    r#type: String,

    #[serde(rename = "Amount")]
    amount: String,

    #[serde(rename = "Memo")]
    memo: String,
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut csv_writer = csv::Writer::from_writer(io::stdout());
    csv_writer.write_record(&[
        "Date",
        "Account",
        "Amount",
        "Fee",
        "Net Amount",
        "Description",
        "Type",
        "Status",
    ])?;

    let input_checking = File::open(Path::new("checking.csv"))?;
    let mut csv_reader_checking = csv::ReaderBuilder::new()
        .flexible(true)
        .from_reader(input_checking);

    for result in csv_reader_checking.deserialize() {
        let record: CheckingRecord = result?;

        csv_writer.write_record(&[
            &record.posting_date,
            "Checking",
            &record.amount,
            "",
            "",
            &record.description,
            &record.r#type,
            &record.details,
        ])?;
    }

    let input_credit_card = File::open(Path::new("credit-card.csv"))?;
    let mut csv_reader_credit_card = csv::ReaderBuilder::new()
        .flexible(true)
        .from_reader(input_credit_card);

    for result in csv_reader_credit_card.deserialize() {
        let record: CreditCardRecord = result?;

        csv_writer.write_record(&[
            &record.transaction_date,
            "Credit Card",
            &record.amount,
            "",
            "",
            &record.description,
            &record.r#type,
            &record.memo,
        ])?;
    }

    csv_writer.flush()?;

    Ok(())
}
