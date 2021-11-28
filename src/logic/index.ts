/**
 * Constrains a number between a minmax range
 * @param value the value to constrain
 * @param min the minimum value
 * @param max the maximum value
 * @returns {number} of the resulting value
 */
export const minmax = (value: number, min: number, max: number) => Math.min(Math.max(+value, min), max);
