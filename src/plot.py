import numpy as np
import matplotlib.pyplot as plt

dt = 1.0e-5

R = np.loadtxt("bubble.txt")

t = np.arange(len(R)) * dt

plt.figure(figsize = (8, 4))
plt.plot(t * 1e3, R * 1e3)
plt.xlabel("Time (ms)")
plt.ylabel("Bubble radius R (mm)")
plt.title("Radial oscillation of an air bubble (Rayleigh–Plesset)")
plt.grid(True)
plt.tight_layout()
plt.show()
