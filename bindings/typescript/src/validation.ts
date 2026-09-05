/** A single validation error. */
export interface ValidationError {
  validator: string;
  message: string;
}

/**
 * Field-level validation errors keyed by field name. Null/missing means the
 * field passed; a field may have multiple failures (arrays) or nested structures.
 */
export type ValidationErrors = {
  [fieldName: string]: ValidationError[] | ValidationErrors;
};

/** Result of a validation operation: true if it passes, else ValidationErrors. */
export type ValidationResult = true | ValidationErrors;

/**
 * Result of a scalar validation: [valid, errors]. Errors is null when valid.
 * Callers (object/type validation) assign errors to the correct field name.
 */
export type ScalarValidationResult = [boolean, ValidationError[] | null];

/**
 * Converts an array of ValidationErrors to a ScalarValidationResult tuple.
 * @param errors - Array of validation errors
 * @returns Tuple of [valid, errors] where valid is true if no errors
 */
export function toScalarResult(errors: ValidationError[]): ScalarValidationResult {
  return errors.length === 0 ? [true, null] : [false, errors];
}

/**
 * Creates a new empty ValidationErrors object
 */
export function newValidationErrors(): ValidationErrors {
  return {};
}

/**
 * Adds a field error to ValidationErrors
 */
export function addFieldError(
  errors: ValidationErrors,
  fieldName: string,
  validator: string,
  message: string
): void {
  if (!errors[fieldName]) {
    errors[fieldName] = [];
  }

  const fieldErrors = errors[fieldName];
  if (Array.isArray(fieldErrors)) {
    fieldErrors.push({ validator, message });
  }
}

/**
 * Sets field errors from an array of ValidationError objects.
 * Used for composing object validation from scalar validation results.
 * Only sets the field if there are errors to add.
 */
export function setFieldErrors(
  errors: ValidationErrors,
  fieldName: string,
  fieldErrors: ValidationError[]
): void {
  if (fieldErrors.length > 0) {
    errors[fieldName] = fieldErrors;
  }
}

/**
 * Adds nested validation errors
 */
export function addNestedErrors(
  errors: ValidationErrors,
  fieldName: string,
  nestedErrors: ValidationErrors
): void {
  if (Object.keys(nestedErrors).length > 0) {
    errors[fieldName] = nestedErrors;
  }
}

/**
 * Checks if ValidationErrors has any errors
 */
export function hasErrors(errors: ValidationErrors): boolean {
  return Object.keys(errors).length > 0;
}

/**
 * Merges multiple ValidationErrors objects
 */
export function mergeValidationErrors(
  ...errorsList: ValidationErrors[]
): ValidationErrors {
  const result: ValidationErrors = {};

  for (const errors of errorsList) {
    for (const [key, value] of Object.entries(errors)) {
      if (Array.isArray(value)) {
        if (!result[key]) {
          result[key] = [];
        }
        if (Array.isArray(result[key])) {
          (result[key] as ValidationError[]).push(...value);
        }
      } else {
        result[key] = value;
      }
    }
  }

  return result;
}
