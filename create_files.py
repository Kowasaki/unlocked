import numpy as np
import os

# Create 100 10 MB files (1GB total)
for i in range(100):
    np.ones((1000, 1250)).tofile(f"./tests/test_artifacts/{i}.npy")
    num_bytes = os.path.getsize(f"./tests/test_artifacts/{i}.npy")
    print(f"{i}: {num_bytes/1000000} MB")