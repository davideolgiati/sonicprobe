"""
AI generated, reviewed by me
not the best but fine for now
"""

import numpy as np
from scipy.signal.windows import kaiser

def improved_kaiser_design_2x():    
    upsample_factor = 2
    num_taps = 24
    beta = 6.5
    
    n = np.arange(num_taps)
    mid = (num_taps - 1) / 2.0

    cutoff_normalized = 1.0 / upsample_factor
    h = np.sinc(2 * cutoff_normalized * (n - mid))

    window = kaiser(num_taps, beta)
    h *= window

    h /= np.sum(h)
    h *= upsample_factor

    quantization_bits = 16
    h_quantized = np.round(h * (1 << quantization_bits)) / (1 << quantization_bits)

    coeffs = []
    for phase in range(upsample_factor):
        phase_coeffs = h_quantized[phase::upsample_factor]
        coeffs.append(phase_coeffs)
    
    return coeffs, h_quantized

def print_coefficients_2x():
    coeffs, _ = improved_kaiser_design_2x()
    
    print("[")
    for phase in coeffs:
        print("\t[" + ", ".join(f"{c:.13f}" for c in phase) + "],")
    print("]")
    
    return coeffs

if __name__ == "__main__":
    print_coefficients_2x()

