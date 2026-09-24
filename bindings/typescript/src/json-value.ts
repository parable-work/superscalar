/** Any value representable by JSON, including an explicit null root. */
export type JSONValue =
  | string
  | number
  | boolean
  | null
  | readonly JSONValue[]
  | { readonly [key: string]: JSONValue };

/**
 * Reports whether a host value is representable by JSON without lossy
 * coercion. In particular, it rejects undefined, non-finite numbers, sparse
 * arrays, non-string object keys, custom objects, and reference cycles.
 */
export function isJSONValue(value: unknown): value is JSONValue {
  try {
    return isJSONValueInner(value, new Set<object>());
  } catch {
    // Getters, proxies, or extreme nesting can throw during inspection. Those
    // values are not safe portable JSON inputs.
    return false;
  }
}

function isJSONValueInner(
  value: unknown,
  ancestors: Set<object>,
): value is JSONValue {
  if (
    value === null ||
    typeof value === "string" ||
    typeof value === "boolean"
  ) {
    return true;
  }
  if (typeof value === "number") {
    return Number.isFinite(value);
  }
  if (typeof value !== "object") {
    return false;
  }

  if (ancestors.has(value)) {
    return false;
  }
  ancestors.add(value);
  try {
    if (Array.isArray(value)) {
      if (
        Object.getOwnPropertySymbols(value).length > 0 ||
        Object.keys(value).length !== value.length
      ) {
        return false;
      }
      for (let index = 0; index < value.length; index += 1) {
        if (!isJSONValueInner(value[index], ancestors)) {
          return false;
        }
      }
      return true;
    }

    const prototype = Object.getPrototypeOf(value);
    if (prototype !== Object.prototype && prototype !== null) {
      return false;
    }
    if (Object.getOwnPropertySymbols(value).length > 0) {
      return false;
    }
    for (const child of Object.values(value)) {
      if (!isJSONValueInner(child, ancestors)) {
        return false;
      }
    }
    return true;
  } finally {
    ancestors.delete(value);
  }
}

// Compile-time contract: every JSON root category remains assignable.
type Assert<T extends true> = T;
type _StringRoot = Assert<string extends JSONValue ? true : false>;
type _NumberRoot = Assert<number extends JSONValue ? true : false>;
type _BooleanRoot = Assert<boolean extends JSONValue ? true : false>;
type _NullRoot = Assert<null extends JSONValue ? true : false>;
type _ArrayRoot = Assert<readonly JSONValue[] extends JSONValue ? true : false>;
type _ObjectRoot = Assert<
  { readonly key: JSONValue } extends JSONValue ? true : false
>;
