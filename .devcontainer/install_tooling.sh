#!/bin/sh

set -ex

mkdir -p $HOME/.local/bin
cd $HOME/.local/bin

if [ X"$1" = X"login" ];then 
    if [ X"${GH_TOKEN}" = X"" ];then 
        read -p "Paste here your Github personal access token: " GH_TOKEN
        export GH_TOKEN
    fi 
fi


## Install Conventional Commits - Cocogitto (`cog`)
CURRENT_REPO="cocogitto/cocogitto"
CURRENT_VERSION=$(gh --repo $CURRENT_REPO release view --json tagName --jq .tagName)
DOWNLOADED_FILE=$(gh --repo $CURRENT_REPO release view --json assets --jq '.assets[] | select(.name | contains("x86_64") and contains("linux") and contains("musl")) .name')
gh --repo $CURRENT_REPO --pattern "$DOWNLOADED_FILE" release download $CURRENT_VERSION
tar zxvf $DOWNLOADED_FILE cog
rm $DOWNLOADED_FILE

## Install Just (`just`)
CURRENT_REPO="casey/just"
CURRENT_VERSION=$(gh --repo $CURRENT_REPO release view --json tagName --jq .tagName)
DOWNLOADED_FILE=$(gh --repo $CURRENT_REPO release view --json assets --jq '.assets[] | select(.name | contains("x86_64") and contains("linux") and contains("musl")) .name')
gh --repo $CURRENT_REPO --pattern "$DOWNLOADED_FILE" release download $CURRENT_VERSION
tar zxvf $DOWNLOADED_FILE just
rm $DOWNLOADED_FILE

## Install Difftastic (`difft`)
CURRENT_REPO="Wilfred/difftastic"
CURRENT_VERSION=$(gh --repo $CURRENT_REPO release view --json tagName --jq .tagName)
DOWNLOADED_FILE=$(gh --repo $CURRENT_REPO release view --json assets --jq '.assets[] | select(.name | contains("x86_64") and contains("linux") and contains("gnu")) .name')
gh --repo $CURRENT_REPO --pattern "$DOWNLOADED_FILE" release download $CURRENT_VERSION
tar zxvf $DOWNLOADED_FILE difft
rm $DOWNLOADED_FILE

## Install Watchexec (`watchexec`)
CURRENT_REPO="watchexec/watchexec"
CURRENT_VERSION=$(gh --repo $CURRENT_REPO release view --json tagName --jq .tagName)
DOWNLOADED_FILE=$(gh --repo $CURRENT_REPO release view --json assets --jq '.assets[] | select(.name | contains("x86_64") and contains("linux") and contains("musl") and endswith(".tar.xz")) .name')
TMP_FOLDER=$(basename $DOWNLOADED_FILE .tar.xz)
gh --repo $CURRENT_REPO --pattern "$DOWNLOADED_FILE" release download $CURRENT_VERSION
tar -Jxvf $DOWNLOADED_FILE
mv ${TMP_FOLDER}/watchexec ./
rm -rf $DOWNLOADED_FILE $TMP_FOLDER

## Install Cargo audit (`cargo-audit`)
CURRENT_REPO="rustsec/rustsec"
CURRENT_VERSION=$(gh --repo $CURRENT_REPO release view --json tagName --jq .tagName)
DOWNLOADED_FILE=$(gh --repo $CURRENT_REPO release view --json assets --jq '.assets[] | select(.name | contains("x86_64") and contains("linux") and contains("musl") and endswith(".tgz")) .name')
TMP_FOLDER=$(basename $DOWNLOADED_FILE .tgz)
gh --repo $CURRENT_REPO --pattern "$DOWNLOADED_FILE" release download $CURRENT_VERSION
tar -xvf $DOWNLOADED_FILE
mv ${TMP_FOLDER}/cargo-audit ./
rm -rf $DOWNLOADED_FILE $TMP_FOLDER

## Install Cargo audit (`cargo-deny`)
CURRENT_REPO="EmbarkStudios/cargo-deny"
CURRENT_VERSION=$(gh --repo $CURRENT_REPO release view --json tagName --jq .tagName)
DOWNLOADED_FILE=$(gh --repo $CURRENT_REPO release view --json assets --jq '.assets[] | select(.name | contains("x86_64") and contains("linux") and contains("musl") and endswith(".tar.gz")) .name')
TMP_FOLDER=$(basename $DOWNLOADED_FILE .tar.gz)
gh --repo $CURRENT_REPO --pattern "$DOWNLOADED_FILE" release download $CURRENT_VERSION
tar -xvf $DOWNLOADED_FILE
mv ${TMP_FOLDER}/cargo-deny ./
rm -rf $DOWNLOADED_FILE $TMP_FOLDER

## Install cargo generate (`cargo-generate`) 
CURRENT_REPO="cargo-generate/cargo-generate"
CURRENT_VERSION=$(gh --repo $CURRENT_REPO release view --json tagName --jq .tagName)
DOWNLOADED_FILE=$(gh --repo $CURRENT_REPO release view --json assets --jq '.assets[] | select(.name | contains("x86_64") and contains("linux") and contains("musl") and endswith(".tar.gz")) .name')
TMP_FOLDER=$(basename $DOWNLOADED_FILE .tar.gz)
gh --repo $CURRENT_REPO --pattern "$DOWNLOADED_FILE" release download $CURRENT_VERSION
tar -xvf $DOWNLOADED_FILE cargo-generate
rm -rf $DOWNLOADED_FILE

