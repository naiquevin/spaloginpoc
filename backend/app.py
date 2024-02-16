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
            resp.set_cookie("logged_in_user", username)
            return resp
        else:
            return "Authentication failed", 401


@app.route("/logout", methods=["GET"])
def logout():
    resp = make_response(redirect("/login", 302))
    resp.delete_cookie("logged_in_user")
    return resp


@app.route("/info", methods=["GET"])
def info():
    user = request.cookies.get("logged_in_user")
    if user:
        return {"user": user}, 200
    else:
        return {"err": "user not logged in"}, 401


@app.route("/auth", methods=["GET"])
def auth():
    user = request.cookies.get("logged_in_user")
    # print(user)
    if user:
        return "", 200
    else:
        return "", 401
