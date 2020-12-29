FROM rust:1.48-alpine

LABEL "com.github.actions.name"="GitHub Action for Cargo doc"
LABEL "com.github.actions.description"="Generate documentation and push to docs branch"
LABEL "com.github.actions.icon"="upload-cloud"
LABEL "com.github.actions.color"="purple"

COPY . .
RUN apk add --no-cache bash
RUN apk add --no-cache git
RUN chmod +x /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
