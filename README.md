# 実行方法

## 事前準備

あらかじめAWS Secrets Managerにシークレットの値を設定しておいてください。
 - `COGNITO_CLIENT_SECRET`
 - `COGNITO_CLIENT_ID`
 - `COGNITO_USER_POOL_ID"`

## 実行
```
export SECRETS_MANAGER_ID='<your_secrets_manager_id>'
export COGNITO_USERNAME='<your_username>'
export COGNITO_PASSWORD='<your_password>'
export API_GATEWAY_URL='<your_api_gateway_url>'

cargo run
```
