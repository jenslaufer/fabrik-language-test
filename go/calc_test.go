package calc

import "testing"

func TestDouble(t *testing.T) {
	cases := []struct {
		in, want int
	}{
		{0, 0},
		{3, 6},
		{-2, -4},
	}
	for _, c := range cases {
		if got := Double(c.in); got != c.want {
			t.Errorf("Double(%d) = %d, want %d", c.in, got, c.want)
		}
	}
}

func TestTriple(t *testing.T) {
	cases := []struct {
		in, want int
	}{
		{0, 0},
		{4, 12},
		{-3, -9},
	}
	for _, c := range cases {
		if got := Triple(c.in); got != c.want {
			t.Errorf("Triple(%d) = %d, want %d", c.in, got, c.want)
		}
	}
}
