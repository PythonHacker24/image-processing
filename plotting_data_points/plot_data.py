import matplotlib.pyplot as plt 

plt.style.use('dark_background')

with open('./dataset_handwritten/0_database.txt', 'r') as file:
    for line in file:
        activation_value = line.split(',')
        pixel_number = []
        for i in range(0, len(activation_value)):
            pixel_number.append(i)
        plt.plot(pixel_number, activation_value)

plt.xlabel("linear pixel plot")
plt.ylabel("activation value")

plt.show()
