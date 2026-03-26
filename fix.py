import re
with open('src/parsers/fluentbit_config.rs', 'r', encoding='utf-8') as f:
    lines = f.readlines()
new_lines = []
for line in lines:
    if 'Environment' in line and 'NewLine' in line:
        new_lines.append('/// ' + '`' + '\n')
        new_lines.append('/// Parsed Fluentbit configuration.\n')
    else:
        new_lines.append(line)
with open('src/parsers/fluentbit_config.rs', 'w', encoding='utf-8') as f:
    f.writelines(new_lines)
print('Fixed')
