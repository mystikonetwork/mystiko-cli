#!/usr/bin/env bash

set -e

download_binary() {
  local base_url=$1
  local git_revision=$2
  local target=$3
  local binary_name=$4
  local binary_folder=$5
  local binary_url="${base_url}/${git_revision}/${target}/${binary_name}"
  local binary_path="${binary_folder}/${binary_name}"
  echo "Downloading ${binary_url} to ${binary_path}"
  curl -sfLS "${binary_url}" -o "${binary_path}"
  chmod +x "${binary_path}"
  echo "Downloaded ${binary_url} to ${binary_path}"
}

setup_path() {
  local bin_dir=$1
  local export_path="export PATH=\$PATH:${bin_dir}"
  case "${SHELL}" in
    /bin/bash)
      echo "${export_path}" >> "${HOME}/.bashrc"
      source "${HOME}/.bashrc"
      ;;
    /bin/zsh)
      echo "${export_path}" >> "${HOME}/.zshrc"
      ;;
    *)
      echo "Unsupported shell: ${SHELL}"
      source "${HOME}/.bashrc"
      exit 1
      ;;
  esac
}

install() {
  local base_url
  local binary_path
  local git_revision

  while [[ "$#" -gt 0 ]]; do
    case $1 in
      -b|--base-url)
        base_url="$2"
        shift
        ;;
      -p|--binary-path)
        binary_path="$2"
        shift
        ;;
      -r|--git-revision)
        git_revision="$2"
        shift
        ;;
      *)
        echo "Unknown parameter passed: $1"
        exit 1
        ;;
    esac
    shift
  done

  if [ -z "${base_url}" ]; then
    base_url="https://static.mystiko.network/cli"
  fi

  if [ -z "${binary_path}" ]; then
    binary_path="${HOME}/.mystiko/bin"
  fi

  if [ ! -d "${binary_path}" ]; then
    mkdir -p "${binary_path}"
  fi

  if [ -z "${git_revision}" ]; then
    local latest_url="${base_url}/latest"
    echo "Fetching latest version from ${latest_url}"
    git_revision=$(curl -sfLS "${latest_url}")
    echo "Latest version is ${git_revision}"
  fi

  local os
  os=$(uname -s | tr '[:upper:]' '[:lower:]')
  local arch
  arch=$(uname -m | tr '[:upper:]' '[:lower:]')

  case "${os}" in
    linux)
      case "${arch}" in
        x86_64)
          download_binary "${base_url}" "${git_revision}" "x86_64-unknown-linux-gnu" "mystiko" "${binary_path}"
          ;;
        arm64)
          download_binary "${base_url}" "${git_revision}" "aarch64-unknown-linux-gnu" "mystiko" "${binary_path}"
          ;;
        *)
          echo "Unsupported arch: ${arch}"
          exit 1
          ;;
      esac
      ;;
    darwin)
      case "${arch}" in
        x86_64)
          download_binary "${base_url}" "${git_revision}" "x86_64-apple-darwin" "mystiko" "${binary_path}"
          ;;
        arm64)
          download_binary "${base_url}" "${git_revision}" "aarch64-apple-darwin" "mystiko" "${binary_path}"
          ;;
        *)
          echo "Unsupported arch: ${arch}"
          exit 1
          ;;
      esac
      ;;
    *)
      echo "Unsupported os: ${os}"
      exit 1
      ;;
  esac
  setup_path "${binary_path}"
}

install "$@"