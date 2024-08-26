import socket
import random
import time
from concurrent.futures import ThreadPoolExecutor

HOST = 'localhost'
PORT = 7878


BLOQUES = ['VIP','A1','A2','B','C']

def enviar_request():

    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((HOST, PORT))

        bloque = random.choice(BLOQUES)
        cantidad_asientos = random.randint(1, 12)
        mensaje = f"{18}/{'VIP'}"

        start_time = time.time()
        s.sendall(mensaje.encode())

        data = s.recv(1024)
        end_time = time.time()

        tiempo_respuesta = end_time - start_time

        print(f"\n\nPeticion enviada: {mensaje}")
        print(f"\nRespuesta del servidor:\n{data.decode()}")
        print(f"Tiempo de respuesta: {tiempo_respuesta:.4f} segundos")

        confirmacion = random.choice(["yes", "no"])
        s.sendall(confirmacion.encode())

        print(f"Confirmacion enviada: {confirmacion}")

def realizar_peticiones_en_lotes():
    time.sleep(5)
    with ThreadPoolExecutor(max_workers=5) as executor:
        for _ in range(2):
            futures = [executor.submit(enviar_request) for _ in range(1)]

            for future in futures:
                future.result()

            time.sleep(2)

if __name__ == "__main__":
    realizar_peticiones_en_lotes()
