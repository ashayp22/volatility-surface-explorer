import init, { OptionDir, implied_vol } from "./pkg/simd_vol.js";
import { AAPL_DATA } from "./js/data.js";
import { plot2D, plot3D } from "./js/plot.js";

var spot;
var call_prices;
var put_prices;
var call_strikes;
var put_strikes;
var years_to_expiry;
var names;
var call_prices;
var interest_rate = 0.01;
var dividend_yield = 0.0;
var prev_vol = [];
var isCall = true;

init().then(() => {
    call_prices = AAPL_DATA.call_prices;
    put_prices = AAPL_DATA.put_prices;
    call_strikes = AAPL_DATA.call_strikes;
    put_strikes = AAPL_DATA.put_strikes;
    years_to_expiry = AAPL_DATA.years_to_expiry;
    names = AAPL_DATA.names;
    prev_vol = Array(call_prices.length).fill(1.0);
    spot = AAPL_DATA.spot;

    update();
    // // plot2D(call_prices, put_prices, call_strikes, put_strikes, years_to_expiry, prev_vol, interest_rates, dividend_yields, spots);
});

function handleOptionTypeChange() {
    var selectedValue = document.getElementById("optionSelect").value;
    isCall = selectedValue === "Call";
    update();
}

function handleInterestRateChange() {
    var selectedValue = document.getElementById("interestRate").value;
    interest_rate = parseInt(selectedValue, 10) / 100.0;
    update();
}

function handleDividendYieldChange() {
    var selectedValue = document.getElementById("dividendYield").value;
    dividend_yield = parseInt(selectedValue, 10) / 100.0;
    update();
}

document.getElementById("optionSelect").addEventListener("change", handleOptionTypeChange);
document.getElementById("interestRate").addEventListener("input", handleInterestRateChange);
document.getElementById("dividendYield").addEventListener("input", handleDividendYieldChange);

function update() {
    document.getElementById("interestRateText").textContent = `Interest Rate: ${interest_rate}%`;
    document.getElementById("dividendYieldText").textContent = `Dividend Yield: ${dividend_yield}%`;

    const n = years_to_expiry.length;
    const interest_rates = Array(n).fill(interest_rate);
    const dividend_yields = Array(n).fill(dividend_yield);
    const spots = Array(n).fill(spot);
    const strikes = isCall ? call_strikes : put_strikes;
    const prices = isCall ? call_prices : put_prices;

    prev_vol = implied_vol(
        isCall ? OptionDir.CALL : OptionDir.PUT,
        prices,
        spots,
        strikes,
        interest_rates,
        dividend_yields,
        years_to_expiry,
        prev_vol,
        20,
        0.0001
    );

    plot3D(prev_vol, names, spot, strikes, years_to_expiry);
}


