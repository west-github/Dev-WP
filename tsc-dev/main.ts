import { create_user } from "$lib";
import { hello } from "./apps/users-service/lib";

export type Vehicle = {
    name: string;
};

interface IBridge {
    operation(name: string): void;
}

abstract class Abstraction {
    protected bridge: IBridge;

    constructor(bridge: IBridge) {
        this.bridge = bridge;
    }

    abstract use_bridge(): void;
}

class BridgeOperation implements IBridge {
    operation(name: string): void {
        console.log("This is from bridge operation with value " + name);
    }
}

class BridgeImplementation extends Abstraction {
    use_bridge(): void {
        this.bridge.operation("From BridgeImplementation");
    }
}

(() => {
    let west = create_user("West");

    function create_vehicle(name: string): Vehicle {
        return { name: name };
    }

    console.log(west, create_vehicle("Ford"), hello);

    const bi = new BridgeImplementation(new BridgeOperation());

    bi.use_bridge();
})();
