/**
 * File: osTest.ts
 * Created Date: 08 Sep 2022
 * Author: realbacon
 * -----
 * Last Modified: 12/09/2022 10:54:31
 * Modified By: realbacon
 * -----
 * License  : MIT
 * -----
 **/

// eslint-disable-next-line node/no-unpublished-import
import {ominiOs} from '../../jslib/src/collect/opersys';
import Otest from './mod';
const osTest = new Otest('OS Test');
osTest.new(
  'Windows',
  () => {
    let testO = ominiOs;
    testO = testO.setEnv(
      'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome',
      undefined
    );
    const r = testO.wrap();

    return r;
  },
  {name: 'Windows', version: '10', typ: 'desktop', arch: 'x64'}
);
osTest.new(
  'Iphone',
  () => {
    let testO = ominiOs;
    testO = testO.setEnv(
      'Mozilla/5.0 (iPhone; CPU iPhone OS 15_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.5 Mobile/15E148 Safari/604.1',
      undefined
    );
    const r = testO.wrap();

    return r;
  },
  {name: 'iPhone', version: '15_5', typ: 'mobile', arch: 'x64'}
);
osTest.new(
  'MacOS',
  () => {
    let testO = ominiOs;
    testO = testO.setEnv(
      'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_6_8) AppleWebKit/534.59.10 (KHTML, like Gecko) Version/5.1.9 Safari/534.59.10',
      undefined
    );
    const r = testO.wrap();

    return r;
  },
  {name: 'macOS', version: '10.6', typ: 'desktop', arch: 'x64'}
);
osTest.new(
  'ChromeOS',
  () => {
    let testO = ominiOs;
    testO = testO.setEnv(
      'Mozilla/5.0 (X11; CrOS armv7l 14816.131.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36',
      undefined
    );
    const r = testO.wrap();

    return r;
  },
  {name: 'ChromeOS', version: '14816.131.0', typ: 'desktop', arch: 'x64'}
);
osTest.new(
  'Windows Phone',
  () => {
    let testO = ominiOs;
    testO = testO.setEnv(
      'Mozilla/5.0 (Mobile; Windows Phone 8.1; Android 4.0; ARM; Trident/7.0; Touch; rv:11.0; IEMobile/11.0; NOKIA; Lumia 635) like iPhone OS 7_0_3 Mac OS X AppleWebKit/537 (KHTML, like Gecko) Mobile Safari/537',
      undefined
    );
    const r = testO.wrap();

    return r;
  },
  {name: 'Windows Phone', version: '8.1', typ: 'mobile', arch: 'x64'}
);
osTest.new(
  'Linux',
  () => {
    let testO = ominiOs;
    testO = testO.setEnv(
      'Mozilla/5.0 (X11; Ubuntu; Linux i686; rv:24.0) Gecko/20100101 Firefox/24.0',
      undefined
    );
    const r = testO.wrap();

    return r;
  },
  {name: 'Linux', version: 'i686', typ: 'desktop', arch: 'x64'}
);
export default osTest;
