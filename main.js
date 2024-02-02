import init, { OptionDir, implied_vol } from "./pkg/simd_vol.js";
import { AAPL_DATA } from "./js/data.js";

function get2DFromImpliedVolatility(strikes, impl_vol, years_to_expiry, time) {
    const x = [];
    const y = [];


    // Normalize all values
    for (let i = 0; i < strikes.length; i++) {
        if (years_to_expiry[i] != time || impl_vol[i] < 0.001) {
            continue;
        }
        x.push(strikes[i]);
        y.push(impl_vol[i])
    }

    return { x, y };
}

function roundTo2DecimalPlaces(num) {
    return (Math.round(num * 100)) / 100
}

function get3DFromImpliedVolatility(strikes, impl_vol, years_to_expiry) {
    const x = []
    const y = []
    const z = []

    const seenX = new Set();
    const seenY = new Set();

    // Normalize all values
    for (let i = 0; i < strikes.length; i++) {
        if (impl_vol[i] < 0.0001) {
            continue;
        }

        const roundedX = roundTo2DecimalPlaces(strikes[i]);
        const roundedY = roundTo2DecimalPlaces(years_to_expiry[i] * 365);
        const roundedZ = roundTo2DecimalPlaces(impl_vol[i]);

        if (seenX.has(roundedX) && seenY.has(roundedY)) {
            continue;
        }

        // seenX.add(roundedX);
        // seenY.add(roundedY)

        x.push((roundedX).toString());
        y.push((roundedY).toString());
        z.push((roundedZ).toString());
    }

    return { x, y, z };
}

function plot2D(call_prices, put_prices, call_strikes, put_strikes, years_to_expiry, prev_vol, interest_rates, dividend_yields, spots) {
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
            type: 'scatter',
        };

        const put = {
            x: put_x,
            y: put_y,
            mode: 'markers',
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

function plot3D(call_prices, put_prices, call_strikes, put_strikes, years_to_expiry, prev_vol, interest_rates, dividend_yields, spots) {
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
            20,
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
            20,
            0.0001
        )

    const { x: callX, y: callY, z: callZ } = get3DFromImpliedVolatility(call_strikes, call_impl_vol, years_to_expiry);
    const { x: putX, y: putY, z: putZ } = get3DFromImpliedVolatility(put_strikes, put_impl_vol, years_to_expiry);

    console.log(call_strikes);
    console.log(years_to_expiry);
    console.log(call_impl_vol);

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
        'intensity': [...callZ],
        x: [...callX],
        y: [...callY],
        z: [...callZ],
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
        title: `Volatility Surface Explorer for APPL Current Price ${spots[0]}`,
        scene: {
            camera: { eye: { x: -1.5, y: -1.5, z: 1 } },
            xaxis: { title: 'Strike Price' },
            yaxis: { title: 'Years to Expiry' },
            zaxis: { title: 'Implied Volatility' }
        },
        autosize: false,
        width: 800,
        height: 800,
        margin: {
            l: 65,
            r: 50,
            b: 65,
            t: 90,
        }
    };

    Plotly.newPlot('info3d', data, layout);
}

init().then(() => {
    const call_prices = AAPL_DATA.call_prices;
    const put_prices = AAPL_DATA.put_prices;
    const call_strikes = AAPL_DATA.call_strikes;
    const put_strikes = AAPL_DATA.put_strikes;
    const years_to_expiry = AAPL_DATA.years_to_expiry;
    const n = years_to_expiry.length;

    const prev_vol = Array(n).fill(1.0);
    const interest_rates = Array(n).fill(0.01);
    const dividend_yields = Array(n).fill(0.0);
    const spots = Array(n).fill(AAPL_DATA.spot);

    plot2D(call_prices, put_prices, call_strikes, put_strikes, years_to_expiry, prev_vol, interest_rates, dividend_yields, spots);
    plot3D(call_prices, put_prices, call_strikes, put_strikes, years_to_expiry, prev_vol, interest_rates, dividend_yields, spots);

    // var scene = new THREE.Scene();
    // var camera = new THREE.PerspectiveCamera(60, 1, 1, 1000);
    // camera.position.setScalar(150);
    // var renderer = new THREE.WebGLRenderer({
    //     antialias: true
    // });
    // var canvas = renderer.domElement;
    // document.body.appendChild(canvas);

    // var controls = new THREE.OrbitControls(camera, canvas);

    // var light = new THREE.DirectionalLight(0xffffff, 1.5);
    // light.position.setScalar(100);
    // scene.add(light);
    // scene.add(new THREE.AmbientLight(0xffffff, 0.5));

    // var geom = new THREE.BufferGeometry().setFromPoints(points3d);
    // var cloud = new THREE.Points(
    //     geom,
    //     new THREE.PointsMaterial({ color: 0x99ccff, size: 2 })
    // );
    // scene.add(cloud);

    // // Triangulate x and z 
    // var indexDelaunay = Delaunator.from(
    //     points3d.map(v => {
    //         return [v.x, v.z];
    //     })
    // );

    // var meshIndex = []; // Use the delaunay index to create a flat surface
    // for (let i = 0; i < indexDelaunay.triangles.length; i++) {
    //     meshIndex.push(indexDelaunay.triangles[i]);
    // }

    // geom.setIndex(meshIndex); // Apply three.js index to the existing geometry
    // geom.computeVertexNormals();
    // var mesh = new THREE.Mesh(
    //     geom, // Re-use the existing geometry
    //     new THREE.MeshLambertMaterial({ color: "purple", wireframe: true })
    // );
    // scene.add(mesh);

    // var gui = new dat.GUI();
    // gui.add(mesh.material, "wireframe");

    // render();

    // function resize(renderer) {
    //     const canvas = renderer.domElement;
    //     const width = canvas.clientWidth;
    //     const height = canvas.clientHeight;
    //     const needResize = canvas.width !== width || canvas.height !== height;
    //     if (needResize) {
    //         renderer.setSize(width, height, false);
    //     }
    //     return needResize;
    // }

    // function render() {
    //     if (resize(renderer)) {
    //         camera.aspect = canvas.clientWidth / canvas.clientHeight;
    //         camera.updateProjectionMatrix();
    //     }
    //     renderer.render(scene, camera);
    //     requestAnimationFrame(render);
    // }
});

