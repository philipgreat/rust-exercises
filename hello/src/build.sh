env GOOS=linux GOARCH=arm  GOARM=5  go build -ldflags "-s -w" -o hello-arm7  main.go

