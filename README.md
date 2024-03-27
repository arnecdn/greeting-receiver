# greeting_rust
This is a simple sample app implementing a service for receiving greetings and storing them in a database. 
It furthers distributes greetings via Kafka topic with Debezium

# Enable offline build for SQLX
```
cargo sqlx prepare'  
cargo build
```
The component is a sample app in RUST for creating a service for receiving greetings
It implements an API for receiving and listing greetings. 
The service publishes events from new greetings downstream for consumers. 

#activat debezium connector 
```
curl -i -X POST -H "Accept:application/json" -H "Content-Type:application/json" localhost:8083/connectors/ --data "@debezium.json"
```
# deleting debezium connector
```
curl -i -X DELETE localhost:8083/connectors/greeting-connector
```


Minikube
configure zookeper for minikube 
```
kubectl apply -f kubernetes/kafka-zookeeper.yaml
```