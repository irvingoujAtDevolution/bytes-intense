
import sys
import os
bindings_path = os.path.abspath(os.path.join(os.path.dirname(__file__), '..', 'bindings', 'python'))
sys.path.append(bindings_path)
import bytes_intense

arr = bytes([1, 2, 3, 4, 5])
bytes_intense.push_some_data(arr)
print(arr)
