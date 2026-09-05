package superscalar

import (
	"strings"
	"testing"
)

var attackSeeds = []string{
	"",
	"'; DROP TABLE users; --",
	"' UNION SELECT * FROM users --",
	"' OR '1'='1",
	"<script>alert('xss')</script>",
	"<img src=x onerror=alert(1)>",
	"\" onmouseover=\"alert(1)\"",
	"valid\x00malicious",
	"../../../etc/passwd",
	"%00",
	"\u0410\u0412\u0421",
	`{"$gt": ""}`,
	"\n\r\t",
	"\xff\xfe",
}

func longString(n int) string {
	return strings.Repeat("a", n)
}

func FuzzValidateEmail(f *testing.F) {
	f.Add("user@example.com")
	f.Add("test@mail.example.com")
	f.Add("TEST@EXAMPLE.COM")
	f.Add("  user@example.com  ")
	f.Add(" User@Example.COM ")
	f.Add("")
	f.Add("not-an-email")
	f.Add("a@b.c")
	f.Add(longString(10000) + "@example.com")
	for _, s := range attackSeeds {
		f.Add(s)
	}

	f.Fuzz(func(t *testing.T, input string) {
		e := ContactEmail(input)
		e.Validate()
		ValidateEmail(input)
	})
}

func FuzzValidateColor(f *testing.F) {
	f.Add("#FF5733FF")
	f.Add("#000000FF")
	f.Add("#FFFFFFFF")
	f.Add("#00000000")
	f.Add("#FF5733")
	f.Add("#F00")
	f.Add("red")
	f.Add("transparent")
	f.Add("rgb(255, 0, 0)")
	f.Add("rgba(255, 0, 0, 0.5)")
	f.Add("hsl(0, 100%, 50%)")
	f.Add("hsla(0, 100%, 50%, 0.5)")
	f.Add("")
	f.Add("notacolor")
	f.Add("#GGG")
	f.Add(longString(10000))
	for _, s := range attackSeeds {
		f.Add(s)
	}

	f.Fuzz(func(t *testing.T, input string) {
		c := DesignColor(input)
		c.Validate()
		ValidateColor(input)
		NormalizeColor(input)
	})
}

func FuzzValidateSlug(f *testing.F) {
	f.Add("atlassian")
	f.Add("my-slug")
	f.Add("jira")
	f.Add("a")
	f.Add("")
	f.Add("acme")
	f.Add("admin")
	f.Add(longString(10000))
	for _, s := range attackSeeds {
		f.Add(s)
	}

	f.Fuzz(func(t *testing.T, input string) {
		s := IdentitySlug(input)
		s.Validate()
		s.ValidateRequired()
	})
}

func FuzzValidateUrl(f *testing.F) {
	f.Add("https://example.com")
	f.Add("http://www.example.com/path?q=1")
	f.Add("https://sub.domain.example.com/path/to/resource")
	f.Add("")
	f.Add("not-a-url")
	f.Add("ftp://example.com")
	f.Add(longString(10000))
	for _, s := range attackSeeds {
		f.Add(s)
	}

	f.Fuzz(func(t *testing.T, input string) {
		u := NetworkUrl(input)
		u.Validate()
		u.ValidateRequired()
	})
}

func FuzzValidateName(f *testing.F) {
	f.Add("Test User")
	f.Add("Atlassian")
	f.Add("Acme Corp")
	f.Add("AB")
	f.Add("")
	f.Add("A")
	f.Add(longString(10000))
	for _, s := range attackSeeds {
		f.Add(s)
	}

	f.Fuzz(func(t *testing.T, input string) {
		n := IdentityName(input)
		n.Validate()
		n.ValidateRequired()
	})
}

func FuzzValidatePassword(f *testing.F) {
	f.Add("password123")
	f.Add("12345678")
	f.Add("short")
	f.Add("")
	f.Add(longString(10000))
	for _, s := range attackSeeds {
		f.Add(s)
	}

	f.Fuzz(func(t *testing.T, input string) {
		p := AuthPassword(input)
		p.Validate()
		p.ValidateRequired()
	})
}

func FuzzValidatePhoneNumber(f *testing.F) {
	f.Add("+14155552671")
	f.Add("+442071838750")
	f.Add("+33123456789")
	f.Add("+493012345678")
	f.Add("+61212345678")
	f.Add("+81312345678")
	f.Add("+1 (415) 555-2671")
	f.Add("")
	f.Add("4155552671")
	f.Add("123")
	f.Add(longString(10000))
	for _, s := range attackSeeds {
		f.Add(s)
	}

	f.Fuzz(func(t *testing.T, input string) {
		ValidatePhoneNumber(input)

		p := ContactPhoneNumber(input)
		p.Validate()
		p.ValidateRequired()
	})
}

func FuzzValidateUUID(f *testing.F) {
	f.Add("YQJpYwUwvbaLOwTUr4thA")
	f.Add("123e4567-e89b-12d3-a456-426614174000")
	f.Add("0")
	f.Add("")
	f.Add("not-a-uuid")
	f.Add(longString(10000))
	for _, s := range attackSeeds {
		f.Add(s)
	}

	f.Fuzz(func(t *testing.T, input string) {
		u, err := ParseUUID(input)
		if err == nil {
			u.Validate()
			u.ValidateRequired()
			u.String()
			u.IsZero()
		}
	})
}

func FuzzValidateDuration(f *testing.F) {
	f.Add("30s")
	f.Add("1h30m")
	f.Add("500ms")
	f.Add("1m")
	f.Add("")
	f.Add("invalid")
	f.Add(longString(10000))
	for _, s := range attackSeeds {
		f.Add(s)
	}

	f.Fuzz(func(t *testing.T, input string) {
		d, err := ParseDuration(input)
		if err == nil {
			d.Validate()
			d.ValidateRequired()
		}
	})
}

func FuzzValidateCronExpression(f *testing.F) {
	f.Add("0 0 * * *")
	f.Add("*/5 * * * *")
	f.Add("0 12 * * MON-FRI")
	f.Add("30 4 1,15 * *")
	f.Add("")
	f.Add("invalid cron")
	f.Add(longString(10000))
	for _, s := range attackSeeds {
		f.Add(s)
	}

	f.Fuzz(func(t *testing.T, input string) {
		c := TemporalCronExpression(input)
		c.Validate()
		c.ValidateRequired()
	})
}

func FuzzValidateDnsLabel(f *testing.F) {
	f.Add("mycompany")
	f.Add("acme-corp")
	f.Add("acme.okta.com")
	f.Add("https://sunrun.my.salesforce.com/path")
	f.Add("acme.okta.com:8080")
	f.Add("")
	f.Add("-acme")
	f.Add(longString(10000))
	for _, s := range attackSeeds {
		f.Add(s)
	}

	f.Fuzz(func(t *testing.T, input string) {
		l := NetworkDnsLabel(input)
		l.Validate()
		l.ValidateRequired()

		normalized, err := NormalizeNetworkDnsLabel(input)
		if err == nil {
			// Normalize is a fixed point: renormalizing must not change it.
			again, err := NormalizeNetworkDnsLabel(normalized)
			if err != nil {
				t.Fatalf("renormalize errored: %v", err)
			}
			if again != normalized {
				t.Fatalf("normalize not a fixed point: %q -> %q -> %q", input, normalized, again)
			}
		}
	})
}
