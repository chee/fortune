var fortunes = require("fs").readFileSync("./fortunes").toString().trim().split(/\n%\n/);
console.log(fortunes[Math.random() * fortunes.length | 0])
