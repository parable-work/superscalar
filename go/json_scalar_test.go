package superscalar

import (
	"database/sql"
	"database/sql/driver"
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

var _ sql.Scanner = (*GenericJSON)(nil)
var _ driver.Valuer = GenericJSON(nil)

func TestGenericJSON_AllRootsAndPresence(t *testing.T) {
	type envelope struct {
		Payload GenericJSON `json:"payload"`
	}

	for _, root := range []string{`{"k":1}`, `[1,2]`, `"text"`, `42`, `true`, `null`} {
		var decoded envelope
		require.NoError(t, json.Unmarshal([]byte(`{"payload":`+root+`}`), &decoded))
		assert.Equal(t, root, string(decoded.Payload))

		encoded, err := json.Marshal(decoded)
		require.NoError(t, err)
		assert.JSONEq(t, `{"payload":`+root+`}`, string(encoded))
	}

	var absent envelope
	require.NoError(t, json.Unmarshal([]byte(`{}`), &absent))
	assert.Nil(t, absent.Payload)

	var explicitNull envelope
	require.NoError(t, json.Unmarshal([]byte(`{"payload":null}`), &explicitNull))
	assert.Equal(t, "null", string(explicitNull.Payload))

	var list []GenericJSON
	require.NoError(t, json.Unmarshal([]byte(`[null,{"k":1}]`), &list))
	require.Len(t, list, 2)
	assert.Equal(t, "null", string(list[0]))
	assert.JSONEq(t, `{"k":1}`, string(list[1]))

	var byName map[string]GenericJSON
	require.NoError(t, json.Unmarshal([]byte(`{"empty":null,"set":true}`), &byName))
	assert.Equal(t, "null", string(byName["empty"]))
	assert.Equal(t, "true", string(byName["set"]))

	valid, errors := explicitNull.Payload.ValidateRequired()
	assert.True(t, valid)
	assert.Nil(t, errors)
	valid, errors = absent.Payload.ValidateRequired()
	assert.False(t, valid)
	require.Len(t, errors, 1)
	assert.Equal(t, "required", errors[0].Validator)
}

func TestGenericJSON_SQLScanAndValuePreserveNull(t *testing.T) {
	for _, root := range []string{`{"k":1}`, `[1,2]`, `"text"`, `42`, `true`, `null`} {
		var value GenericJSON
		require.NoError(t, value.Scan([]byte(root)))
		assert.Equal(t, root, string(value))

		driverValue, err := value.Value()
		require.NoError(t, err)
		assert.Equal(t, []byte(root), driverValue)
	}

	var compacted GenericJSON
	require.NoError(t, compacted.Scan(`["value", 2]`))
	assert.Equal(t, `["value",2]`, string(compacted))

	var fromString GenericJSON
	require.NoError(t, fromString.Scan(`null`))
	assert.Equal(t, "null", string(fromString))

	var fromRawMessage GenericJSON
	require.NoError(t, fromRawMessage.Scan(json.RawMessage(`null`)))
	assert.Equal(t, "null", string(fromRawMessage))

	var sqlNull GenericJSON
	require.NoError(t, sqlNull.Scan(nil))
	assert.Nil(t, sqlNull)
	driverValue, err := sqlNull.Value()
	require.NoError(t, err)
	assert.Nil(t, driverValue)

	var invalid GenericJSON
	assert.Error(t, invalid.Scan([]byte(`{invalid`)))
	assert.Error(t, invalid.Scan(42))
	assert.Error(t, (*GenericJSON)(nil).Scan([]byte(`null`)))
	_, err = GenericJSON(`{invalid`).Value()
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
