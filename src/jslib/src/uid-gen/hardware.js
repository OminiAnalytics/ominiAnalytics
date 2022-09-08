/**
 * File: hardware.js
 * Created Date: 02 Sep 2022
 * Author: realbacon
 * -----
 * Last Modified: 8/09/2022 08:48:35
 * Modified By: realbacon
 * -----
 * @license MIT
 * -----
 **/

export const ominiHardware = {
  /**
   * Get the memory size of the device.
   * @returns {number} Memory size in Go.
   * @example
   * const memory = ominiHardware.getMemorySize();
   * console.log(memory);
   * // => 8
   * */
  getMemory: function () {
    return navigator.deviceMemory || 0
  },
  /**
   * Get the CPU cores of the device.
   * @returns {number} CPU cores.
   * @example
   * const cpu = ominiHardware.getCpuCores();
   * console.log(cpu);
   * // => 4
   * */
  getCPU: function () {
    return navigator.hardwareConcurrency || 0
  },
  /**
   * Get the GPU information of the device.
   * @returns {string} GPU information.
   * @example
   * const gpu = ominiHardware.getGpuInfo();
   * console.log(gpu);
   * // => "ANGLE (Google, Vulkan 1.3.0 (SwiftShader Device (Subzero) (0x0000C0DE)), SwiftShader driver)"
   **/
  getGPU: function () {
    const gl = document.createElement('canvas').getContext('webgl')
    if (!gl) {
      return 'unknown'
    }
    const debugInfo = gl.getExtension('WEBGL_debug_renderer_info')
    return debugInfo
      ? gl.getParameter(debugInfo.UNMASKED_RENDERER_WEBGL)
      : 'unknown'
  },
  /**
   * Get the color buffer of gpu
   * @returns {Number} Color buffer in bits.
   */
  getColorBufferFloat: function () {
    const gl = document.createElement('canvas').getContext('webgl')
    if (!gl) {
      return 0
    }
    const ext = gl.getExtension('WEBGL_color_buffer_float')
    return ext ? ext.RGBA32F_EXT : 0
  },
  /**
   * Get screen size of device and color depth
   * @returns {[number, number, number]} [width, height, color depth]
   * @example
   * console.log(ominiHardware.getScreenSize());
   * // [1920, 1080,24]
   **/
  getScreen: function () {
    return [screen.width || 0, screen.height || 0, screen.colorDepth || 0]
  },
  /**
   * Wrap the hardware information
   * @returns Hardware information
   * @example
   * console.log(ominiHardware.wrap());
   * // {
   * //   memory: 8,
   * //   cpu: 4,
   * //   gpu: "ANGLE (Google, Vulkan 1.3.0 (SwiftShader Device (Subzero) (0x0000C0DE)), SwiftShader driver)",
   * //   colorBufferFloat: 0,
   * //   screen: [1920, 1080, 24],
   * // }
   */
  wrap: function () {
    return {
      memory: this.getMemory(),
      cpu: this.getCPU(),
      gpu: this.getGPU(),
      colorBufferFloat: this.getColorBufferFloat(),
      screen: this.getScreen()
    }
  }
}
