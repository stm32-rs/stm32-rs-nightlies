#[doc = "Register `CALR` reader"]
pub type R = crate::R<CALRrs>;
#[doc = "Register `CALR` writer"]
pub type W = crate::W<CALRrs>;
#[doc = "Field `CALM` reader - Calibration minus The frequency of the calendar is reduced by masking CALM out of 220 RTCCLK pulses (32 seconds if the input frequency is 32768 Hz). This decreases the frequency of the calendar with a resolution of 0.9537 ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See Section24.3.12: RTC smooth digital calibration on page13."]
pub type CALM_R = crate::FieldReader<u16>;
#[doc = "Field `CALM` writer - Calibration minus The frequency of the calendar is reduced by masking CALM out of 220 RTCCLK pulses (32 seconds if the input frequency is 32768 Hz). This decreases the frequency of the calendar with a resolution of 0.9537 ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See Section24.3.12: RTC smooth digital calibration on page13."]
pub type CALM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 9, u16>;
#[doc = "Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected.This bit must not be set to 1 if CALW8=1. Note: CALM\\[0\\]
is stuck at 0 when CALW16= 1. Refer to Section24.3.12: RTC smooth digital calibration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALW16 {
    #[doc = "1: When CALW16 is set to ‘1’, the 16-second calibration cycle period is selected.This bit must not be set to ‘1’ if CALW8=1"]
    SixteenSecond = 1,
}
impl From<CALW16> for bool {
    #[inline(always)]
    fn from(variant: CALW16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALW16` reader - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected.This bit must not be set to 1 if CALW8=1. Note: CALM\\[0\\]
is stuck at 0 when CALW16= 1. Refer to Section24.3.12: RTC smooth digital calibration."]
pub type CALW16_R = crate::BitReader<CALW16>;
impl CALW16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CALW16> {
        match self.bits {
            true => Some(CALW16::SixteenSecond),
            _ => None,
        }
    }
    #[doc = "When CALW16 is set to ‘1’, the 16-second calibration cycle period is selected.This bit must not be set to ‘1’ if CALW8=1"]
    #[inline(always)]
    pub fn is_sixteen_second(&self) -> bool {
        *self == CALW16::SixteenSecond
    }
}
#[doc = "Field `CALW16` writer - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected.This bit must not be set to 1 if CALW8=1. Note: CALM\\[0\\]
is stuck at 0 when CALW16= 1. Refer to Section24.3.12: RTC smooth digital calibration."]
pub type CALW16_W<'a, REG> = crate::BitWriter<'a, REG, CALW16>;
impl<'a, REG> CALW16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When CALW16 is set to ‘1’, the 16-second calibration cycle period is selected.This bit must not be set to ‘1’ if CALW8=1"]
    #[inline(always)]
    pub fn sixteen_second(self) -> &'a mut crate::W<REG> {
        self.variant(CALW16::SixteenSecond)
    }
}
#[doc = "Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\]
are stuck at 00; when CALW8= 1. Refer to Section24.3.12: RTC smooth digital calibration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALW8 {
    #[doc = "1: When CALW8 is set to ‘1’, the 8-second calibration cycle period is selected"]
    EightSecond = 1,
}
impl From<CALW8> for bool {
    #[inline(always)]
    fn from(variant: CALW8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALW8` reader - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\]
are stuck at 00; when CALW8= 1. Refer to Section24.3.12: RTC smooth digital calibration."]
pub type CALW8_R = crate::BitReader<CALW8>;
impl CALW8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CALW8> {
        match self.bits {
            true => Some(CALW8::EightSecond),
            _ => None,
        }
    }
    #[doc = "When CALW8 is set to ‘1’, the 8-second calibration cycle period is selected"]
    #[inline(always)]
    pub fn is_eight_second(&self) -> bool {
        *self == CALW8::EightSecond
    }
}
#[doc = "Field `CALW8` writer - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\]
are stuck at 00; when CALW8= 1. Refer to Section24.3.12: RTC smooth digital calibration."]
pub type CALW8_W<'a, REG> = crate::BitWriter<'a, REG, CALW8>;
impl<'a, REG> CALW8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When CALW8 is set to ‘1’, the 8-second calibration cycle period is selected"]
    #[inline(always)]
    pub fn eight_second(self) -> &'a mut crate::W<REG> {
        self.variant(CALW8::EightSecond)
    }
}
#[doc = "Increase frequency of RTC by 488.5 ppm This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. if the input frequency is 32768 Hz, the number of RTCCLK pulses added during a 32-second window is calculated as follows: (512 * CALP) - CALM. Refer to Section24.3.12: RTC smooth digital calibration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALP {
    #[doc = "0: No RTCCLK pulses are added"]
    NoChange = 0,
    #[doc = "1: One RTCCLK pulse is effectively inserted every 2^11 pulses (frequency increased by 488.5 ppm)"]
    IncreaseFreq = 1,
}
impl From<CALP> for bool {
    #[inline(always)]
    fn from(variant: CALP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALP` reader - Increase frequency of RTC by 488.5 ppm This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. if the input frequency is 32768 Hz, the number of RTCCLK pulses added during a 32-second window is calculated as follows: (512 * CALP) - CALM. Refer to Section24.3.12: RTC smooth digital calibration."]
pub type CALP_R = crate::BitReader<CALP>;
impl CALP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CALP {
        match self.bits {
            false => CALP::NoChange,
            true => CALP::IncreaseFreq,
        }
    }
    #[doc = "No RTCCLK pulses are added"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == CALP::NoChange
    }
    #[doc = "One RTCCLK pulse is effectively inserted every 2^11 pulses (frequency increased by 488.5 ppm)"]
    #[inline(always)]
    pub fn is_increase_freq(&self) -> bool {
        *self == CALP::IncreaseFreq
    }
}
#[doc = "Field `CALP` writer - Increase frequency of RTC by 488.5 ppm This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. if the input frequency is 32768 Hz, the number of RTCCLK pulses added during a 32-second window is calculated as follows: (512 * CALP) - CALM. Refer to Section24.3.12: RTC smooth digital calibration."]
pub type CALP_W<'a, REG> = crate::BitWriter<'a, REG, CALP>;
impl<'a, REG> CALP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No RTCCLK pulses are added"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(CALP::NoChange)
    }
    #[doc = "One RTCCLK pulse is effectively inserted every 2^11 pulses (frequency increased by 488.5 ppm)"]
    #[inline(always)]
    pub fn increase_freq(self) -> &'a mut crate::W<REG> {
        self.variant(CALP::IncreaseFreq)
    }
}
impl R {
    #[doc = "Bits 0:8 - Calibration minus The frequency of the calendar is reduced by masking CALM out of 220 RTCCLK pulses (32 seconds if the input frequency is 32768 Hz). This decreases the frequency of the calendar with a resolution of 0.9537 ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See Section24.3.12: RTC smooth digital calibration on page13."]
    #[inline(always)]
    pub fn calm(&self) -> CALM_R {
        CALM_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 13 - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected.This bit must not be set to 1 if CALW8=1. Note: CALM\\[0\\]
is stuck at 0 when CALW16= 1. Refer to Section24.3.12: RTC smooth digital calibration."]
    #[inline(always)]
    pub fn calw16(&self) -> CALW16_R {
        CALW16_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\]
are stuck at 00; when CALW8= 1. Refer to Section24.3.12: RTC smooth digital calibration."]
    #[inline(always)]
    pub fn calw8(&self) -> CALW8_R {
        CALW8_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Increase frequency of RTC by 488.5 ppm This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. if the input frequency is 32768 Hz, the number of RTCCLK pulses added during a 32-second window is calculated as follows: (512 * CALP) - CALM. Refer to Section24.3.12: RTC smooth digital calibration."]
    #[inline(always)]
    pub fn calp(&self) -> CALP_R {
        CALP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Calibration minus The frequency of the calendar is reduced by masking CALM out of 220 RTCCLK pulses (32 seconds if the input frequency is 32768 Hz). This decreases the frequency of the calendar with a resolution of 0.9537 ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See Section24.3.12: RTC smooth digital calibration on page13."]
    #[inline(always)]
    #[must_use]
    pub fn calm(&mut self) -> CALM_W<CALRrs> {
        CALM_W::new(self, 0)
    }
    #[doc = "Bit 13 - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected.This bit must not be set to 1 if CALW8=1. Note: CALM\\[0\\]
is stuck at 0 when CALW16= 1. Refer to Section24.3.12: RTC smooth digital calibration."]
    #[inline(always)]
    #[must_use]
    pub fn calw16(&mut self) -> CALW16_W<CALRrs> {
        CALW16_W::new(self, 13)
    }
    #[doc = "Bit 14 - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\]
are stuck at 00; when CALW8= 1. Refer to Section24.3.12: RTC smooth digital calibration."]
    #[inline(always)]
    #[must_use]
    pub fn calw8(&mut self) -> CALW8_W<CALRrs> {
        CALW8_W::new(self, 14)
    }
    #[doc = "Bit 15 - Increase frequency of RTC by 488.5 ppm This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. if the input frequency is 32768 Hz, the number of RTCCLK pulses added during a 32-second window is calculated as follows: (512 * CALP) - CALM. Refer to Section24.3.12: RTC smooth digital calibration."]
    #[inline(always)]
    #[must_use]
    pub fn calp(&mut self) -> CALP_W<CALRrs> {
        CALP_W::new(self, 15)
    }
}
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
