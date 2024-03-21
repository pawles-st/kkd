from matplotlib import pyplot as plt

#X = [i * 0.001 for i in range(1, 1001)]
X = [i * 0.1 for i in range(1, 12)]
Y1 = []
Y2 = []
Y3 = []
no_blocks = None

with open("result.txt") as results:
    for line in results.readlines():
        data = line.split();
        no_blocks = int(data[3])
        Y1.append(float(data[0]) / no_blocks)
        Y2.append(float(data[1]) / no_blocks)
        Y3.append(float(data[2]) / no_blocks)

#print(Y1)
#print(Y2)
#print(Y3)
plt.plot(X, Y1)
plt.plot(X, Y2)
plt.plot(X, Y3)
plt.show()

