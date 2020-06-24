#!/bin/bash
#
# Example test script that posts a build to Enclave Manager and runs the example.
#

set -eo pipefail

config_file="$1"

if [ -z "$config_file" ]; then
    echo "Please provide a config file as first parameter"
    echo "An example config file is 'config'"
    exit 1
fi

. $config_file

domains="${domain//,/\",\"}"

function em-cli-ratelimit() {
    set -e

    em-cli $*

    if [ ! -z "$rate_limit" ]; then
        sleep $rate_limit
    fi
}

function login() {
    em-cli-ratelimit user login $manager_url $username $password
}

function configureAccount() {
    account_id=$(em-cli-ratelimit account list  | jq -r '.items[] | select(.name=="'"$account_name"'") | .acct_id')
    if [ -z "$account_id" ]; then
        account_id=$(em-cli-ratelimit account create "$account_name" | jq -r '.acct_id')
        echo "Account created: $account_id"
    fi
    em-cli-ratelimit account select "$account_id"
}

function configureApp() {
    sigstruct_json=$(em-cli-ratelimit build parse-sigstruct "$sigstruct")

    app=$(em-cli-ratelimit app list | jq -r '.items[] | select(.name=="'"$app_name"'")')
    if [ -z "$app" ]; then
        echo "Creating application"
        app_isvsvn=$(echo "$sigstruct_json" | jq -r '.isvsvn')
        app_isvprodid=$(echo "$sigstruct_json" | jq -r '.isvprodid')
        app_id=$(em-cli-ratelimit app create "$app_name" "$app_isvprodid" "$app_isvsvn" "$domain" | jq -r '.app_id')
    else
        app_id=$(echo "$app" | jq -r '.app_id')
        if [[ "false" == $(echo "$app" | jq -r '.whitelisted_domains | contains(["'"$domains"'"])') ]]; then
            if [[ "false" == $(echo "$app" | jq -r '.domains_added | contains(["'"$domains"'"])') ]]; then
                allowed_domains=$(echo "$app" | jq -r '.whitelisted_domains | join(",")')
                allowed_domains="${allowed_domains},$domain"
                echo "Updating domains on existing application"
                em-cli-ratelimit app update "$app_id" --allowed-domains "$allowed_domains" > /dev/null
            fi
        fi
    fi
    echo "Application configuration finished."
}

function whitelistDomain() {
    tasks=$(em-cli-ratelimit task list | jq -r '.items[] | select(.task_type=="DOMAIN_WHITELIST" and .entity_id=="'"$app_id"'" and .status.status == "INPROGRESS") | .task_id')
    for task_id in $tasks; do
        status=$(em-cli-ratelimit task update "$task_id" approved | jq -r '.task_status.status')
        echo "Domain whitelist result: $status"
    done
    echo "Domain whitelisting finished."
}

function configureBuild() {
    mrenclave=$(echo "$sigstruct_json" | jq -r '.mrenclave')

    build=$(em-cli-ratelimit build list | jq -r '.items[] | select(.enclave_info.mrenclave=="'"$mrenclave"'")')
    if [ -z "$build" ]; then
        build_id=$(em-cli-ratelimit build create "$app_id" "$sigstruct" | jq -r ".build_id")
    else
        build_id=$(echo "$build" | jq -r '.build_id')
        build_app_name=$(echo "$build" | jq -r '.app_name')

        if [[ "$build_app_name" != "$app_name" ]]; then
            echo "Error: Image MRENCLAVE already exists in a different application: $build_app_name"
            echo "       Please update config to use that application or create a different build"
            exit 0
        fi
    fi
    echo "Build configuration finished"
}

function whitelistBuild() {
    tasks=$(em-cli-ratelimit task list | jq -r '.items[] | select(.entity_id=="'"$build_id"'" and .task_type=="BUILD_WHITELIST" and .status.status == "INPROGRESS") | .task_id')
    for task_id in ${tasks[@]}; do
        status=$(em-cli-ratelimit task update "$task_id" approved | jq -r '.task_status.status')
        echo "Build whitelist result: $status"
    done
    echo "Build whitelisting finished"
}

login
configureAccount
configureApp
whitelistDomain
configureBuild
whitelistBuild

