from fastapi import FastAPI, Request
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates
from modules import is_get

templates = Jinja2Templates(directory="templates")

app = FastAPI()
app.mount("/asset", StaticFiles(directory="static/asset"), name="static")


@app.route("/security/sign-in")
async def sign_in(req: Request):
    if is_get(req.method):
        return templates.TemplateResponse(request=req, name="sign-in.html")

    # Treat other as post


@app.route("/security/sign-up")
async def sign_up(req: Request):
    if is_get(req.method):
        return templates.TemplateResponse(request=req, name="sign-up.html")

    # Treat other as post


@app.get("/")
async def index(req: Request):
    return templates.TemplateResponse(request=req, name="index.html")
