build:
	docker compose build

# --abort-on-container-exit で、テストが終了したら新たに起動したコンテナを停止する
# すでに起動していたコンテナは停止しない
# api-test コンテナ内で workspace の全クレート（api_schema, api, api_test）をテストする
test:
	docker compose --profile test up --build api-test --abort-on-container-exit

up:
	docker compose up -d

down:
	docker compose down -v
