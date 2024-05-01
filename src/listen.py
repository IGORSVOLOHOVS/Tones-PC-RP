from flask import Flask, request

app = Flask(__name__)

@app.route('/', methods=['POST'])
def handle_post():
    post_data = request.get_json()  # Access JSON data directly
    print(f"Received POST data: {post_data}")  
    return "POST request received!", 200 

if __name__ == '__main__':
    app.run(debug=True) 