import { get3DFromImpliedVolatility, get2DFromImpliedVolatility } from "./calc.js";

export function plot2D(call_strikes, call_impl_vol, put_strikes, years_to_expiry, put_impl_vol) {
    const times = Array.from(new Set(years_to_expiry));

    for (let i = 0; i < times.length; i++) {
        const divId = `info-2d-${i}`;
        var newDiv = document.createElement("div");
        newDiv.id = divId
        document.getElementById("info2d").appendChild(newDiv);

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

        let days = Math.round(times[i] * 365)

        var layout = {
            title: `${days} ${days === 1 ? "Day" : "Days"}`,
            scene: {
                xaxis: { title: 'Strike Price' },
                yaxis: { title: 'Implied Volatility' }
            },
        };

        const data = [call, put];
        Plotly.newPlot(divId, data, layout);
    }
}

export function plot3D(optionName, impl_vol, spot, strikes, years_to_expiry, time, plotType = "mesh3d") {
    const { x, y, z } = get3DFromImpliedVolatility(strikes, impl_vol, years_to_expiry);

    let data = null;

    if (plotType === "mesh3d") {
        data = [{
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
    } else if (plotType === "surface") {
        data = [{
            "type": "surface",
            x: x,
            y: y,
            z: z,
            'connectgaps': true,
            'line': { 'smoothing': '1' },
            'contours': { 'coloring': 'contour' },
            'autocolorscale': false,
            "colorscale": [
                [0, "rgb(244,236,21)"], [0.3, "rgb(249,210,41)"], [0.4, "rgb(134,191,118)"],
                [0.5, "rgb(37,180,167)"], [0.65, "rgb(17,123,215)"], [1, "rgb(54,50,153)"],
            ],
            "reversescale": true,
        }];
    } else if (plotType === "markers") {
        data = [{
            "mode": "markers",
            "type": "scatter3d",
            x: x,
            y: y,
            z: z,
            'connectgaps': true,
            'line': { 'smoothing': '1' },
            'contours': { 'coloring': "contour" },
            'autocolorscale': false,
            "colorscale": [
                [0, "rgb(244,236,21)"], [0.3, "rgb(249,210,41)"], [0.4, "rgb(134,191,118)"],
                [0.5, "rgb(37,180,167)"], [0.65, "rgb(17,123,215)"], [1, "rgb(54,50,153)"],
            ],
            "reversescale": true,
        }];
    } else if (plotType === "lines") {
        data = [{
            "mode": "lines",
            "type": "scatter3d",
            x: x,
            y: y,
            z: z,
            'connectgaps': true,
            'line': { 'smoothing': '1' },
            'contours': { 'coloring': "contour" },
            'autocolorscale': false,
            "colorscale": [
                [0, "rgb(244,236,21)"], [0.3, "rgb(249,210,41)"], [0.4, "rgb(134,191,118)"],
                [0.5, "rgb(37,180,167)"], [0.65, "rgb(17,123,215)"], [1, "rgb(54,50,153)"],
            ],
            "reversescale": true,
        }];
    }

    var layout = {
        title: `${optionName} | Current Price: ${spot} | ${time}`,
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