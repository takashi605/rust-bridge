build:
	docker compose build

# --abort-on-container-exit で、テストが終了したら新たに起動したコンテナを停止する
# すでに起動していたコンテナは停止しない
test:
	docker compose --profile test up --build --abort-on-container-exit

down:
	docker compose down -v
