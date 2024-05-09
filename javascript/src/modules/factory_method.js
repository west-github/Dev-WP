function new_cat(name) {
    return {
        name,
        description: () => {
            return `Pet: ${name} category Cat`;
        },
    };
}

function new_dog(name) {
    return {
        name,
        description: () => {
            return `Pet: ${name} category Dog`;
        },
    };
}

function factory_method(type, name) {
    const animal_type = {
        cat: new_cat,
        dog: new_dog,
    };

    function d_type() {
        throw new Error(`No  pet with ${type}`);
    }

    return animal_type[type](name) ?? d_type();
}

class AbstractPet {
    name;

    constructor(name) {
        this.name = name;
    }

    description(type) {
        return `Pet: ${this.name} category ${type}`;
    }
}

class Dog extends AbstractPet {
    description() {
        return super.description("Dog");
    }
}

class Cat extends AbstractPet {
    description() {
        return super.description("Cat");
    }
}

class Pet {
    static create(type, name) {
        switch (type) {
            case "cat":
                return new Cat(name);

            case "dog":
                return new Dog(name);

            default:
                throw new Error(`No  pet with ${type}`);
        }
    }
}

export default function () {
    const pet_creator = factory_method("dog", "Bongo");

    console.log(pet_creator.name);
    console.log(pet_creator.description());

    const petcreator_class = Pet.create("cat", "Lucky");

    console.log(petcreator_class.name);
    console.log(petcreator_class.description());
}
