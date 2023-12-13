import argparse
import boto3
import os

DEFAULT_S3_BUCKET = 'static.mystiko.network'
DEFAULT_S3_PREFIX = 'cli'
DEFAULT_AWS_REGION = 'us-east-1'


def publish_binary(script_path, aws_region, s3_bucket, s3_prefix):
    client = boto3.client('s3', region_name=aws_region)
    script_name = os.path.basename(script_path)
    s3_path = f'{s3_prefix}/scripts/{script_name}'
    upload_args = {'ACL': 'public-read', 'CacheControl': 'no-cache'}
    print(f'Uploading {script_path} to s3://{s3_bucket}/{s3_path} with args {upload_args}')
    client.upload_file(script_path, Bucket=s3_bucket, Key=s3_path, ExtraArgs=upload_args)
    print(f'Uploaded {script_path} to s3://{s3_bucket}/{s3_path}')


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Publish a binary to S3')
    parser.add_argument('script_path', help='Path to the script to publish')
    parser.add_argument('--aws_region', default=DEFAULT_AWS_REGION, help='AWS region to use')
    parser.add_argument('--s3_bucket', default=DEFAULT_S3_BUCKET, help='S3 bucket to publish to')
    parser.add_argument('--s3_prefix', default=DEFAULT_S3_PREFIX, help='S3 prefix to publish to')
    args = parser.parse_args()
    publish_binary(args.script_path, args.aws_region, args.s3_bucket, args.s3_prefix)
