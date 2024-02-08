import init, { OptionDir, implied_vol } from "./pkg/simd_vol.js";
import { AAPL_DATA, SPY_DATA } from "./js/data.js";
import { plot2D, plot3D } from "./js/plot.js";
import { parseOptionData } from "./js/parse.js";

var dataType = "SPY";

var spot;
var call_prices;
var put_prices;
var call_strikes;
var put_strikes;
var years_to_expiry;
var time;
var interest_rate = 0.01;
var dividend_yield = 0.0;
var isCall = true;
var plotType = "mesh3d";
var shouldPlot2D = false;
var option_name = ""

// Initialize WASM and set the initial graph
init().then(() => {
    setOptionData();
    update(true);
});

function setOptionData() {
    let selectedData = SPY_DATA;

    if (dataType === "AAPL") {
        selectedData = AAPL_DATA;
    }

    spot = selectedData.spot;
    call_prices = selectedData.call_prices;
    put_prices = selectedData.put_prices;
    call_strikes = selectedData.call_strikes;
    put_strikes = selectedData.put_strikes;
    years_to_expiry = selectedData.years_to_expiry;
    time = selectedData.time;
    option_name = selectedData.option_name;
}

function handleDataTypeChange() {
    var selectedValue = document.getElementById("dataSelect").value;
    dataType = selectedValue;

    setOptionData();
    update(true);
}

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
    update(true);
}

function handleDividendYieldChange() {
    var selectedValue = document.getElementById("dividendYield").value;
    dividend_yield = parseInt(selectedValue, 10) / 100.0;
    update(true);
}

function handleOptionFileChange() {
    let fr = new FileReader();
    fr.onload = function () {
        const data = parseOptionData(fr.result);

        spot = data.spot;
        call_prices = data.call_prices;
        put_prices = data.put_prices;
        call_strikes = data.call_strikes;
        put_strikes = data.put_strikes;
        years_to_expiry = data.years_to_expiry;
        time = data.time;
        option_name = data.option_name;
        update(true);
    }

    fr.readAsText(this.files[0]);
}

// Responds to user changing volatility surface inputs
document.getElementById("dataSelect").addEventListener("change", handleDataTypeChange);
document.getElementById("optionSelect").addEventListener("change", handleOptionTypeChange);
document.getElementById("interestRate").addEventListener("input", handleInterestRateChange);
document.getElementById("dividendYield").addEventListener("input", handleDividendYieldChange);
document.getElementById("plotType").addEventListener("change", handlePlotTypeChange);
document.getElementById("see2DButton").addEventListener("click", toggle2D);
document.getElementById('inputfile').addEventListener('change', handleOptionFileChange);

function toggle2D() {
    shouldPlot2D = !shouldPlot2D;
    document.getElementById("see2DButton").textContent = shouldPlot2D ? "Hide Plots" : "View Implied Vol vs Strike Plots";

    if (!shouldPlot2D) {
        document.getElementById("info2d").textContent = '';
    }

    update(shouldPlot2D);
}

function update(shouldUpdate2D = false) {
    document.getElementById("interestRateText").textContent = `Interest Rate: ${Math.round(interest_rate * 100)}%`;
    document.getElementById("dividendYieldText").textContent = `Dividend Yield: ${Math.round(dividend_yield * 100)}%`;

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
        20,
        0.0001
    );

    plot3D(option_name, isCall ? call_impl_vol : put_impl_vol, spot, strikes, years_to_expiry, time, plotType);

    if (shouldPlot2D && shouldUpdate2D) {
        plot2D(call_strikes, call_impl_vol, put_strikes, years_to_expiry, put_impl_vol);
    }
}


