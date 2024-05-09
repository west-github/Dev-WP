/**
 *
 * @param {string} string_content
 * @returns {typeof string_to_json}
 */
function string_to_json(string_content) {
    return {
        data: string_content,
    };
}

/**
 *This function will return what we need can be any type
 * @param {string[]} array_data
 * @returns {string}
 */
function array_to_string(array_data) {
    return array_data.join(" ");
}

export default function () {
    const { data } = string_to_json("West");

    console.log(data);

    const { data: other } = string_to_json(array_to_string(["Other", "Type"]));

    console.log(other);
}
