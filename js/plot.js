import { get3DFromImpliedVolatility, get2DFromImpliedVolatility } from "./calc.js";

export function plot2D(call_prices, put_prices, call_strikes, put_strikes, years_to_expiry, prev_vol, interest_rates, dividend_yields, spots) {
    const times = [0.030136986, 0.115068495, 0.18630137, 0.27671233, 0.35890412, 0.61095893, 1.1150684, 2.1150684]
    const titles = ["Expiry: Dec 2013", "Expiry: Jan 2014", "Expiry: Feb 2014", "Expiry: Mar 2014", "Expiry: Apr 2014", "Expiry: Jul 2014", "Expiry: Jan 2015", "Expiry: Jan 2016"]

    for (let i = 0; i < times.length; i++) {
        const call_impl_vol =
            implied_vol(
                OptionDir.CALL,
                call_prices,
                spots,
                call_strikes,
                interest_rates,
                dividend_yields,
                years_to_expiry,
                prev_vol,
                25,
                0.0001
            )

        const put_impl_vol =
            implied_vol(
                OptionDir.PUT,
                put_prices,
                spots,
                put_strikes,
                interest_rates,
                dividend_yields,
                years_to_expiry,
                prev_vol,
                25,
                0.0001
            )

        const { x: call_x, y: call_y } = get2DFromImpliedVolatility(call_strikes, call_impl_vol, years_to_expiry, times[i]);
        const { x: put_x, y: put_y } = get2DFromImpliedVolatility(put_strikes, put_impl_vol, years_to_expiry, times[i]);

        const call = {
            x: call_x,
            y: call_y,
            mode: 'markers',
            name: "Call",
            type: 'scatter',
        };

        const put = {
            x: put_x,
            y: put_y,
            mode: 'markers',
            name: "Put",
            type: 'scatter',
        };

        var layout = {
            title: titles[i],
            scene: {
                xaxis: { title: 'Strike Price' },
                yaxis: { title: 'Implied Volatility' }
            },
        };

        const data = [call, put];
        Plotly.newPlot(`myDiv${i}`, data, layout);
    }
}

export function plot3D(impl_vol, names, spot, strikes, years_to_expiry) {
    const { x, y, z } = get3DFromImpliedVolatility(names, strikes, impl_vol, years_to_expiry);

    // var data = [{
    //     "mode": "markers",
    //     "type": "scatter3d",
    //     x: [...callX, ...putX],
    //     y: [...callY, ...putY],
    //     z: [...callZ, ...putZ],
    //     'connectgaps': true,
    //     'line': { 'smoothing': '1' },
    //     'contours': { 'coloring': "contour" },
    //     'autocolorscale': false,
    //     "colorscale": [
    //         [0, "rgb(244,236,21)"], [0.3, "rgb(249,210,41)"], [0.4, "rgb(134,191,118)"],
    //         [0.5, "rgb(37,180,167)"], [0.65, "rgb(17,123,215)"], [1, "rgb(54,50,153)"],
    //     ],
    //     "reversescale": true,
    // }];

    // var data = [{
    //     "type": "surface",
    //     x: [...callX],
    //     y: [...callY],
    //     z: [...callZ],
    //     'connectgaps': true,
    //     'line': { 'smoothing': '1' },
    //     'contours': { 'coloring': 'contour' },
    //     'autocolorscale': false,
    //     "colorscale": [
    //         [0, "rgb(244,236,21)"], [0.3, "rgb(249,210,41)"], [0.4, "rgb(134,191,118)"],
    //         [0.5, "rgb(37,180,167)"], [0.65, "rgb(17,123,215)"], [1, "rgb(54,50,153)"],
    //     ],
    //     "reversescale": true,
    // }];

    var data = [{
        "type": "mesh3d",
        'intensity': z,
        x: x,
        y: y,
        z: z,
        'autocolorscale': false,
        "colorscale": [
            [0, "rgb(244,236,21)"], [0.3, "rgb(249,210,41)"], [0.4, "rgb(134,191,118)"], [
                0.5, "rgb(37,180,167)"], [0.65, "rgb(17,123,215)"], [1, "rgb(54,50,153)"],
        ],
        "lighting": {
            "ambient": 1,
            "diffuse": 0.9,
            "fresnel": 0.5,
            "roughness": 0.9,
            "specular": 2
        },
        "flatshading": true,
        "reversescale": true,
    }];

    var layout = {
        title: `Volatility Surface Explorer for APPL Current Price ${spot}`,
        scene: {
            camera: { eye: { x: -1.5, y: -1.5, z: 1 } },
            xaxis: { title: 'Strike Price' },
            yaxis: { title: 'Days to Expiry' },
            zaxis: { title: 'Implied Volatility' }
        },
        autosize: false,
        width: 800,
        height: 500,
        margin: {
            l: 65,
            r: 50,
            b: 65,
            t: 90,
        },
        font: {
            family: 'Noto Sans',
        }
    };

    Plotly.newPlot('info3d', data, layout);
}