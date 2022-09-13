/**
 * File: test.js
 * Created Date: 08 Sep 2022
 * Author: realbacon
 * -----
 * Last Modified: 12/09/2022 10:49:11
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
browserTest.new(
  'Edge',
  () => {
    const testB = ominiBrowser;
    testB.setEnv(
      'Mozilla/5.0 (Linux; Android 10; ONEPLUS A6003) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.5195.79 Mobile Safari/537.36 EdgA/100.0.1185.50',
      undefined,
      {}
    );
    return testB.gbrowser();
  },
  {name: 'Edge', version: '100', fversion: '100.0.1185.50'}
);

browserTest.new(
  'Chrome Mobile',
  () => {
    const testB = ominiBrowser;
    testB.setEnv(
      'ozilla/5.0 (iPad; CPU OS 15_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/105.0.5195.100 Mobile/15E148 Safari/604.1',
      undefined,
      {}
    );
    return testB.gbrowser();
  },
  {name: 'Chrome', version: '105', fversion: '105.0.5195.100'}
);

export default browserTest;
