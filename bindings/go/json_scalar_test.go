package superscalar

import (
	"encoding/json"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestJSON_MarshalJSON_Empty(t *testing.T) {
	var j GenericJSON
	data, err := json.Marshal(j)
	require.NoError(t, err)
	assert.Equal(t, "null", string(data))
}

func TestJSON_MarshalJSON_EmptySlice(t *testing.T) {
	j := GenericJSON([]byte{})
	data, err := json.Marshal(j)
	require.NoError(t, err)
	assert.Equal(t, "null", string(data))
}

func TestJSON_MarshalJSON_Object(t *testing.T) {
	j := GenericJSON(`{"key":"value"}`)
	data, err := json.Marshal(j)
	require.NoError(t, err)
	assert.Equal(t, `{"key":"value"}`, string(data))
}

func TestJSON_UnmarshalJSON_Object(t *testing.T) {
	var j GenericJSON
	require.NoError(t, json.Unmarshal([]byte(`{"key":"value"}`), &j))
	assert.Equal(t, `{"key":"value"}`, string(j))
}

func TestJSON_UnmarshalJSON_Array(t *testing.T) {
	var j GenericJSON
	require.NoError(t, json.Unmarshal([]byte(`[1,2,3]`), &j))
	assert.Equal(t, `[1,2,3]`, string(j))
}

func TestJSON_UnmarshalJSON_String(t *testing.T) {
	var j GenericJSON
	require.NoError(t, json.Unmarshal([]byte(`"hello"`), &j))
	assert.Equal(t, `"hello"`, string(j))
}

func TestJSON_UnmarshalJSON_Invalid(t *testing.T) {
	var j GenericJSON
	err := json.Unmarshal([]byte(`{invalid`), &j)
	assert.Error(t, err)
}

func TestJSON_RoundTrip(t *testing.T) {
	original := GenericJSON(`{"nested":{"arr":[1,2]}}`)
	data, err := json.Marshal(original)
	require.NoError(t, err)

	var restored GenericJSON
	require.NoError(t, json.Unmarshal(data, &restored))
	assert.JSONEq(t, string(original), string(restored))
}
