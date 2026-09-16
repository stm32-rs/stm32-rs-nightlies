///Register `CALR` reader
pub type R = crate::R<CALRrs>;
///Register `CALR` writer
pub type W = crate::W<CALRrs>;
///Field `CALM` reader - Calibration minus The frequency of the calendar is reduced by masking CALM out of 2<sup>20</sup> RTCCLK pulses (321seconds if the input frequency is 327681Hz). This decreases the frequency of the calendar with a resolution of 0.95371ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See Section128.3.14: RTC smooth digital calibration on page1733.
pub type CALM_R = crate::FieldReader<u16>;
///Field `CALM` writer - Calibration minus The frequency of the calendar is reduced by masking CALM out of 2<sup>20</sup> RTCCLK pulses (321seconds if the input frequency is 327681Hz). This decreases the frequency of the calendar with a resolution of 0.95371ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See Section128.3.14: RTC smooth digital calibration on page1733.
pub type CALM_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16, crate::Safe>;
///Field `LPCAL` reader - RTC low-power mode
pub type LPCAL_R = crate::BitReader;
///Field `LPCAL` writer - RTC low-power mode
pub type LPCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\[0\] is stuck at 0 when CALW16 = 1. Refer to Section128.3.14: RTC smooth digital calibration.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALW16 {
    ///1: When CALW16 is set to ‘1’, the 16-second calibration cycle period is selected.This bit must not be set to ‘1’ if CALW8=1
    SixteenSeconds = 1,
}
impl From<CALW16> for bool {
    #[inline(always)]
    fn from(variant: CALW16) -> Self {
        variant as u8 != 0
    }
}
///Field `CALW16` reader - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\[0\] is stuck at 0 when CALW16 = 1. Refer to Section128.3.14: RTC smooth digital calibration.
pub type CALW16_R = crate::BitReader<CALW16>;
impl CALW16_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CALW16> {
        match self.bits {
            true => Some(CALW16::SixteenSeconds),
            _ => None,
        }
    }
    ///When CALW16 is set to ‘1’, the 16-second calibration cycle period is selected.This bit must not be set to ‘1’ if CALW8=1
    #[inline(always)]
    pub fn is_sixteen_seconds(&self) -> bool {
        *self == CALW16::SixteenSeconds
    }
}
///Field `CALW16` writer - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\[0\] is stuck at 0 when CALW16 = 1. Refer to Section128.3.14: RTC smooth digital calibration.
pub type CALW16_W<'a, REG> = crate::BitWriter<'a, REG, CALW16>;
impl<'a, REG> CALW16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When CALW16 is set to ‘1’, the 16-second calibration cycle period is selected.This bit must not be set to ‘1’ if CALW8=1
    #[inline(always)]
    pub fn sixteen_seconds(self) -> &'a mut crate::W<REG> {
        self.variant(CALW16::SixteenSeconds)
    }
}
/**Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\[1:0\] are stuck at 00 when CALW8 = 1. Refer to Section128.3.14: RTC smooth digital calibration.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALW8 {
    ///1: When CALW8 is set to ‘1’, the 8-second calibration cycle period is selected
    EightSeconds = 1,
}
impl From<CALW8> for bool {
    #[inline(always)]
    fn from(variant: CALW8) -> Self {
        variant as u8 != 0
    }
}
///Field `CALW8` reader - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\[1:0\] are stuck at 00 when CALW8 = 1. Refer to Section128.3.14: RTC smooth digital calibration.
pub type CALW8_R = crate::BitReader<CALW8>;
impl CALW8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CALW8> {
        match self.bits {
            true => Some(CALW8::EightSeconds),
            _ => None,
        }
    }
    ///When CALW8 is set to ‘1’, the 8-second calibration cycle period is selected
    #[inline(always)]
    pub fn is_eight_seconds(&self) -> bool {
        *self == CALW8::EightSeconds
    }
}
///Field `CALW8` writer - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\[1:0\] are stuck at 00 when CALW8 = 1. Refer to Section128.3.14: RTC smooth digital calibration.
pub type CALW8_W<'a, REG> = crate::BitWriter<'a, REG, CALW8>;
impl<'a, REG> CALW8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When CALW8 is set to ‘1’, the 8-second calibration cycle period is selected
    #[inline(always)]
    pub fn eight_seconds(self) -> &'a mut crate::W<REG> {
        self.variant(CALW8::EightSeconds)
    }
}
/**Increase frequency of RTC by 488.51ppm.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALP {
    ///0: No RTCCLK pulses are added
    NoChange = 0,
    ///1: One RTCCLK pulse is effectively inserted every 2^11 pulses (frequency increased by 488.5 ppm)
    IncreaseFreq = 1,
}
impl From<CALP> for bool {
    #[inline(always)]
    fn from(variant: CALP) -> Self {
        variant as u8 != 0
    }
}
///Field `CALP` reader - Increase frequency of RTC by 488.51ppm.
pub type CALP_R = crate::BitReader<CALP>;
impl CALP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CALP {
        match self.bits {
            false => CALP::NoChange,
            true => CALP::IncreaseFreq,
        }
    }
    ///No RTCCLK pulses are added
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == CALP::NoChange
    }
    ///One RTCCLK pulse is effectively inserted every 2^11 pulses (frequency increased by 488.5 ppm)
    #[inline(always)]
    pub fn is_increase_freq(&self) -> bool {
        *self == CALP::IncreaseFreq
    }
}
///Field `CALP` writer - Increase frequency of RTC by 488.51ppm.
pub type CALP_W<'a, REG> = crate::BitWriter<'a, REG, CALP>;
impl<'a, REG> CALP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No RTCCLK pulses are added
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(CALP::NoChange)
    }
    ///One RTCCLK pulse is effectively inserted every 2^11 pulses (frequency increased by 488.5 ppm)
    #[inline(always)]
    pub fn increase_freq(self) -> &'a mut crate::W<REG> {
        self.variant(CALP::IncreaseFreq)
    }
}
impl R {
    ///Bits 0:8 - Calibration minus The frequency of the calendar is reduced by masking CALM out of 2<sup>20</sup> RTCCLK pulses (321seconds if the input frequency is 327681Hz). This decreases the frequency of the calendar with a resolution of 0.95371ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See Section128.3.14: RTC smooth digital calibration on page1733.
    #[inline(always)]
    pub fn calm(&self) -> CALM_R {
        CALM_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bit 12 - RTC low-power mode
    #[inline(always)]
    pub fn lpcal(&self) -> LPCAL_R {
        LPCAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\[0\] is stuck at 0 when CALW16 = 1. Refer to Section128.3.14: RTC smooth digital calibration.
    #[inline(always)]
    pub fn calw16(&self) -> CALW16_R {
        CALW16_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\[1:0\] are stuck at 00 when CALW8 = 1. Refer to Section128.3.14: RTC smooth digital calibration.
    #[inline(always)]
    pub fn calw8(&self) -> CALW8_R {
        CALW8_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Increase frequency of RTC by 488.51ppm.
    #[inline(always)]
    pub fn calp(&self) -> CALP_R {
        CALP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALR")
            .field("calm", &self.calm())
            .field("lpcal", &self.lpcal())
            .field("calw16", &self.calw16())
            .field("calw8", &self.calw8())
            .field("calp", &self.calp())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Calibration minus The frequency of the calendar is reduced by masking CALM out of 2<sup>20</sup> RTCCLK pulses (321seconds if the input frequency is 327681Hz). This decreases the frequency of the calendar with a resolution of 0.95371ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See Section128.3.14: RTC smooth digital calibration on page1733.
    #[inline(always)]
    pub fn calm(&mut self) -> CALM_W<'_, CALRrs> {
        CALM_W::new(self, 0)
    }
    ///Bit 12 - RTC low-power mode
    #[inline(always)]
    pub fn lpcal(&mut self) -> LPCAL_W<'_, CALRrs> {
        LPCAL_W::new(self, 12)
    }
    ///Bit 13 - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\[0\] is stuck at 0 when CALW16 = 1. Refer to Section128.3.14: RTC smooth digital calibration.
    #[inline(always)]
    pub fn calw16(&mut self) -> CALW16_W<'_, CALRrs> {
        CALW16_W::new(self, 13)
    }
    ///Bit 14 - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\[1:0\] are stuck at 00 when CALW8 = 1. Refer to Section128.3.14: RTC smooth digital calibration.
    #[inline(always)]
    pub fn calw8(&mut self) -> CALW8_W<'_, CALRrs> {
        CALW8_W::new(self, 14)
    }
    ///Bit 15 - Increase frequency of RTC by 488.51ppm.
    #[inline(always)]
    pub fn calp(&mut self) -> CALP_W<'_, CALRrs> {
        CALP_W::new(self, 15)
    }
}
/**RTC calibration register

You can [`read`](crate::Reg::read) this register and get [`calr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#RTC:CALR)*/
pub struct CALRrs;
impl crate::RegisterSpec for CALRrs {
    type Ux = u32;
}
///`read()` method returns [`calr::R`](R) reader structure
impl crate::Readable for CALRrs {}
///`write(|w| ..)` method takes [`calr::W`](W) writer structure
impl crate::Writable for CALRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CALR to value 0
impl crate::Resettable for CALRrs {}
