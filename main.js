import init, { OptionDir, implied_vol } from "./pkg/simd_vol.js";
import { AAPL_DATA } from "./js/data.js";

init().then(() => {

    const call_prices = AAPL_DATA.call_prices;
    const call_strikes = AAPL_DATA.call_strikes;
    const years_to_expiry = AAPL_DATA.years_to_expiry;
    const n = years_to_expiry.length;

    const prev_vol = Array(n).fill(1.0);
    const interest_rates = Array(n).fill(0.01);
    const dividend_yields = Array(n).fill(0.0);
    const spots = Array(n).fill(AAPL_DATA.spot);

    console.error(`Weird Data: price: ${call_prices[958]}, strike: ${call_strikes[958]}, years: ${years_to_expiry[958]}, spot: ${spots[958]}`);

    const impl_vol =
        implied_vol(
            OptionDir.CALL,
            call_prices,
            spots,
            call_strikes,
            interest_rates,
            dividend_yields,
            years_to_expiry,
            prev_vol,
            5,
            0.0001
        )

    console.error(impl_vol);
    console.error(impl_vol[958]);

    const points3d = [];
    const scale_factor = 20;
    let max_x = 0;
    let max_y = 0;
    let max_z = 0;

    // Determine max values
    for (let i = 0; i < n; i++) {
        max_x = Math.max(max_x, call_strikes[i]);
        max_y = Math.max(max_y, impl_vol[i]);
        max_z = Math.max(max_z, years_to_expiry[i]);

        if (max_y === Infinity) {
            // console.error(i);
        }
    }

    console.log(max_x);
    console.log(max_z * 365);
    console.log(max_y);

    // Normalize all values
    for (let i = 0; i < n; i++) {
        points3d.push(new THREE.Vector3(scale_factor * call_strikes[i] / max_x, scale_factor * impl_vol[i] / max_y, scale_factor * years_to_expiry[i] / max_z));
    }


    var scene = new THREE.Scene();
    var camera = new THREE.PerspectiveCamera(60, 1, 1, 1000);
    camera.position.setScalar(150);
    var renderer = new THREE.WebGLRenderer({
        antialias: true
    });
    var canvas = renderer.domElement;
    document.body.appendChild(canvas);

    var controls = new THREE.OrbitControls(camera, canvas);

    var light = new THREE.DirectionalLight(0xffffff, 1.5);
    light.position.setScalar(100);
    scene.add(light);
    scene.add(new THREE.AmbientLight(0xffffff, 0.5));

    var size = { x: 200, y: 200 };
    // var pointsCount = 50;
    // var points3d = [];
    // for (let i = 0; i < pointsCount; i++) {
    //     let x = THREE.Math.randFloatSpread(size.x);
    //     let z = THREE.Math.randFloatSpread(size.y);
    //     let y = noise.perlin2(x / size.x * 5, z / size.y * 5) * 50;
    //     points3d.push(new THREE.Vector3(x, y, z));
    // }

    var geom = new THREE.BufferGeometry().setFromPoints(points3d);
    var cloud = new THREE.Points(
        geom,
        new THREE.PointsMaterial({ color: 0x99ccff, size: 2 })
    );
    scene.add(cloud);

    // triangulate x, z
    var indexDelaunay = Delaunator.from(
        points3d.map(v => {
            return [v.x, v.z];
        })
    );

    var meshIndex = []; // delaunay index => three.js index
    for (let i = 0; i < indexDelaunay.triangles.length; i++) {
        meshIndex.push(indexDelaunay.triangles[i]);
    }

    geom.setIndex(meshIndex); // add three.js index to the existing geometry
    geom.computeVertexNormals();
    var mesh = new THREE.Mesh(
        geom, // re-use the existing geometry
        new THREE.MeshLambertMaterial({ color: "purple", wireframe: true })
    );
    scene.add(mesh);

    var gui = new dat.GUI();
    gui.add(mesh.material, "wireframe");

    render();

    function resize(renderer) {
        const canvas = renderer.domElement;
        const width = canvas.clientWidth;
        const height = canvas.clientHeight;
        const needResize = canvas.width !== width || canvas.height !== height;
        if (needResize) {
            renderer.setSize(width, height, false);
        }
        return needResize;
    }

    function render() {
        if (resize(renderer)) {
            camera.aspect = canvas.clientWidth / canvas.clientHeight;
            camera.updateProjectionMatrix();
        }
        renderer.render(scene, camera);
        requestAnimationFrame(render);
    }
});

