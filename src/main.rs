use std::io;
/*
 * usage:
 *   ./etherscan_holdings_csv_parser < export.csv
 *
 *
 * description:
 *    Parses a token transfer CSV export from etherscan.io passed in through stdin
 *    and prints out the addresses and their holdings with each line storing a new 
 *    account/holding pair and each line storing `address` `holding`, delimited by spaces
 *
 */

fn main() {
    // create csv parser 
    let mut reader = csv::Reader::from_reader(io::stdin());
    // loop through each line of the csv
    for result in reader.records() { 
        // create a string record from the line in the csv
        let record = result.expect("a CSV record");

        // iterate through the record for this line
        for (i, character) in record.iter().enumerate() {
            // 5th char is the address the shares are going to
            if i ==  5 { 
                print!("{} ", character);
            }
            // 6th char is the quantity of shares
            if i == 6  {
                print!("{}\n", character);
            }
        }
    }
}
