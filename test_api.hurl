# /api/test
GET http://localhost:3000/api/test


# /api/user/insert
POST http://localhost:3000/api/user/insert
Content-Type: application/json

{
  "name": "mic",
  "email": "test@test.com",
  "password": "12345"
}
