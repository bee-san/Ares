from flask import Flask, render_template_string, request, redirect
from jinja2.exceptions import TemplateSyntaxError
import re
app = Flask(__name__)

@app.route('/', methods = ['GET', 'POST'])
def home():
    if request.method == 'POST':
        return redirect('/announce', code=307)
    else:
        return render_template_string("""
                <!doctype html>
                <title>SSTI2</title>

                <h1> Home </h1>

                <p> I built a cool website that lets you announce whatever you want!* </p>

                <form action="/" method="POST">
                What do you want to announce: <input name="content" id="announce"> <button type="submit"> Ok </button>
                </form>
                
                <p style="font-size:10px;position:fixed;bottom:10px;left:10px;"> *Announcements may only reach yourself </p>
                                      """
                                    )
    
@app.route("/announce", methods = ["POST"])
def announcement():
    # Filter out all the bad characters
    announcement = request.form.get("content", "")
    announcement = re.sub(r'[_\[\]\.]|\|join|base', "", announcement)

    try: 
        return render_template_string("""
                    <!doctype html>
                    <h1 style="font-size:100px;" align="center">""" +
                    announcement + 
                    """</h1>""",
                                    )
    except TemplateSyntaxError:
        return render_template_string("""
                    <!doctype html>
                    <h1 style="font-size:100px;" align="center">""" +
                    "Stop trying to break me >:(" + 
                    """</h1>""",
                                    )
if __name__ == '__main__':
    app.run(port=5001)