#!/bin/sh

set -e

owner=leonovk
repo=yaisd
exe_name=yaisd
githubUrl="https://github.com"
githubApiUrl="https://api.github.com"

get_os(){
    os=$(uname -s | awk '{print tolower($0)}')
    if [ "$os" = "linux" ]; then
        echo "unknown-linux-gnu"
    elif [ "$os" = "darwin" ]; then
        echo "apple-darwin"
    else
        echo $os
    fi
}

downloadFolder="${TMPDIR:-/tmp}"
os=$(get_os)
arch=$(uname -m)
file_name="${exe_name}-${arch}-${os}.tar.gz" # the file name should be download
downloaded_file="${downloadFolder}/${file_name}" # the file path should be download
executable_folder="$HOME/bin" # Eventually, the executable file will be placed here
mkdir -p ${executable_folder}

asset_path=$(
    command curl -s -L \
        -H "Accept: application/vnd.github+json" \
        -H "X-GitHub-Api-Version: 2022-11-28" \
        ${githubApiUrl}/repos/${owner}/${repo}/releases |
    command grep -o "/${owner}/${repo}/releases/download/.*/${file_name}" |
    command head -n 1
)
if [[ ! "$asset_path" ]]; then
    echo "ERROR: unable to find a release asset called ${file_name}"
    exit 1
fi
asset_uri="${githubUrl}${asset_path}"

echo "[1/3] Download ${asset_uri} to tmp dir ${downloadFolder}"
rm -f ${downloaded_file}
curl --silent --fail --location --output "${downloaded_file}" "${asset_uri}"

echo "[2/3] Install ${exe_name} to the ${executable_folder}"
tar -xz -f ${downloaded_file} -C ${executable_folder}
exe=${executable_folder}/${exe_name}
chmod +x ${exe}

echo "[3/3] Set environment variables"
echo ""
printf "\e[1;32m%s\e[0m was installed successfully to \e[1;34m%s\e[0m\n" "${exe_name}" "${exe}"

if command -v $exe_name --version >/dev/null; then
    printf "Run '\e[1;32m%s --help\e[0m' to get started\n" "$exe_name"
else
    echo "Manually add the directory to your \$HOME/.bash_profile (or similar)"
    echo "  echo 'export PATH=${executable_folder}:\$PATH' >> .bash_profile"
    printf "Run '\e[1;32m%s --help\e[0m' to get started\n" "$exe_name"
fi

exit 0
