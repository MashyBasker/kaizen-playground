from kaizen.generator.unit_test import UnitTestGenerator
from kaizen.llms.provider import LLMProvider

source="testdata/calculator-rust/src/main.rs"
output="output/"
generator = UnitTestGenerator()
_, usage = generator.generate_tests(file_path=source, output_path=output)
print(usage)
