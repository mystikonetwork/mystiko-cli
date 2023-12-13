import argparse
import boto3

DEFAULT_S3_BUCKET = 'static.mystiko.network'
DEFAULT_S3_PREFIX = 'cli'
DEFAULT_AWS_REGION = 'us-east-1'


def publish_index(git_revision, aws_region, s3_bucket, s3_prefix):
    client = boto3.client('s3', region_name=aws_region)
    s3_path = f'{s3_prefix}/latest'
    print(f'Setting s3://{s3_bucket}/{s3_path} to {git_revision}')
    client.put_object(
        Body=git_revision,
        Bucket=s3_bucket,
        Key=s3_path,
        ContentType='text/plain',
        ACL='public-read',
        CacheControl='no-cache',
    )
    print(f'Done setting s3://{s3_bucket}/{s3_path} to {git_revision}')


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Publish a binary to S3')
    parser.add_argument('git_revision', help='Git revision of the binary')
    parser.add_argument('--aws_region', default=DEFAULT_AWS_REGION, help='AWS region to use')
    parser.add_argument('--s3_bucket', default=DEFAULT_S3_BUCKET, help='S3 bucket to publish to')
    parser.add_argument('--s3_prefix', default=DEFAULT_S3_PREFIX, help='S3 prefix to publish to')
    args = parser.parse_args()
    publish_index(args.git_revision, args.aws_region, args.s3_bucket, args.s3_prefix)