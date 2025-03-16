# **Caching Proxy Server** 🚀  

A **high-performance caching proxy** built with **Rust, Axum, and Redis** to improve response times and reduce API load. Supports **persistent caching, compression, rate limiting**, and is **fully containerized** with Docker.


[roadmap.sh project url](https://roadmap.sh/projects/caching-server)

## 📌 **Table of Contents**  

- [🔧 Features](#-features)  
- [📥 Installation](#-installation)  
- [🐳 Docker Setup](#-docker-setup)  
- [⚙️ Configuration](#-configuration)  
- [📝 Usage Examples](#-usage-examples)  
- [🛠 API Endpoints](#-api-endpoints)  
- [📜 License](#-license)  


## 🔧 **Features**  
✅ **Fast caching** with Redis (persistent storage)  
✅ **Gzip compression** for bandwidth efficiency  
✅ **Configurable cache expiration (TTL)**  
✅ **Rate limiting to prevent abuse**  
✅ **Detailed logging and monitoring**  
✅ **Fully containerized with Docker**  



## 📥 **Installation**  

### **1️⃣ Install Manually (Rust & Cargo)**
> **Requirements:** Rust, Cargo, Redis

```sh
# Clone the repository
git clone https://github.com/your-repo/cache-proxy.git
cd cache-proxy

# Install dependencies & build
cargo build --release

# Run the proxy
./target/release/cache-proxy --port 3000 --origin "http://example.com" --cache-ttl 120
```



## 🐳 **Docker Setup**  

### **2️⃣ Using Docker (Recommended)**
> **Requirements:** Docker & Docker Compose

```sh
# Build and run the container
docker-compose up --build -d
```

To stop:
```sh
docker-compose down
```

View logs:
```sh
docker-compose logs -f cache-proxy
```



## ⚙️ **Configuration**  

You can configure the caching proxy using **command-line flags** or **Docker environment variables**.

### **Available Options:**
| Flag / Env Var   | Description | Default |
|------------------|-------------|---------|
| `--port` / `$PORT` | Server listening port | `3000` |
| `--origin` / `$ORIGIN` | The origin URL to proxy | `"http://localhost"` |
| `--cache-ttl` / `$CACHE_TTL` | Cache expiration time (seconds) | `60` |
| `--redis-url` / `$REDIS_URL` | Redis connection URL | `"redis://127.0.0.1/"` |
| `--clear-cache` | Clears Redis cache before starting | `false` |


## 📝 **Usage Examples**  

### **Run with Custom Configuration**
```sh
./cache-proxy --port 8080 --origin "http://example.com" --cache-ttl 120
```

### **Clear Cache**
```sh
./cache-proxy --clear-cache
```

### **Using Docker**
```sh
docker run -p 3000:3000 -e ORIGIN="http://example.com" -e CACHE_TTL=120 your-image-name
```



## 🛠 **API Endpoints**  

### **📌 `GET /` (Proxy Request)**
Fetches and caches responses from the **origin server**.

#### **Example Request**
```sh
curl -i http://localhost:3000/
```

#### **Response Headers**
```http
X-Cache: HIT  # Cached response
X-Cache: MISS  # New response from origin
```


## 📜 **License**
MIT License. Feel free to modify and use! 🚀  


