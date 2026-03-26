with open("src/parsers/fluentbit_config.rs", "r") as f:
    lines = f.readlines()
new_lines = []
i = 0
while i < len(lines):
    stripped = lines[i].rstrip()
    if stripped == "/// `" or stripped == "/// ``":
        # Replace with three backticks line
        new_lines.append("/// ```\n")
        i += 1
        continue
    new_lines.append(lines[i])
    i += 1
with open("src/parsers/fluentbit_config.rs", "w") as f:
    f.writelines(new_lines)
print("Fixed backticks")
