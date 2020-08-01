const { solve } = require('../pkg/math_funcs_lib.js');
const { fib_dp } = require('../pkg/math_funcs_lib.js');
const { ap1_sum_d } = require('../pkg/math_funcs_lib.js');
const { ap1_d } = require('../pkg/math_funcs_lib.js');
const { ap2_d } = require('../pkg/math_funcs_lib.js');
const { ap3_d } = require('../pkg/math_funcs_lib.js');
const { ap4_d } = require('../pkg/math_funcs_lib.js');
const { gp1_sum_d } = require('../pkg/math_funcs_lib.js');
const { gp1_d } = require('../pkg/math_funcs_lib.js');

console.log( solve([2., 5., -3.]) );
console.log( fib_dp([40]) );
console.log( ap1_sum_d([2, 5, 3]) );
console.log( ap1_d([2, 5, 3]) );
console.log( ap2_d([2, 5, 3]) );
console.log( ap3_d([2, 5, 3]) );
console.log( ap4_d([2, 5, 3]) );
console.log( gp1_sum_d([2, 5, 3]) );
console.log( gp1_d([2, 5, 3]) );
