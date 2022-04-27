import _init, {run_app} from "./pkg/index.js";
var web_pb = require("./web_pb.js");

const main = () => {
  console.log(web_pb);
  run_app();
}

main();
