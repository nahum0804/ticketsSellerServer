import socket
import random
import time
from concurrent.futures import ThreadPoolExecutor

# Configuración del cliente
HOST = 'localhost'  # Dirección IP del servidor
PORT = 7878        # Puerto utilizado por el servidor

# Posibles bloques
BLOQUES = ['VIP']

def enviar_request():
    # Crear un socket para conectarse al servidor
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((HOST, PORT))

        # Seleccionar aleatoriamente un bloque y la cantidad de asientos
        bloque = random.choice(BLOQUES)
        cantidad_asientos = random.randint(1, 10)
        mensaje = f"{cantidad_asientos}/{bloque}"

        # Enviar la petición al servidor
        start_time = time.time()
        s.sendall(mensaje.encode())

        # Esperar la respuesta del servidor
        data = s.recv(1024)
        end_time = time.time()

        # Calcular el tiempo de respuesta
        tiempo_respuesta = end_time - start_time

        # Imprimir la respuesta del servidor y el tiempo que tardó
        print(f"Respuesta del servidor: {data.decode()}")
        print(f"Tiempo de respuesta: {tiempo_respuesta:.4f} segundos")

        # Seleccionar aleatoriamente "yes" o "no"
        confirmacion = random.choice(["yes", "no"])
        s.sendall(confirmacion.encode())

        # Imprimir la confirmación enviada
        print(f"Confirmación enviada: {confirmacion}")

def realizar_peticiones_en_lotes():
    time.sleep(5)
    with ThreadPoolExecutor(max_workers=5) as executor:
        for _ in range(3):  # 6 lotes de 5 peticiones = 30 peticiones
            # Enviar 5 peticiones simultáneamente
            futures = [executor.submit(enviar_request) for _ in range(2)]
            
            # Esperar a que terminen todas las peticiones del lote
            for future in futures:
                future.result()
            
            # Esperar 2 segundos antes de enviar el siguiente lote
            time.sleep(2)

if __name__ == "__main__":
    realizar_peticiones_en_lotes()
