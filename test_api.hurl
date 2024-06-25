## /api/test
## GET http://localhost:3000/api/test

# /api/user/all
GET http://localhost:3000/api/user/all


# /api/user/register
POST http://localhost:3000/api/user/register
Content-Type: application/json
{
  "name": "mic",
  "email": "test@test.com",
  "password": "12345"
}

POST http://localhost:3000/api/user/register
Content-Type: application/json
{
  "name": "mic2",
  "email": "test2@test.com",
  "password": "12345"
}

# /api/user/login
POST http://localhost:3000/api/user/login
# HTTP 200
Content-Type: application/json
[Captures]
uuid: jsonpath "$['uuid']"
{
  "email": "test@test.com",
  "password": "12345"
}

## /api/user/uuid/update
PUT http://localhost:3000/api/user/uuid/update
Content-Type: application/json
# uuid: 76e7fd27-a05c-49eb-a893-8a47d0205224,
uuid: {{uuid}},
{
  "name": "New Name"
}
