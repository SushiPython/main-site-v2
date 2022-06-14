# flask boilerplate
from flask import Flask, render_template
import json

app = Flask(__name__)

@app.route('/')
def index():
    return render_template('index.html')

@app.route('/about')
def about():
    return render_template('about.html')

@app.route('/projects')
def projects():
    # open projects.json and read
    with open('projects.json', encoding="utf-8") as projects_file:
        return render_template('projects.html', projects=json.load(projects_file))
    

app.run(host='0.0.0.0', port=8080)