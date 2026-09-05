package superscalar

type ValidationError struct {
	Validator string `json:"validator"`
	Message   string `json:"message"`
}

type ValidationErrors map[string]interface{}

func NewValidationErrors() ValidationErrors {
	return make(ValidationErrors)
}

func (ve ValidationErrors) AddFieldError(field string, validator string, message string) {
	if ve[field] == nil {
		ve[field] = []ValidationError{}
	}

	errors := ve[field].([]ValidationError)
	errors = append(errors, ValidationError{
		Validator: validator,
		Message:   message,
	})
	ve[field] = errors
}

func (ve ValidationErrors) AddNestedError(field string, nestedErrors ValidationErrors) {
	ve[field] = nestedErrors
}

func (ve ValidationErrors) SetFieldErrors(field string, errors []ValidationError) {
	if len(errors) > 0 {
		ve[field] = errors
	}
}

func (ve ValidationErrors) HasErrors() bool {
	return len(ve) > 0
}

func (ve ValidationErrors) GetFieldErrors(field string) []ValidationError {
	if errors, ok := ve[field].([]ValidationError); ok {
		return errors
	}
	return nil
}

func (ve ValidationErrors) GetNestedErrors(field string) ValidationErrors {
	if nested, ok := ve[field].(ValidationErrors); ok {
		return nested
	}
	return nil
}
