import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function convertNullToUndefined<T extends Record<string, any>>(obj: T): any {
  const result = {} as any;

  for (const key in obj) {
    result[key] =
      obj[key] === null
        ? undefined
        : typeof obj[key] === 'object' && !Array.isArray(obj[key])
          ? convertNullToUndefined(obj[key])
          : obj[key];
  }

  return result;
}
