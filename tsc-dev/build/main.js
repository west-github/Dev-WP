"use strict";
// import { create_user } from "$lib";
// import { hello } from "./apps/users-service/lib";
class Abstraction {
    bridge;
    constructor(bridge) {
        this.bridge = bridge;
    }
}
class BridgeOperation {
    operation(name) {
        console.log("This is from bridge operation with value " + name);
    }
}
class BridgeImplementation extends Abstraction {
    use_bridge() {
        this.bridge.operation("From BridgeImplementation");
    }
}
(() => {
    // let west = create_user("West");
    // function create_vehicle(name: string): Vehicle {
    //     return { name: name };
    // }
    // console.log(west, create_vehicle("Ford"), hello);
    const bi = new BridgeImplementation(new BridgeOperation());
    bi.use_bridge();
})();
