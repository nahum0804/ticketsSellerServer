import socket
import random
import time
from concurrent.futures import ThreadPoolExecutor

HOST = 'localhost'
PORT = 7878


BLOQUES = ['VIP','A1','A2','B','C']

def enviar_request():
    try:
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect((HOST, PORT))

            bloque = random.choice(BLOQUES)
            cantidad_asientos = random.randint(1, 15)
            mensaje = f"{cantidad_asientos}/{bloque}"

            start_time = time.perf_counter()
            s.sendall(mensaje.encode())

            data1 = s.recv(1024)
            end_time = time.perf_counter()

            tiempo_respuesta = end_time - start_time

            confirmacion = random.choice(["yes", "no"])
            wait_time = random.randint(1, 15)
            time.sleep(wait_time)

            s.sendall(confirmacion.encode())

            data = s.recv(1024)
            print(f"\n\nPeticion enviada: {mensaje}")
            print(f"\nRespuesta del servidor:\n{data1.decode()}")
            print(f"Tiempo de respuesta: {tiempo_respuesta:.100f} segundos")
            print(f"Cliente respondio: {confirmacion}")
            print(f"Con un tiempo de espera de: {wait_time}")
            print(f"{data.decode()}")

    except ConnectionAbortedError as e:
        print(f"Connection was aborted: {e}")
    except Exception as e:
        print(f"An error occurred: {e}")



def realizar_peticiones_en_lotes():
    with ThreadPoolExecutor(max_workers=5) as executor:
        for _ in range(5):
            futures = [executor.submit(enviar_request) for _ in range(3)]

            for future in futures:
                future.result()
            print("\n\n\n")
            time.sleep(2)

if __name__ == "__main__":
    realizar_peticiones_en_lotes()
