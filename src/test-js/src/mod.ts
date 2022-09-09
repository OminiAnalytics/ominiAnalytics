/**
 * File: mod.ts
 * Created Date: 08 Sep 2022
 * Author: realbacon
 * -----
 * Last Modified: 9/09/2022 12:03:12
 * Last Modified: 9/09/2022 12:03:12
 * Modified By: realbacon
 * License  : MIT
 * -----
 **/

/* eslint-disable @typescript-eslint/restrict-plus-operands */
/**
 * File: test.js
 * Created Date: 08 Sep 2022
 * Author: realbacon
 * -----
 * Last Modified: 9/09/2022 12:03:12
 * Modified By: realbacon
 * -----
 * @license MIT
 * -----
 **/
/*
This is the test suite for OminiTracker.
*/
'use strict';
import * as Color from 'colors.ts';
Color.enable();
/**
 * Otest class.
 * @param {string} name Test name
 * @example const test = new Otest("example test")
 */
class Otest {
  name: string;
  // eslint-disable-next-line @typescript-eslint/ban-types
  tests: {[key: string]: {func: Function; comp: unknown}};
  passed: number;
  failed: number;
  constructor(name: string) {
    this.name = name;
    this.tests = {};
    this.passed = 0;
    this.failed = 0;
  }

  /**
   * Test the function and compare to expect function
   * @param {Function} func The function to test
   * @param {*} expect The expected result
   * @param {string} name The test name
   */
  // eslint-disable-next-line @typescript-eslint/ban-types
  async testResult(func: Function, expect: unknown): Promise<void> {
    // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
    let result = await func();
    if (typeof result === 'object') {
      result = JSON.stringify(result);
    }
    if (typeof expect === 'object') {
      expect = JSON.stringify(expect);
    }
    if (result === expect) {
      this.passed++;
      console.log('Passed'.bg_green.white + ' : ' + result);
    } else {
      this.failed++;
      // eslint-disable-next-line @typescript-eslint/restrict-template-expressions, @typescript-eslint/no-base-to-string
      console.log('Failed'.bg_red.white + ` expected ${expect}, got ${result}`);
    }
  }

  /**
   * Add a test
   * @param {string} name test name
   * @param {Function} func test function
   * @param {*} comp value to compare to
   */
  // eslint-disable-next-line @typescript-eslint/ban-types
  new(name: string, func: Function, comp: unknown): void {
    this.tests[name] = {func, comp};
  }

  /**
   * Run the tests suite
   */
  async run(): Promise<void> {
    console.log(`Running ${this.name}...`.blue);
    for (const [name, test] of Object.entries(this.tests)) {
      console.log(`\n- Testing "${name.cyan}"`);
      await this.testResult(test.func, test.comp);
    }
    const sum = this.passed + this.failed;
    console.log('\n');
    if (this.passed > 0) {
      console.log(`${this.passed}/${sum} test(s) passed`.green);
    }
    if (this.failed > 0) {
      console.log(`${this.failed}/${sum} test(s) failed`.red);
    }
  }
}
export default Otest;
