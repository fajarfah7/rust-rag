# RAG with Rust
## Technologies
* PostgreSQL
* Qdrant
* MinIO
* Kafka
* LM Studio

# installation steps
1. setup your environment
    * rename .env_example -> .env and set the credentials
2. run migration using sqlx
3. install qdrant on docker by running docker compose on docker folder
4. run LM Studio to manage embedding and chat
    * on LM Studio MUST ```meta-llama-3-8b-instruct``` installed
5. install CMake
6. install kafka
7. run api ```cargo run --bin api```
8. run consumer ```cargo run --bin consumer```

# about version v0.1
this version actually not perfect yet, the objective here are:
1. user able to login
2. user upload document
3. worker receiving uploaded document
4. worker process the document like:
    * create chunks
    * process embedding
    * store vector into qdrant
    * able to answer the question(for this feature still in development)