import type { VolumePercent } from "../volumeController";

export function getNumber(num: unknown) {
  const number = Number(num);
  return Number.isInteger(number) ? number : undefined;
}

export async function sleep(timeMs: number) {
  return new Promise((resolve, reject) => {
    setTimeout(resolve, timeMs, true);
  });
}

export function isEnumValue<T extends string>(
  enumObject: Record<string, T>,
  value: unknown
): value is T {
  return Object.values(enumObject).includes(value as T);
}

export function getIncludes<T extends readonly unknown[]>(
  enumArray: T,
  value: unknown
): T[number] | null {
  if (!enumArray.includes(value as T[number])) {
    return null;
  }
  return (value as T[number]);
}

export function getEnumIncludes<E extends Record<string, string>>(
  enumObject: E,
  value: unknown
): E[keyof E] | null {
  if (!isEnumValue(enumObject, value as E[keyof E])) {
    return null;
  }
  return value as E[keyof E];
}

export function isVolumePercent(value: number): value is VolumePercent {
  return value >= 0 && value <= 100;
}