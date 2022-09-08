/**
 * File: test.js
 * Created Date: 08 Sep 2022
 * Author: realbacon
 * -----
 * Last Modified: 8/09/2022 06:27:47
 * Modified By: realbacon
 * -----
 * @license MIT
 * -----
 **/

import Otest from './mod';
// eslint-disable-next-line node/no-unpublished-import
import {ominiBrowser} from '../../jslib/src/collect/browser';
const browserTest = new Otest('Browser Test');

// Chrome browser test
browserTest.new(
  'Chrome',
  () => {
    const testB = ominiBrowser;
    testB.setEnv(
      'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36',
      undefined,
      {}
    );
    return testB.gbrowser();
  },
  {name: 'Chrome', version: '105', fversion: '105.0.0.0'}
);
// Firefox browser test
browserTest.new(
  'Firefox',
  () => {
    const testB = ominiBrowser;
    testB.setEnv(
      'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:104.0) Gecko/20100101 Firefox/104.0',
      undefined,
      {}
    );
    return testB.gbrowser();
  },
  {name: 'Firefox', version: '104', fversion: '104.0'}
);
// Safari browser test
browserTest.new(
  'Safari',
  () => {
    const testB = ominiBrowser;
    testB.setEnv(
      'Mozilla/5.0 (Macintosh; Intel Mac OS X 12_5_1) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.6 Safari/605.1.15',
      undefined,
      {}
    );
    return testB.gbrowser();
  },
  {name: 'Safari', version: '605', fversion: '605.1.15'}
);
// Opera browser test
browserTest.new(
  'Opera',
  () => {
    const testB = ominiBrowser;
    testB.setEnv(
      'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36 OPR/90.0.4480.84',
      undefined,
      {}
    );
    return testB.gbrowser();
  },
  {name: 'Opera', version: '90', fversion: '90.0.4480.84'}
);
browserTest.new(
  'Yandex',
  () => {
    const testB = ominiBrowser;
    testB.setEnv(
      'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 YaBrowser/22.7.5 Yowser/2.5 Safari/537.36',
      undefined,
      {}
    );
    return testB.gbrowser();
  },
  {name: 'Yandex', version: '22', fversion: '22.7.5'}
);

browserTest.run();
