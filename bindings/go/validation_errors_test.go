package superscalar

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestNewValidationErrors(t *testing.T) {
	ve := NewValidationErrors()
	assert.NotNil(t, ve)
	assert.False(t, ve.HasErrors())
}

func TestValidationErrors_AddFieldError(t *testing.T) {
	ve := NewValidationErrors()
	ve.AddFieldError("email", "required", "Email is required")

	assert.True(t, ve.HasErrors())
	errors := ve.GetFieldErrors("email")
	assert.Len(t, errors, 1)
	assert.Equal(t, "required", errors[0].Validator)
	assert.Equal(t, "Email is required", errors[0].Message)
}

func TestValidationErrors_AddMultipleErrors(t *testing.T) {
	ve := NewValidationErrors()
	ve.AddFieldError("email", "required", "Email is required")
	ve.AddFieldError("email", "format", "Email format is invalid")
	ve.AddFieldError("password", "minLength", "Password must be at least 8 characters")

	assert.True(t, ve.HasErrors())

	emailErrors := ve.GetFieldErrors("email")
	assert.Len(t, emailErrors, 2)

	passwordErrors := ve.GetFieldErrors("password")
	assert.Len(t, passwordErrors, 1)
}

func TestValidationErrors_AddNestedError(t *testing.T) {
	nested := NewValidationErrors()
	nested.AddFieldError("street", "required", "Street is required")

	ve := NewValidationErrors()
	ve.AddNestedError("address", nested)

	assert.True(t, ve.HasErrors())
	addressErrors := ve.GetNestedErrors("address")
	assert.NotNil(t, addressErrors)
	assert.True(t, addressErrors.HasErrors())
	assert.Len(t, addressErrors.GetFieldErrors("street"), 1)
}

func TestValidationErrors_SetFieldErrors(t *testing.T) {
	ve := NewValidationErrors()

	errors := []ValidationError{
		{Validator: "pattern", Message: "Invalid format"},
		{Validator: "minLength", Message: "Too short"},
	}
	ve.SetFieldErrors("email", errors)

	assert.True(t, ve.HasErrors())
	fieldErrors := ve.GetFieldErrors("email")
	assert.Len(t, fieldErrors, 2)
	assert.Equal(t, "pattern", fieldErrors[0].Validator)
	assert.Equal(t, "minLength", fieldErrors[1].Validator)
}

func TestValidationErrors_SetFieldErrors_Empty(t *testing.T) {
	ve := NewValidationErrors()

	ve.SetFieldErrors("email", []ValidationError{})

	assert.False(t, ve.HasErrors())
	assert.Nil(t, ve.GetFieldErrors("email"))
}

func TestValidationErrors_SetFieldErrors_Nil(t *testing.T) {
	ve := NewValidationErrors()

	ve.SetFieldErrors("email", nil)

	assert.False(t, ve.HasErrors())
	assert.Nil(t, ve.GetFieldErrors("email"))
}

func TestValidationErrors_GetFieldErrors_NonExistent(t *testing.T) {
	ve := NewValidationErrors()
	errors := ve.GetFieldErrors("nonexistent")
	assert.Nil(t, errors)
}

func TestValidationErrors_GetNestedErrors_NonExistent(t *testing.T) {
	ve := NewValidationErrors()
	nested := ve.GetNestedErrors("nonexistent")
	assert.Nil(t, nested)
}

func TestValidationErrors_GetNestedErrors_WrongType(t *testing.T) {
	ve := NewValidationErrors()
	ve.AddFieldError("field", "validator", "message")

	nested := ve.GetNestedErrors("field")
	assert.Nil(t, nested)
}

func TestValidationErrors_GetFieldErrors_WrongType(t *testing.T) {
	ve := NewValidationErrors()
	nestedVe := NewValidationErrors()
	nestedVe.AddFieldError("inner", "required", "Required")
	ve.AddNestedError("nested", nestedVe)

	errors := ve.GetFieldErrors("nested")
	assert.Nil(t, errors)
}
