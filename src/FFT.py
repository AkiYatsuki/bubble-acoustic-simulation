import numpy as np
import matplotlib.pyplot as plt

dt = 1.0e-5   # 时间步长
R = np.loadtxt("bubble.txt")

# 去掉直流分量
signal = R - R.mean()

# FFT
fft = np.fft.rfft(signal)
freqs = np.fft.rfftfreq(len(signal), dt)

# 找主峰
idx = np.argmax(np.abs(fft))
f = freqs[idx]
T = 1.0 / f

print("Frequency:", f, "Hz")
print("Period:", T, "s")
print("Mean radius:", R.mean()*1.0e3, "mm")
print("factor f * R", f * R.mean())

# 画频谱
plt.plot(freqs, np.abs(fft))
plt.xlim(0, 5000)
plt.xlabel("Frequency (Hz)")
plt.ylabel("Amplitude")
plt.show()