const Users = require("../models/userModel");
const bodyParser = require("body-parser");

// This is how to curl the post request
// curl -X POST \
//   http://localhost:3000/api/user \
//   -H 'Content-Type: application/json' \
//   -d '{
// 	"username": "hello",
// 	"address": "dwded"
// }'

module.exports = (app) => {
  app.use(bodyParser.json());
  app.use(bodyParser.urlencoded({ extended: true }));

  app.get("/api/users", (req, res) => {
    Users.find({})
      .then((users) => res.send(users))
      .catch((error) => {
        res.status(400).send({
          message: error.message,
        });
      });
  });

  app.post("/api/users", (req, res) => {
    let user = Users({
      username: req.body.username,
      address: req.body.address,
    });
    user
      .save()
      .then(() => {
        res.send("User created successfully!");
      })
      .catch((error) => {
        res.status(400).send({
          message: error.message,
        });
      });
  });
};
