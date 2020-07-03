import sys
import time
from os import path
from pkgutil import iter_modules
from importlib import import_module


if __name__ == '__main__':
    if len(sys.argv) < 3:
        print('\nUsage: python3 -m src [problem number] [filename (without extension)]')
        print('For example: `python3 -m src 16 v1`\n')
        sys.exit()

    try:
        problems_path = path.join('src', 'problems')
        problem_prefix = sys.argv[1].rjust(4, '0')
        problem_module_name = next(
            name for _, name, _ in iter_modules([problems_path]) if name.startswith(problem_prefix)
        )
        module = import_module(f'src.problems.{problem_module_name}.{sys.argv[2]}')

        print('--------------------------------------------------------------\n')
        now = time.time()
        module.run()
        elapsed = time.time() - now
        print(f'\n~ Duration: %.9f seconds ~' % elapsed)
        print('\n--------------------------------------------------------------')

    except Exception as e:
        print('\nError while loading/running the problem!')
        print(e, '\n')
