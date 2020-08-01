const { solve } = require('../pkg/math_funcs_lib.js');
const { fib_rec } = require('../pkg/math_funcs_lib.js');
const { fib_dp } = require('../pkg/math_funcs_lib.js');
const { fib_dp_f } = require('../pkg/math_funcs_lib.js');

console.log( solve([2., 5., -3.]) );
console.log( fib_rec([6]) );
console.log( fib_dp([80]) );
console.log( fib_dp_f([80]) );
