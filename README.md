# Minimal Microservice example

Minimal API and UI microservice example

# Build
UI:  
`docker build -t minimal-ui:<tag> -f Dockerfile --target ui-build .`  
API:  
`docker build -t minimal-api:<tag> -f Dockerfile --target api-build --target api-release .`  
