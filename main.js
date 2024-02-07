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
var time;
var plotType = "mesh3d";
var shouldPlot2D = false;

init().then(() => {
    call_prices = AAPL_DATA.call_prices;
    put_prices = AAPL_DATA.put_prices;
    call_strikes = AAPL_DATA.call_strikes;
    put_strikes = AAPL_DATA.put_strikes;
    years_to_expiry = AAPL_DATA.years_to_expiry;
    names = AAPL_DATA.names;
    prev_vol = Array(call_prices.length).fill(1.0);
    spot = AAPL_DATA.spot;
    time = AAPL_DATA.time;

    update();
    // // plot2D(call_prices, put_prices, call_strikes, put_strikes, years_to_expiry, prev_vol, interest_rates, dividend_yields, spots);
});

function handleOptionTypeChange() {
    var selectedValue = document.getElementById("optionSelect").value;
    isCall = selectedValue === "Call";
    update();
}

function handlePlotTypeChange() {
    var selectedValue = document.getElementById("plotType").value;
    plotType = selectedValue;
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
document.getElementById("plotType").addEventListener("change", handlePlotTypeChange);
document.getElementById("see2DButton").addEventListener("click", toggle2D);

function toggle2D() {
    shouldPlot2D = !shouldPlot2D;
    document.getElementById("see2DButton").textContent = shouldPlot2D ? "Hide 2D Plots" : "Show 2D Plots";
    update();
}

function update() {
    document.getElementById("interestRateText").textContent = `Interest Rate: ${interest_rate * 100}%`;
    document.getElementById("dividendYieldText").textContent = `Dividend Yield: ${dividend_yield * 100}%`;

    const n = years_to_expiry.length;
    const interest_rates = Array(n).fill(interest_rate);
    const dividend_yields = Array(n).fill(dividend_yield);
    const spots = Array(n).fill(spot);
    const strikes = isCall ? call_strikes : put_strikes;

    let call_impl_vol = implied_vol(
        OptionDir.CALL,
        call_prices,
        spots,
        call_strikes,
        interest_rates,
        dividend_yields,
        years_to_expiry,
        prev_vol,
        20,
        0.0001
    );

    let put_impl_vol = implied_vol(
        OptionDir.PUT,
        put_prices,
        spots,
        put_strikes,
        interest_rates,
        dividend_yields,
        years_to_expiry,
        prev_vol,
        20,
        0.0001
    );

    prev_vol = isCall ? call_impl_vol : put_impl_vol;

    plot3D(prev_vol, spot, strikes, years_to_expiry, time, plotType);

    if (shouldPlot2D) {
        plot2D(call_strikes, call_impl_vol, put_strikes, years_to_expiry, put_impl_vol);
    }
}


