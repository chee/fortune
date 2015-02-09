package main

import (
	"fmt"
	"io/ioutil"
	"math/rand"
	"strings"
	"time"
)

func main() {
	rand.Seed(time.Now().UTC().UnixNano() + 100);
	text, _ := ioutil.ReadFile("fortunes")
	fortunes := strings.Split(string(text), "\n%\n")
	fmt.Println(fortunes[rand.Intn(len(fortunes))])
}
