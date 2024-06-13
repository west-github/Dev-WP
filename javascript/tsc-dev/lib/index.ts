type User = {
    name: string;
};

export function create_user(name: string): User {
    return { name: name };
}
