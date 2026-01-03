use std::f32::consts::PI;

/// Poly6 kernel for density calculation (2D version).
/// Formula: W(r,h) = (4/(π*h^8)) * (h^2 - r^2)^3
#[inline(always)]
pub fn poly6_kernel(dist_sq: f32, h: f32) -> f32 {
    let h2 = h * h;
    if dist_sq < h2 {
        let h8 = h.powi(8);
        let coeff = 4.0 / (PI * h8);
        let diff = h2 - dist_sq;
        coeff * diff * diff * diff
    } else {
        0.0
    }
}

/// Spiky kernel gradient magnitude for pressure force calculation (2D version).
/// Returns |∇W| = (10/(π h^5)) * (h - r)^3 (positive magnitude)
#[inline(always)]
pub fn spiky_kernel_gradient(dist: f32, h: f32) -> f32 {
    if dist < h {
        let h5 = h.powi(5);
        let coeff = 10.0 / (PI * h5);
        let diff = h - dist;
        coeff * diff * diff * diff
    } else {
        0.0
    }
}

/// Viscosity kernel laplacian for viscosity force calculation (2D version).
/// Formula: ∇²W(r,h) = (40/(π h^6)) * (h - r)
#[inline(always)]
pub fn viscosity_laplacian(dist: f32, h: f32) -> f32 {
    if dist < h {
        let h6 = h.powi(6);
        let coeff = 40.0 / (PI * h6);
        let diff = h - dist;
        coeff * diff
    } else {
        0.0
    }
}
