/**
 * Constrains a number between a minmax range
 * @param value the value to constrain
 * @param min the minimum value
 * @param max the maximum value
 * @returns {number} of the resulting value
 */
export const minmax = (value: number, min: number, max: number) => Math.min(Math.max(+value, min), max);

/**
 * Gets the value a :root css variable holds
 * @param name the name of the :root  css variable to get from
 * @returns {string} of what the :root css variable holds
 */
export const rootHexColor = (name: string) => getComputedStyle(document.documentElement).getPropertyValue(name).trim();
