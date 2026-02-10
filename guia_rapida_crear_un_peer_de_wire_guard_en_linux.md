# Guía rápida: Crear un peer de WireGuard en Linux

Esta guía está pensada para que cualquier compañero pueda **crear su peer de WireGuard** y entender *qué está haciendo y por qué*, no solo copiar/pegar comandos.

> Contexto típico: Windows 10 → SSH → VM Linux → WireGuard

---

## 1️⃣ Instalar y establecer conexión SSH (Windows 10 → VM Linux)

### En la VM Linux
Asegúrate de tener el servidor SSH instalado y activo:

```bash
sudo apt update
sudo apt install openssh-server
sudo systemctl enable ssh
sudo systemctl start ssh
```

Obtén la IP de la VM:
```bash
ip addr
```

### En Windows 10
Desde PowerShell o CMD:

```powershell
ssh usuario@IP_DE_LA_VM
```

Ejemplo:
```powershell
ssh jj@192.168.100.10
```

Si es la primera vez, acepta la clave del host.

---

## 2️⃣ Convertirse en root

WireGuard **necesita privilegios de administrador** porque crea interfaces de red y modifica rutas.

```bash
sudo -i
```

Verifica:
```bash
whoami
```
Debe decir `root`.

---

## 3️⃣ Navegar a `/etc/wireguard`

Este es el directorio estándar donde WireGuard guarda sus configuraciones.

```bash
cd /etc/wireguard
```

Si no existe:
```bash
mkdir /etc/wireguard
chmod 700 /etc/wireguard
cd /etc/wireguard
```

---

## 4️⃣ Crear las llaves (privada y pública)

### ¿Para qué sirven?

- **Llave privada** 🔒
  - Identifica *secretamente* al peer
  - **Nunca se comparte**

- **Llave pública** 🔑
  - Se deriva de la privada
  - Es la que se intercambia con otros peers

### Generación de llaves

```bash
wg genkey | tee privatekey | wg pubkey > publickey
```

Archivos creados:
- `privatekey` → solo para este peer
- `publickey` → se comparte con el otro extremo

Protege la privada:
```bash
chmod 600 privatekey
```

---

## 5️⃣ Crear el archivo `wg0.conf`

Este archivo define **la interfaz WireGuard** y **los peers**.

```bash
nano wg0.conf
```

### 📌 Plantilla: Peer (cliente)

```ini
[Interface]
PrivateKey = <LLAVE_PRIVADA_DEL_PEER>
Address = 10.0.0.2/24
DNS = 1.1.1.1

[Peer]
PublicKey = <LLAVE_PUBLICA_DEL_SERVIDOR>
Endpoint = IP_PUBLICA_DEL_SERVIDOR:51820
AllowedIPs = 0.0.0.0/0
PersistentKeepalive = 25
```

---

### 📌 Plantilla: Servidor

```ini
[Interface]
PrivateKey = <LLAVE_PRIVADA_DEL_SERVIDOR>
Address = 10.0.0.1/24
ListenPort = 51820

[Peer]
PublicKey = <LLAVE_PUBLICA_DEL_PEER>
AllowedIPs = 10.0.0.2/32
```

---

### Notas importantes
- `Address` → IP **dentro del túnel**
- `AllowedIPs` → qué tráfico se envía por el túnel
- `Endpoint` → IP y puerto del otro extremo

---

## 6️⃣ Crear el servicio `wg-quick`

### ¿Qué significa “crear el servicio”?

WireGuard puede levantarse **a mano**, pero como servicio:

- Se inicia automáticamente al prender la VM
- systemd lo supervisa
- Se aplican rutas y firewall correctamente
- Es la forma correcta en infraestructura real

### Comandos

```bash
sudo systemctl enable wg-quick@wg0
sudo systemctl start wg-quick@wg0
```

Esto crea el servicio:
```
wg-quick@wg0.service
```

Basado en el archivo:
```
/etc/wireguard/wg0.conf
```

### Ver estado

```bash
systemctl status wg-quick@wg0
```

---

## 🔧 Comandos útiles para diagnóstico

```bash
wg show
```
Muestra estado del túnel y último handshake.

```bash
ip addr
```
Ver interfaces de red (debe aparecer `wg0`).

```bash
ip route
```
Ver rutas activas.

```bash
ss -lunp | grep 51820
```
Verifica que WireGuard escucha en el puerto UDP.

```bash
journalctl -u wg-quick@wg0
```
Logs del servicio WireGuard.

---

## 🧠 Idea clave para recordar

> WireGuard = criptografía + red + systemd

Si una parte falla, el túnel no funciona.

---

Si algo no conecta:
- revisa llaves
- revisa IPs
- revisa `AllowedIPs`
- revisa si el servicio está activo

Fin 🚀

