from flask import Flask, request, jsonify, send_from_directory
import requests
import os

app = Flask(__name__, static_folder='.')

# 1. Esta línea es la clave: Sirve tu index.html cuando entras a la raíz
@app.route('/')
def home():
    return send_from_directory('.', 'index.html')

# 2. Tu puente para hablar con Ollama (se mantiene igual)
@app.route('/preguntar', methods=['POST'])
def preguntar():
    try:
        data = request.json
        pregunta = data.get('pregunta', '')

        url_ollama = "http://localhost:11434/api/generate"
        payload = {
            "model": "qwen2.5:7b",
            "prompt": pregunta,
            "stream": False
        }
        
        respuesta = requests.post(url_ollama, json=payload)
        respuesta_json = respuesta.json()
        
        return jsonify({"respuesta": respuesta_json['response']})
    
    except Exception as e:
        return jsonify({"respuesta": f"Error: {str(e)}"})

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000)
