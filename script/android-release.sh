#!/bin/bash
# Copyright (c) 2026 Mivik <mivik@qq.com>
# Portions Copyright (c) 2022 1Password
#
# This file includes code from
# [rustls-platform-verifier](https://github.com/rustls/rustls-platform-verifier)
# Licensed under the MIT License and the Apache License, Version 2.0.
# You may obtain a copy of the Licenses at:
# http://opensource.org/licenses/MIT
# http://www.apache.org/licenses/LICENSE-2.0

set -euo pipefail

if ! type mvn > /dev/null; then
  echo "The maven CLI, mvn, is required to run this script."
  echo "Download it from: https://maven.apache.org/download.cgi"
  exit 1
fi

version=$(grep -m 1 "version = " android-release-support/Cargo.toml | tr -d "version= " | tr -d '"')
echo "Packaging v$version of the Android support component"

pushd ./android
./gradlew assembleRelease
popd

artifact_name="inputbox-release.aar"

pushd ./android-release-support
artifact_path="../android/inputbox/build/outputs/aar/$artifact_name"

# Ensure no prior artifacts are present
git clean -dfX "./maven/"

cp ./pom-template.xml ./maven/pom.xml

# This sequence is meant to workaround the incompatibilites between macOS's sed
# command and the GNU command. Referenced from the following:
# https://stackoverflow.com/questions/5694228/sed-in-place-flag-that-works-both-on-mac-bsd-and-linux
sed -i.bak "s/\$VERSION/$version/" ./maven/pom.xml
rm ./maven/pom.xml.bak

mvn install:install-file -Dfile="$artifact_path" -Dpackaging="aar" -DpomFile="./maven/pom.xml" -DlocalRepositoryPath="./maven/"
