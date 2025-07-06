import numpy as np
import matplotlib.pyplot as plt

vals = np.fromfile("testing/MyMesh", dtype='>f8')
vals = np.reshape(vals, (1000, 1000))

x = np.linspace(0, 6, 1000)
t = np.linspace(0, 3, 1000)

X, T = np.meshgrid(x, t)


plt.pcolormesh(X, T, vals)
plt.clim([-1, 1])
plt.colorbar()

plt.savefig("plot.png")

# plt.show(block=True)
