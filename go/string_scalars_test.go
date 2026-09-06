package superscalar

import (
	"database/sql/driver"
	"testing"
)

func TestSlug_Scan(t *testing.T) {
	tests := []struct {
		name    string
		src     interface{}
		want    IdentitySlug
		wantErr bool
	}{
		{"nil", nil, "", false},
		{"bytes", []byte("atlassian"), "atlassian", false},
		{"string", "jira", "jira", false},
		{"unsupported", 123, "", true},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var v IdentitySlug
			err := v.Scan(tt.src)
			if (err != nil) != tt.wantErr {
				t.Errorf("Slug.Scan() error = %v, wantErr %v", err, tt.wantErr)
				return
			}
			if !tt.wantErr && v != tt.want {
				t.Errorf("Slug.Scan() = %q, want %q", v, tt.want)
			}
		})
	}
}

func TestSlug_Value(t *testing.T) {
	v := IdentitySlug("my-slug")
	got, err := v.Value()
	if err != nil {
		t.Errorf("Slug.Value() error = %v", err)
		return
	}
	if got != "my-slug" {
		t.Errorf("Slug.Value() = %v, want my-slug", got)
	}
	_, ok := got.(string)
	if !ok {
		t.Errorf("Slug.Value() should return driver.Value (string), got %T", got)
	}
}

func TestName_Scan(t *testing.T) {
	tests := []struct {
		name    string
		src     interface{}
		want    IdentityName
		wantErr bool
	}{
		{"nil", nil, "", false},
		{"bytes", []byte("Atlassian"), "Atlassian", false},
		{"string", "Acme Corp", "Acme Corp", false},
		{"unsupported", 3.14, "", true},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var v IdentityName
			err := v.Scan(tt.src)
			if (err != nil) != tt.wantErr {
				t.Errorf("Name.Scan() error = %v, wantErr %v", err, tt.wantErr)
				return
			}
			if !tt.wantErr && v != tt.want {
				t.Errorf("Name.Scan() = %q, want %q", v, tt.want)
			}
		})
	}
}

func TestName_Value(t *testing.T) {
	v := IdentityName("Acme Corp")
	got, err := v.Value()
	if err != nil {
		t.Errorf("Name.Value() error = %v", err)
		return
	}
	if got != "Acme Corp" {
		t.Errorf("Name.Value() = %v, want Acme Corp", got)
	}
	if _, ok := got.(string); !ok {
		t.Errorf("Name.Value() should return driver.Value (string), got %T", got)
	}
}

var _ driver.Valuer = IdentitySlug("")
var _ driver.Valuer = IdentityName("")
