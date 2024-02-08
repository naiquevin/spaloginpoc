from flask import Flask, request, make_response, redirect, url_for, render_template

app = Flask(__name__)

USERS = {"vineet": "s3cret"}


@app.route("/")
def index():
    user = request.cookies.get("logged_in_user")
    data = {}
    if user:
        data["name"] = user
        data["is_logged_in"] = True
    else:
        data["name"] = "guest"
        data["is_logged_in"] = False
    return render_template("index.html", **data)


@app.route("/login", methods=["GET", "POST"])
def login():
    if request.method == "GET":
        return render_template("login.html")
    else:
        username = request.form.get("username")
        if username and username in USERS and request.form.get("password") == USERS[username]:
            resp = make_response(redirect(url_for("index"), 302))
            resp.set_cookie("logged_in_user", username)
            return resp
        else:
            return "Authentication failed", 401


@app.route("/logout", methods=["GET"])
def logout():
    resp = make_response(redirect(url_for("index"), 302))
    resp.delete_cookie("logged_in_user")
    return resp


@app.route("/info", methods=["GET"])
def info():
    user = request.cookies.get("logged_in_user")
    if user:
        return {"info": user}, 200
    else:
        return {"err": "user not logged in"}, 401
