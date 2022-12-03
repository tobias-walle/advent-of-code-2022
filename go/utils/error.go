package utils

func PanicOnErr(e error) {
	if e != nil {
		panic(e)
	}
}
