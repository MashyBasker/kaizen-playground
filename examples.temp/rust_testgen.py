from kaizen.generator.unit_test import UnitTestGenerator

source="/path/to/source/code"
output="/path/to/generated/tests"
generator = UnitTestGenerator()
_, usage = generator.generate_tests(file_path=source, output_path=output)
print(usage)
