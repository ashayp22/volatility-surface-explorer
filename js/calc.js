export function get2DFromImpliedVolatility(strikes, impl_vol, years_to_expiry, time) {
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

export function roundTo2DecimalPlaces(num) {
    return (Math.round(num * 100)) / 100
}

export function get3DFromImpliedVolatility(names, strikes, impl_vol, years_to_expiry) {
    const x = []
    const y = []
    const z = []

    const seen = {};

    // Normalize all values
    for (let i = 0; i < strikes.length; i++) {
        if (impl_vol[i] < 0.0001) {
            continue;
        }

        const roundedX = roundTo2DecimalPlaces(strikes[i]);
        const roundedY = roundTo2DecimalPlaces(years_to_expiry[i] * 365);
        const roundedZ = roundTo2DecimalPlaces(impl_vol[i]);

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