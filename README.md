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
configure zookeper for minikube based on article:https://gsfl3101.medium.com/kafka-raft-kraft-cluster-configuration-from-dev-to-prod-part-1-8a844fabf804
```
kubectl apply -f kubernetes/kafka-zookeeper.yaml
kubectl apply -f kubernetes/kafka.yaml

kubectl delete -n default deployment kafka-deployment-1
kubectl delete -n default deployment kafka-deployment-2
kubectl delete -n default service kafka-service
```


