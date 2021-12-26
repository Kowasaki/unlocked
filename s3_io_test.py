import boto3
import concurrent.futures
import os
import ray
import time

from multiprocessing import Pool
from functools import partial

def upload_to_s3(bucket_name, local_path, s3_path):
    s3 = boto3.client('s3')
    s3.upload_file(local_path, bucket_name, s3_path)

def download_from_s3(bucket_name, s3_path, local_path):
    s3 = boto3.client('s3')
    s3.download_file(bucket_name, s3_path, local_path)

@ray.remote
def ray_upload_to_s3(bucket_name, local_path, s3_path):
    s3 = boto3.client('s3')
    s3.upload_file(local_path, bucket_name, s3_path)

@ray.remote
def ray_download_from_s3(bucket_name, s3_path, local_path):
    s3 = boto3.client('s3')
    s3.download_file(bucket_name, s3_path, local_path)

def p_upload_to_s3(bucket_name, local_dir, obj_name):
    s3 = boto3.client('s3')
    s3.upload_file(os.path.join(local_dir, obj_name), bucket_name, obj_name)

def p_download_from_s3(bucket_name, obj_name, local_dir):
    s3 = boto3.client('s3')
    s3.download_file(bucket_name, obj_name, os.path.join(local_dir, obj_name))

def ray_upload():
    ray.init()
    bucket_name = "data-testing-ground"
    local_dir = "./tests/test_artifacts"
    files = os.listdir(local_dir)

    t_start = time.time()
    res = [ray.get(ray_upload_to_s3.remote(bucket_name, os.path.join(local_dir, f), f)) for f in files]
    print(f"Uploaded {len(files)} files in {time.time() - t_start:.3f} seconds")
    print(res)

def thread_upload():
    bucket_name = "data-testing-ground"
    local_dir = "./tests/test_artifacts"
    files = os.listdir(local_dir)

    with concurrent.futures.ThreadPoolExecutor() as executor:
        t_start = time.time()
        futures = [executor.submit(upload_to_s3, bucket_name, os.path.join(local_dir, f), f) for f in files]

        res = [future.result() for future in concurrent.futures.as_completed(futures)]
        print(f"Uploaded {len(files)} files in {time.time() - t_start:.3f} seconds")
        print(res)

def process_upload():
    bucket_name = "data-testing-ground"
    local_dir = "./tests/test_artifacts"
    files = os.listdir(local_dir)

    func = partial(p_upload_to_s3, bucket_name, local_dir)
    with Pool() as pool:
        t_start = time.time()
        res = pool.map(func, files)
        print(f"Uploaded {len(files)} files in {time.time() - t_start:.3f} seconds")
        print(res)

if __name__ == "__main__":
    # ray_upload()
    # thread_upload()
    process_upload()
