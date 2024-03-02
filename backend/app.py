from flask import Flask, request, make_response, redirect, render_template

app = Flask(__name__)

USERS = {"vineet": "s3cret"}


@app.route("/login", methods=["GET", "POST"])
def login():
    if request.method == "GET":
        return render_template("login.html")
    else:
        username = request.form.get("username")
        if username and username in USERS and request.form.get("password") == USERS[username]:
            resp = make_response(redirect("/", 302))
            resp.set_cookie("session_id", username)
            return resp
        else:
            return "Authentication failed", 401


@app.route("/logout", methods=["GET"])
def logout():
    resp = make_response(redirect("/login", 302))
    resp.delete_cookie("session_id")
    return resp


@app.route("/info", methods=["GET"])
def info():
    user = request.cookies.get("session_id")
    if user:
        return {"user": user}, 200
    else:
        return {"err": "user not logged in"}, 401


@app.route("/auth", methods=["GET"])
def auth():
    user = request.cookies.get("session_id")
    print(request.headers.get("X-Original-URI"))
    # print(user)
    if user:
        return "", 200
    else:
        return "", 401
