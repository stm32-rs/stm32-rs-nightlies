#[doc = "Register `CALR` reader"]
pub type R = crate::R<CALRrs>;
#[doc = "Register `CALR` writer"]
pub type W = crate::W<CALRrs>;
#[doc = "Field `CALM` reader - Calibration minus The frequency of the calendar is reduced by masking CALM out of 2&lt;sup>20&lt;/sup> RTCCLK pulses (32 seconds if the input frequency is 32768 Hz). This decreases the frequency of the calendar with a resolution of 0.9537 ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See Section 31.3.15: RTC smooth digital calibration on page 1092."]
pub type CALM_R = crate::FieldReader<u16>;
#[doc = "Field `CALM` writer - Calibration minus The frequency of the calendar is reduced by masking CALM out of 2&lt;sup>20&lt;/sup> RTCCLK pulses (32 seconds if the input frequency is 32768 Hz). This decreases the frequency of the calendar with a resolution of 0.9537 ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See Section 31.3.15: RTC smooth digital calibration on page 1092."]
pub type CALM_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `LPCAL` reader - RTC low-power mode"]
pub type LPCAL_R = crate::BitReader;
#[doc = "Field `LPCAL` writer - RTC low-power mode"]
pub type LPCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALW16` reader - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\\[0\\]
is stuck at 0 when CALW16 = 1. Refer to Section 31.3.15: RTC smooth digital calibration."]
pub type CALW16_R = crate::BitReader;
#[doc = "Field `CALW16` writer - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\\[0\\]
is stuck at 0 when CALW16 = 1. Refer to Section 31.3.15: RTC smooth digital calibration."]
pub type CALW16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALW8` reader - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\]
are stuck at 00 when CALW8 = 1. Refer to Section 31.3.15: RTC smooth digital calibration."]
pub type CALW8_R = crate::BitReader;
#[doc = "Field `CALW8` writer - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\]
are stuck at 00 when CALW8 = 1. Refer to Section 31.3.15: RTC smooth digital calibration."]
pub type CALW8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALP` reader - Increase frequency of RTC by 488.5 ppm This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. if the input frequency is 32768 Hz, the number of RTCCLK pulses added during a 32-second window is calculated as follows: (512 × CALP) ‑ CALM. Refer to Section 31.3.15: RTC smooth digital calibration."]
pub type CALP_R = crate::BitReader;
#[doc = "Field `CALP` writer - Increase frequency of RTC by 488.5 ppm This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. if the input frequency is 32768 Hz, the number of RTCCLK pulses added during a 32-second window is calculated as follows: (512 × CALP) ‑ CALM. Refer to Section 31.3.15: RTC smooth digital calibration."]
pub type CALP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Calibration minus The frequency of the calendar is reduced by masking CALM out of 2&lt;sup>20&lt;/sup> RTCCLK pulses (32 seconds if the input frequency is 32768 Hz). This decreases the frequency of the calendar with a resolution of 0.9537 ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See Section 31.3.15: RTC smooth digital calibration on page 1092."]
    #[inline(always)]
    pub fn calm(&self) -> CALM_R {
        CALM_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 12 - RTC low-power mode"]
    #[inline(always)]
    pub fn lpcal(&self) -> LPCAL_R {
        LPCAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\\[0\\]
is stuck at 0 when CALW16 = 1. Refer to Section 31.3.15: RTC smooth digital calibration."]
    #[inline(always)]
    pub fn calw16(&self) -> CALW16_R {
        CALW16_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\]
are stuck at 00 when CALW8 = 1. Refer to Section 31.3.15: RTC smooth digital calibration."]
    #[inline(always)]
    pub fn calw8(&self) -> CALW8_R {
        CALW8_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Increase frequency of RTC by 488.5 ppm This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. if the input frequency is 32768 Hz, the number of RTCCLK pulses added during a 32-second window is calculated as follows: (512 × CALP) ‑ CALM. Refer to Section 31.3.15: RTC smooth digital calibration."]
    #[inline(always)]
    pub fn calp(&self) -> CALP_R {
        CALP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Calibration minus The frequency of the calendar is reduced by masking CALM out of 2&lt;sup>20&lt;/sup> RTCCLK pulses (32 seconds if the input frequency is 32768 Hz). This decreases the frequency of the calendar with a resolution of 0.9537 ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See Section 31.3.15: RTC smooth digital calibration on page 1092."]
    #[inline(always)]
    #[must_use]
    pub fn calm(&mut self) -> CALM_W<CALRrs> {
        CALM_W::new(self, 0)
    }
    #[doc = "Bit 12 - RTC low-power mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpcal(&mut self) -> LPCAL_W<CALRrs> {
        LPCAL_W::new(self, 12)
    }
    #[doc = "Bit 13 - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\\[0\\]
is stuck at 0 when CALW16 = 1. Refer to Section 31.3.15: RTC smooth digital calibration."]
    #[inline(always)]
    #[must_use]
    pub fn calw16(&mut self) -> CALW16_W<CALRrs> {
        CALW16_W::new(self, 13)
    }
    #[doc = "Bit 14 - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\]
are stuck at 00 when CALW8 = 1. Refer to Section 31.3.15: RTC smooth digital calibration."]
    #[inline(always)]
    #[must_use]
    pub fn calw8(&mut self) -> CALW8_W<CALRrs> {
        CALW8_W::new(self, 14)
    }
    #[doc = "Bit 15 - Increase frequency of RTC by 488.5 ppm This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. if the input frequency is 32768 Hz, the number of RTCCLK pulses added during a 32-second window is calculated as follows: (512 × CALP) ‑ CALM. Refer to Section 31.3.15: RTC smooth digital calibration."]
    #[inline(always)]
    #[must_use]
    pub fn calp(&mut self) -> CALP_W<CALRrs> {
        CALP_W::new(self, 15)
    }
}
#[doc = "RTC calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALRrs;
impl crate::RegisterSpec for CALRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calr::R`](R) reader structure"]
impl crate::Readable for CALRrs {}
#[doc = "`write(|w| ..)` method takes [`calr::W`](W) writer structure"]
impl crate::Writable for CALRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CALR to value 0"]
impl crate::Resettable for CALRrs {
    const RESET_VALUE: u32 = 0;
}
