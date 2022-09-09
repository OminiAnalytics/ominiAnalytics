/**
 * File: test.ts
 * Created Date: 08 Sep 2022
 * Author: realbacon
 * -----
 * Last Modified: 9/09/2022 12:04:22
 * Modified By: realbacon
 * -----
 * License  : MIT
 * -----
 **/
import browserTest from './browserTest';
import osTest from './osTest';

const main = async () => {
  await browserTest.run();
  await osTest.run();
};
main();
