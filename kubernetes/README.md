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
Nota: Después de instalar, asegúrate de configurar las interfaces en /etc/wireguard/wg0.conf y levantar el servicio con wg-quick up wg0 (o el comando específico de tu configuración).
3. Despliegue de Contenedores (k3s)
Instalación del clúster k3s en el servidor:
Bash
# Comando estándar de instalación de k3s
```bash
curl -sfL [https://get.k3s.io](https://get.k3s.io) | sh -
```
Verificar que el nodo esté listo:

```bash
sudo k3s kubectl get nodes
```
Despliegue de manifiestos:
Coloca aquí tus archivos .yaml o ejecuta los comandos de despliegue:

```bash
# Ejemplo:
# sudo k3s kubectl apply -f ./k8s/deployment.yaml
```
________________________________________
🛠️ Compilación y Ejecución (Rust)
Para compilar y ejecutar el sistema distribuido localmente o en el entorno de desarrollo:
1.	Navega a la raíz del proyecto.
2.	Ejecuta los siguientes comandos de Cargo:
Compilar el proyecto:

``` bash
cargo build --release
# Ejecutar el sistema:
cargo run
```
