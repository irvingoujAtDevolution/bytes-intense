import sys
import os
bindings_path = os.path.abspath(os.path.join(os.path.dirname(__file__), '..', 'bindings', 'python'))
sys.path.append(bindings_path)
import bytes_intense

arr = bytes_intense.return_vec()
print(arr)
