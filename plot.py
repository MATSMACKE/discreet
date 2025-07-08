import numpy as np
import matplotlib.pyplot as plt

xvals = 100
yvals = 100

vals = np.fromfile("testing/MyMesh", dtype='>f8')
vals = np.reshape(vals, (xvals, yvals))

x = np.linspace(0, 6, xvals)
t = np.linspace(0, 3, yvals)

X, T = np.meshgrid(x, t)


plt.pcolormesh(X, T, vals)
plt.clim([-1, 1])
plt.colorbar()

plt.savefig("plot.png")

# plt.show(block=True)
