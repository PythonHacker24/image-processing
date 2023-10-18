file_name = "./dataset_handwritten/0_database.txt"

with open(file_name, 'r') as input_file:
    lines = input_file.readlines()

with open(file_name, 'w') as output_file:
    for line in lines:
        cleaned_line = line.rstrip()
        output_file.write(cleaned_line + "\n")
