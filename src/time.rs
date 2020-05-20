use crate::device::Device;

// TODO: put device into a low power mode instead
// https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-11057-32-bit-Cortex-M3-Microcontroller-SAM3X-SAM3A_Datasheet.pdf
// Wait mode has very rapid wakeup, system state preserved
// Sleep mode slower wakeup, system state preserved
// Backup mode (deep sleep - core and PIOs reset on wake-up)
// instead enter WFE (wait for event) mode for low power, get woken up by RTC, RTT, or USB
// 1. Select 4/8/12 MHz Fast RC Oscillator as the Main Clock
// 2. Set LPM (low power mode) bit = 1 in PMC_FSMR
// 3. Execute WFE instruction
//
// Real-time Timer Mode Register (RTT_MR, 0x400E1A30)
// TODO: Verify this table
// 0x0000: 2^16 * SCLK = 1Hz
// 0x8000 * SCLK =     2Hz
// 0x4000 * SCLK =     4Hz
// 0x2000 * SCLK =     8Hz
// 0x1000 * SCLK =    16Hz
// 0x0800 * SCLK =    32Hz
// 0x0400 * SCLK =    64Hz
// 0x0200 * SCLK =   128Hz
// 0x0100 * SCLK =   256Hz
// 0x0080 * SCLK =   512Hz
// 0x0040 * SCLK =  1024Hz (~1kHz/0.98 ms resolution)
// 0x0020 * SCLK =  2048Hz
// 0x0010 * SCLK =  4096Hz
// 0x0008 * SCLK =  8192Hz
// 0x0004 * SCLK = 16384Hz
// 0x0002 * SCLK = 32768Hz
// 0x0001 * SCLK = 65535Hz (65kHz/25us resolution)
//
// RTPRES = 0x0040 for ms < 1000 (accurate to 0.98ms)
// RTPRES = 0x0200 for 1000 < ms < 5000 (accurate to 7.8ms)
// RTPRES = 0x4000 for 5000 < ms < 20000 (accurate to 250ms)
// RTPRES = 0x0000 for 20000 < ms (accurate to 1s)
//
// Alarm Interrupt Enable (ALMIEN): 1 enabled
// Real-time Timer Increment Interrupt Enable (RTTINCIEN): 0 disabled
// Real-time Timer Restart (RTTRST): 1 restart, reset counter
//
// Real-time Timer Alarm Register (RTT_AR, 0x400E1A34)
// Alarm Value (ALMV), timer alarms when counter = ALMV + 1
//
// Real-time Timer Value Register (RTT_VR, 0x400E1A38)
// Current Real-time Value (CRTV), timer alarms when counter = ALMV + 1
//
// Real-time Timer Status Register (RTT_SR, 0x400E1A3C)
// Real-time Alarm Status (ALMS): 1 alarm has occurred since last read
//
// Real-time Timer Increment (RTTINC): 1 timer has incremented since last read
pub(crate) fn delay_ms(device: &Device, ms: u32) {
    let now = device.rtt_get_value();
    let next = now.checked_add(ms).expect("Correctly handle overflow");
    while device.rtt_get_value() < next {}
}
