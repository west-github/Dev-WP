import type { PageServerLoad } from "./$types";

const encoder = new TextEncoder();
export const load: PageServerLoad = ({ }) => {
    console.log("Load data here");


    console.log(encoder.encode("We gucci"));
    return {}
}