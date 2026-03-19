# Sistema Distribuido en Rust 🦀

Este proyecto implementa un sistema distribuido desarrollado en **Rust**, diseñado para ejecutarse sobre un clúster de **Kubernetes (k3s)** y comunicarse a través de una red segura **WireGuard**.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)
![Kubernetes](https://img.shields.io/badge/kubernetes-%23326ce5.svg?style=for-the-badge&logo=kubernetes&logoColor=white)
![WireGuard](https://img.shields.io/badge/wireguard-%2388171A.svg?style=for-the-badge&logo=wireguard&logoColor=white)

## 📋 Requisitos Previos

Antes de comenzar, asegúrate de tener el siguiente entorno y herramientas:

### Entorno de Virtualización
* **Hyper-V** habilitado.
* **Imagen ISO:** Ubuntu Server (Recomendado 20.04 LTS o superior).

### Software Base
* **Rust** (Cargo & rustc instalados en el entorno de desarrollo).
* **Docker** (Para la creación de imágenes del sistema).
* **k3s** (Distribución ligera de Kubernetes).
* **WireGuard** (Para la VPN entre nodos).

---

## 🚀 Instalación y Despliegue

### 1. Configuración del Entorno (Hyper-V)
1.  Crear una máquina virtual en Hyper-V utilizando la ISO de Ubuntu Server.
2.  Asegurar conectividad a internet en la VM.
3.  Actualizar el sistema base.

### 2. Levantar la VPN (WireGuard)

Para asegurar la comunicación entre los nodos o servicios, instalamos WireGuard:

```bash
sudo apt update
sudo apt install wireguard
```
Nota: Después de instalar, asegúrate de configurar las interfaces en  `/etc/wireguard/wg0.conf` y levantar el servicio con `wg-quick up wg0`  (o el comando específico de tu configuración).
# 🚀 Instalación y Configuración de k3s

## Servidor (Control Plane) con WireGuard

Instalar k3s en el nodo servidor considerando que la comunicación se realiza a través del túnel de WireGuard:

```bash
curl -sfL https://get.k3s.io | sh -
```

⚠️ Asegúrate de que la interfaz de WireGuard esté activa y que las IPs del túnel se usen para la comunicación del clúster.

Verificar que el nodo esté listo:
```bash

sudo k3s kubectl get nodes
```

Instalar k3s agent en cada nodo worker:
```bash
curl -sfL https://get.k3s.io | K3S_URL=https://<IP_DEL_SERVER>:6443 K3S_TOKEN=<TOKEN> sh -
```
🔑 El K3S_TOKEN se obtiene en el servidor en:
```bash

sudo cat /var/lib/rancher/k3s/server/node-token
```
## 🧩 Cómo armar el clúster

Instala k3s en el servidor.

Obtén el node-token.

Conecta cada worker usando la IP del túnel WireGuard del servidor.

Verifica que todos los nodos estén registrados:
```bash

sudo k3s kubectl get nodes
```
## 🛠️ Compilación y Ejecución (Rust)
Para compilar y ejecutar el sistema distribuido localmente o en el entorno de desarrollo:
1.	Navega a la raíz del proyecto.
2.	Ejecuta los siguientes comandos de Cargo:
Compilar el proyecto:

``` bash
cargo build --release
# Ejecutar el sistema:
cargo run
```

## 📦 Despliegue de manifiestos

Coloca aquí tus archivos .yaml o ejecuta los comandos de despliegue:
```bash

sudo k3s kubectl apply -f ./k8s/deployment.yaml
```

## 🔍 Monitoreo del clúster

Consultar los pods corriendo:
```bash

sudo k3s kubectl get pods -A
```

Consultar más detalle de los pods:
```bash

sudo k3s kubectl describe pod <nombre-del-pod>
```

Consultar los contenedores en cada nodo:
```bash

sudo k3s kubectl get pods -o wide
```


# Notas finales
1. Para que el sistema funcione correctamente, es necesario que cada nodo tenga su carpeta correspondiente del proyecto dentro de su directorio de trabajo.
2. Si el servidor utiliza una IP pública dinámica (común en redes domésticas), los peers pueden perder conectividad cuando esta cambie.