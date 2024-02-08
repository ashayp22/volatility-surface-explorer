import { roundToDecimalPlaces } from "./calc.js";

function getDaysFromJanuary(month) {
    switch (month) {
        case "Jan":
        case "January":
            return 31.0;
        case "Feb":
        case "February":
            return 57.0;
        case "Mar":
        case "March":
            return 90.0;
        case "Apr":
        case "April":
            return 120.0;
        case "May":
            return 151.0
        case "Jun":
        case "June":
            return 181.0
        case "Jul":
        case "July":
            return 212.0
        case "Aug":
        case "August":
            return 243.0
        case "Sep":
        case "September":
            return 273.0
        case "Oct":
        case "October":
            return 304.0
        case "Nov":
        case "November":
            return 334.0
        case "Dec":
        case "December":
            return 365.0
        default:
            return 0.0
    }
}

function removeNonNumCharacters(str) {
    let newStr = "";

    for (let i = 0; i < str.length; i++) {
        if (!isNaN(str[i]) || str[i] == ".") {
            newStr += str[i];
        }
    }

    return newStr;
}

// Parses CBOE data based on the option chain data format 
// speicfied here https://www.cboe.com/delayed_quotes/spy/quote_table (February 2024)
export function parseOptionData(data) {
    const split_data = data.split("\n");

    let spot = 0.0;
    let today = 0.0;
    let current_year = 2000;
    let call_prices = [];
    let call_strikes = [];
    let put_prices = [];
    let put_strikes = [];
    let years_to_expiry = [];
    let names = [];
    let option_name = ""
    let time = "";

    let line_counter = 0;

    // Start at 1 since the first row is empty
    for (let i = 1; i < split_data.length; i++) {
        const line = split_data[i];

        line_counter += 1;

        if (line_counter == 1) {
            let split_line = line.split(",");
            option_name = split_line[0];
        } else if (line_counter == 2) {
            // This line should look like:
            // "Date: February 8, 2024 at 11:23 AM EST",Bid: 497.7,Ask: 497.71,Size: 17*14,"Volume: 10,598,673"
            let split_line = line.split(",");

            let bid = parseFloat(removeNonNumCharacters(split_line[2]));
            let ask = parseFloat(removeNonNumCharacters(split_line[3]));

            spot = roundToDecimalPlaces((bid + ask) / 2.0, 6);

            time = split_line[0] + split_line[1];

            let date_line = split_line[0].split(" ");
            let month = date_line[1];

            let year_line = split_line[1].split(" ");
            current_year = parseInt(year_line[1], 10);

            let current_day = parseInt(removeNonNumCharacters(date_line[2]));

            today = getDaysFromJanuary(month) + current_day;
        } else if (line_counter >= 4) {
            let split_line = line.split(",");

            if (split_line.length != 22) {
                continue;
            }

            // Parse strike
            let strike = parseFloat(split_line[11]);

            if (strike <= 0.0) {
                continue;
            }

            call_strikes.push(strike);
            put_strikes.push(strike);

            // Parse name
            names.push(split_line[1]);

            // Parse time to expiry
            let date_price = split_line[0].split(" ");

            let year = parseInt(date_price[3], 10);
            let days = parseFloat(date_price[2]);
            let month_days = getDaysFromJanuary(date_price[1]);

            // Add one for end of day
            let expiration = ((year - current_year)) * 365.0 + month_days + days + 1.0;

            years_to_expiry.push((expiration - today) / 365.0);

            // Parse put and call prices
            let call_bid = parseFloat(split_line[4]);
            let call_ask = parseFloat(split_line[5]);
            let put_bid = parseFloat(split_line[15]);
            let put_ask = parseFloat(split_line[16]);

            call_prices.push((call_bid + call_ask) / 2.0);
            put_prices.push((put_bid + put_ask) / 2.0);
        }
    }

    return ({
        spot,
        call_prices,
        call_strikes,
        put_prices,
        put_strikes,
        years_to_expiry,
        names,
        option_name,
        time,
    });
}