package superscalar

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestIpAddress_Value(t *testing.T) {
	ip := NetworkIpAddress("192.168.1.1")
	val, err := ip.Value()
	require.NoError(t, err)
	assert.Equal(t, "192.168.1.1", val)
}

func TestIpAddress_Scan_Nil(t *testing.T) {
	var ip NetworkIpAddress
	require.NoError(t, ip.Scan(nil))
	assert.Equal(t, NetworkIpAddress(""), ip)
}

func TestIpAddress_Scan_Bytes(t *testing.T) {
	var ip NetworkIpAddress
	require.NoError(t, ip.Scan([]byte("10.0.0.1")))
	assert.Equal(t, NetworkIpAddress("10.0.0.1"), ip)
}

func TestIpAddress_Scan_String(t *testing.T) {
	var ip NetworkIpAddress
	require.NoError(t, ip.Scan("172.16.0.1"))
	assert.Equal(t, NetworkIpAddress("172.16.0.1"), ip)
}

func TestIpAddress_Scan_UnsupportedType(t *testing.T) {
	var ip NetworkIpAddress
	err := ip.Scan(42)
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "unsupported type")
}
