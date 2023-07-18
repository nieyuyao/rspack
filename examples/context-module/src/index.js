const context = require.context('./', false, /mod/)

console.log(context('./mod1.js').name)
