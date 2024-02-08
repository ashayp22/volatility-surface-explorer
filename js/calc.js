// Extracts x and y coordinates from the strikes and implied volatility
// based on a specific years_to_expiry
export function get2DFromImpliedVolatility(strikes, impl_vol, years_to_expiry, time) {
    const x = [];
    const y = [];

    // Normalize all values
    for (let i = 0; i < strikes.length; i++) {
        // Skip options with 0 implied volatility
        if (years_to_expiry[i] != time || impl_vol[i] < 0.001) {
            continue;
        }
        x.push(strikes[i]);
        y.push(impl_vol[i])
    }

    return { x, y };
}

export function roundToDecimalPlaces(num, places = 2) {
    const divis = Math.pow(10, places);
    return (Math.round(num * divis)) / divis
}

// Extracts 3D points from strikes, implied volatility, and years to expiry
export function get3DFromImpliedVolatility(strikes, impl_vol, years_to_expiry) {
    const x = []
    const y = []
    const z = []

    const seen = {};

    // Normalize all values
    for (let i = 0; i < strikes.length; i++) {
        // Skip options with 0 implied volatility
        if (impl_vol[i] < 0.0001) {
            continue;
        }

        const roundedX = roundToDecimalPlaces(strikes[i]);
        const roundedY = roundToDecimalPlaces(years_to_expiry[i] * 365);
        const roundedZ = roundToDecimalPlaces(impl_vol[i]);

        if (`${roundedX}, ${roundedY}` in seen) {
            // Skip strikes and years to expiry that we've already seen
            continue;
        } else {
            seen[`${roundedX}, ${roundedY}`] = i;
        }

        x.push((roundedX).toString());
        y.push((roundedY).toString());
        z.push((roundedZ).toString());
    }
    return { x, y, z };
}