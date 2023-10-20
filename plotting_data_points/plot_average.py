import matplotlib.pyplot as plt 

plt.style.use('dark_background')

activation_average = []
sample_count = []

with open('./0_vs_0_delta.txt', 'r') as file:
    count = 0;
    for line in file:
        sample_count.append(count)
        count += 1
        
    for line in file:
        digits_split = line.split(',')
        sum_value = 0 
        for digit in digits_split:
            sum_value += digit
        average_value = sum_value / len(digits_split)
        activation_average.append(average_value)

plt.plot(sample_count, activation_average)

plt.xlabel("linear pixel plot")
plt.ylabel("activation value")

plt.show()
