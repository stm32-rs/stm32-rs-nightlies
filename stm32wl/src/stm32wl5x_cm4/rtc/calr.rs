///Register `CALR` reader
pub type R = crate::R<CALRrs>;
///Register `CALR` writer
pub type W = crate::W<CALRrs>;
///Field `CALM` reader - Calibration minus
pub type CALM_R = crate::FieldReader<u16>;
///Field `CALM` writer - Calibration minus
pub type CALM_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16, crate::Safe>;
/**Calibration low-power mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPCAL {
    ///0: Calibration window is 220 RTCCLK, which is a high-consumption mode. This mode should be set only when less than 32s calibration window is required
    Rtcclk = 0,
    ///1: Calibration window is 220 ck_apre, which is the required configuration for ultra-low consumption mode
    CkApre = 1,
}
impl From<LPCAL> for bool {
    #[inline(always)]
    fn from(variant: LPCAL) -> Self {
        variant as u8 != 0
    }
}
///Field `LPCAL` reader - Calibration low-power mode
pub type LPCAL_R = crate::BitReader<LPCAL>;
impl LPCAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPCAL {
        match self.bits {
            false => LPCAL::Rtcclk,
            true => LPCAL::CkApre,
        }
    }
    ///Calibration window is 220 RTCCLK, which is a high-consumption mode. This mode should be set only when less than 32s calibration window is required
    #[inline(always)]
    pub fn is_rtcclk(&self) -> bool {
        *self == LPCAL::Rtcclk
    }
    ///Calibration window is 220 ck_apre, which is the required configuration for ultra-low consumption mode
    #[inline(always)]
    pub fn is_ck_apre(&self) -> bool {
        *self == LPCAL::CkApre
    }
}
///Field `LPCAL` writer - Calibration low-power mode
pub type LPCAL_W<'a, REG> = crate::BitWriter<'a, REG, LPCAL>;
impl<'a, REG> LPCAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Calibration window is 220 RTCCLK, which is a high-consumption mode. This mode should be set only when less than 32s calibration window is required
    #[inline(always)]
    pub fn rtcclk(self) -> &'a mut crate::W<REG> {
        self.variant(LPCAL::Rtcclk)
    }
    ///Calibration window is 220 ck_apre, which is the required configuration for ultra-low consumption mode
    #[inline(always)]
    pub fn ck_apre(self) -> &'a mut crate::W<REG> {
        self.variant(LPCAL::CkApre)
    }
}
/**CALW16

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
///Field `CALW16` reader - CALW16
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
///Field `CALW16` writer - CALW16
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
/**Use a 16-second calibration cycle period

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
///Field `CALW8` reader - Use a 16-second calibration cycle period
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
///Field `CALW8` writer - Use a 16-second calibration cycle period
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
/**Use an 8-second calibration cycle period

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
///Field `CALP` reader - Use an 8-second calibration cycle period
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
///Field `CALP` writer - Use an 8-second calibration cycle period
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
    ///Bits 0:8 - Calibration minus
    #[inline(always)]
    pub fn calm(&self) -> CALM_R {
        CALM_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bit 12 - Calibration low-power mode
    #[inline(always)]
    pub fn lpcal(&self) -> LPCAL_R {
        LPCAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CALW16
    #[inline(always)]
    pub fn calw16(&self) -> CALW16_R {
        CALW16_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Use a 16-second calibration cycle period
    #[inline(always)]
    pub fn calw8(&self) -> CALW8_R {
        CALW8_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Use an 8-second calibration cycle period
    #[inline(always)]
    pub fn calp(&self) -> CALP_R {
        CALP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALR")
            .field("calp", &self.calp())
            .field("calw8", &self.calw8())
            .field("calw16", &self.calw16())
            .field("lpcal", &self.lpcal())
            .field("calm", &self.calm())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Calibration minus
    #[inline(always)]
    pub fn calm(&mut self) -> CALM_W<'_, CALRrs> {
        CALM_W::new(self, 0)
    }
    ///Bit 12 - Calibration low-power mode
    #[inline(always)]
    pub fn lpcal(&mut self) -> LPCAL_W<'_, CALRrs> {
        LPCAL_W::new(self, 12)
    }
    ///Bit 13 - CALW16
    #[inline(always)]
    pub fn calw16(&mut self) -> CALW16_W<'_, CALRrs> {
        CALW16_W::new(self, 13)
    }
    ///Bit 14 - Use a 16-second calibration cycle period
    #[inline(always)]
    pub fn calw8(&mut self) -> CALW8_W<'_, CALRrs> {
        CALW8_W::new(self, 14)
    }
    ///Bit 15 - Use an 8-second calibration cycle period
    #[inline(always)]
    pub fn calp(&mut self) -> CALP_W<'_, CALRrs> {
        CALP_W::new(self, 15)
    }
}
/**Calibration register

You can [`read`](crate::Reg::read) this register and get [`calr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#RTC:CALR)*/
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
