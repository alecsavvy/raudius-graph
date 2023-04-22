docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate \
    -i https://discoveryprovider.audius.co/v1/swagger.json \
    -g graphql-schema \
    -o /local/gql \
    --additional-properties=packageName=schemas,