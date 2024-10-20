Add a bid: curl -X POST http://127.0.0.1:8080/add_bid -H "Content-Type: application/json" -d '{"price": 100, "quantity": 10}'
Add an ask: curl -X POST http://127.0.0.1:8080/add_ask -H "Content-Type: application/json" -d '{"price": 105, "quantity": 5}'
Remove a bid: curl -X DELETE "http://127.0.0.1:8080/remove_bid?price=100.0"
Remove an ask: curl -X DELETE "http://127.0.0.1:8080/remove_ask?price=105.0"
Fetch the order book: curl http://127.0.0.1:8080/array


curl -X POST http://127.0.0.1:8080/update



curl -X POST http://127.0.0.1:8080/add_bid -H "Content-Type: application/json" -d '{"price": 100, "quantity": 10, "addresses": []}'

curl -X POST http://127.0.0.1:8080/add_ask -H "Content-Type: application/json" -d '{"price": 105, "quantity": 5, "addresses": []}'