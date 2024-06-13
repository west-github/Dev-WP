import express, { Request, Response } from "express";
import cors from "cors";


const app = express();
const port = 3000;

app.use(cors({ origin: "http://localhost:3001" }));

app.get("/", (req: Request, res: Response) => {
    const _res = {
        message: "Fuck we in express world",
        metadata: {
            data: null
        }
    };
    res.json(_res);
});

try {
    app.listen(port, on_serve);

    function on_serve() {
        console.log(`[server] listening on port ${port}`);
    }
} catch (error) {
    console.log(`[Error]: ${error}`);
}