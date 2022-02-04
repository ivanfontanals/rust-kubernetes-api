#!/bin/bash

set -e

# Example usage with concrete parameters:
# Install gh, see https://cli.github.com/manual/installation
# export GITHUB_USER="spt-cre-jobs-travis"
# export GITHUB_TOKEN="$(gopass show cre/services/github.mpi-internal.com/spt-cre-jobs-travis@adevinta.com/GITHUB_TOKEN_UNICRON_PRS)"
# bash -x scripts/gitops.sh \
#   clone ivan-fontanals/unicron /tmp/gitops dev 

# Default values
declare -r GITHUB_HOST=github.XXX.com
declare -r GITHUB_USERNAME="${GITHUB_USER:-"github-user"}"

function git_config() {
  git config --global user.email "$(<<<"${GITHUB_USERNAME}" tr - .)@adevinta.com"
  git config --global user.name "${GITHUB_USERNAME}"
}

function print_config() {
  echo "REPO: $REPO_DIRECTORY"
  echo "COMMIT_MSG: $COMMIT_MSG "
  echo "PR_TITLE: $PR_TITLE "
  echo "PR_BRANCH_NAME: $PR_BRANCH_NAME"
  echo "BASE_BRANCH: $BASE_BRANCH"
}

function clone_repo() {
  GITOPS_REPO="${1}"
  DESTINATION="${2}"
  BRANCH="${3}"
  GITOPS_REPO_URL="https://${GITHUB_HOST}/${GITOPS_REPO}.git"
  
  rm -rf /tmp/gitops
  mkdir -p /tmp/gitops/ && cd /tmp/gitops
  echo "Cloning repo from ${GITOPS_REPO_URL} to the ${DESTINATION} folder and ${BRANCH} branch"
  # Clone GitOps repo and set up
  echo "https://${GITHUB_USERNAME}:${GITHUB_TOKEN}@${GITHUB_HOST}" >~/.git-credentials-tmp
  git config --global --add hub.host "${GITHUB_HOST}"
  git_config
  git config --global credential.helper 'store --file ~/.git-credentials-tmp'
  git config --global credential.helper 'cache --timeout 1800' # 30m
  git config --global http.sslVerify false
  git clone "https://${GITHUB_HOST}/${GITOPS_REPO}.git" --quiet
  cd "/tmp/gitops/$(basename -- "${GITOPS_REPO}" .git)" || exit 1
  git checkout "${BRANCH}" --quiet
  echo "Clone OK!"
  rm -rf ~/.git-credentials-tmp
}


# Required variables: 
# REPO_DIRECTORY
# COMMIT_MSG
# PR_TITLE
# PR_BODY
# PR_BRANCH_NAME
# BASE_BRANCH
function create_pr() {
  
  print_config
  
  cd "${REPO_DIRECTORY}" || exit 1
  git checkout ${BASE_BRANCH}
  echo "Creating PR for the base branch: ${BASE_BRANCH}"
  
  # Create branch and commit
  TIMESTAMP="$(date +%Y-%m-%d_%H_%M_%S)"
  PR_BRANCH="${PR_BRANCH_NAME}-${TIMESTAMP}"
  git_config
  git checkout -b "${PR_BRANCH}"
  git add . 
  git commit -m "${COMMIT_MSG}"
  git push origin "${PR_BRANCH}"

  # Login to Github Cli and create Pull Request
  <<<"${GITHUB_TOKEN}" gh auth login --with-token --hostname ${GITHUB_HOST}
  gh pr create --title "${PR_TITLE}" \
    --body "${PR_BODY}" \
    --base "${BASE_BRANCH}"
  rm -rf "${REPO_DIRECTORY}"
  echo "Directory deleted: ${REPO_DIRECTORY}"
}

# Required variables: 
# REPO_DIRECTORY
# COMMIT_MSG
function auto_commit() {
  
  cd "${REPO_DIRECTORY}" || exit 1
  BRANCH=$(git rev-parse --abbrev-ref HEAD)
  echo "Auto-commit enabled. Pushing all the changes to ${BRANCH}"
  git add . 
  git commit -m "${COMMIT_MSG}"
  git push
  rm -rf "${REPO_DIRECTORY}"
  echo "Directory deleted: ${REPO_DIRECTORY}"
}

# catch first arguments with $1
case "$1" in
  clone)
  # Clones a repository
  clone_repo ${@:2}
  ;;
  pull_request)
  # Creates a Pull Request
  create_pr
  ;;
  auto_commit)
  # Creates an auto_commit
  auto_commit
  ;;
  clean)
  clean_directory ${@:2}
  ;;
 *)
  # else
  echo "Usage:"
  echo "  clone GITOPS_REPO DESTINTATION BRANCH"
  echo "  clean DIR_TO_DELETE"
  echo "  auto_commit [with the following ENV_VARS => REPO_DIRECTORY COMMIT_MSG]"
  echo "  pull_request [with the following ENV_VARS =>  REPO_DIRECTORY COMMIT_MSG PR_TITLE PR_BODY PR_BRANCH_NAME]"
  ;;
esac