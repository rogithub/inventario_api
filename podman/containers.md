podman run -d --name postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 postgres:latest
podman exec -it postgres psql -U postgres -c "CREATE DATABASE dev_inventario;"

podman run -d --name redis -p 6379:6379 redis:latest


# Desde tu máquina (si tienes redis-cli instalado)
redis-cli -h localhost -p 6379

# Desde el contenedor
podman exec -it redis redis-cli

# Si tiene contraseña
podman exec -it redis redis-cli -a mipassword

# URL de conexión
redis://localhost:6379
# o con contraseña
redis://:mipassword@localhost:6379