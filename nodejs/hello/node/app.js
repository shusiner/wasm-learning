import init, { say } from './hello_lib.js';

(async () => {
  await init();
  console.log(say('World!'));
})();