///Register `RTCCR` reader
pub type R = crate::R<RTCCRrs>;
///Register `RTCCR` writer
pub type W = crate::W<RTCCRrs>;
///Field `CAL` reader - Calibration value
pub type CAL_R = crate::FieldReader;
///Field `CAL` writer - Calibration value
pub type CAL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `CCO` reader - Calibration Clock Output
pub type CCO_R = crate::BitReader;
///Field `CCO` writer - Calibration Clock Output
pub type CCO_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Alarm or second output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASOE {
    ///0: Disabled
    Disabled = 0,
    ///1: Setting this bit outputs either the RTC Alarm pulse signal or the Second pulse signal on the TAMPER pin depending on the ASOS bit
    Enabled = 1,
}
impl From<ASOE> for bool {
    #[inline(always)]
    fn from(variant: ASOE) -> Self {
        variant as u8 != 0
    }
}
///Field `ASOE` reader - Alarm or second output enable
pub type ASOE_R = crate::BitReader<ASOE>;
impl ASOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASOE {
        match self.bits {
            false => ASOE::Disabled,
            true => ASOE::Enabled,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ASOE::Disabled
    }
    ///Setting this bit outputs either the RTC Alarm pulse signal or the Second pulse signal on the TAMPER pin depending on the ASOS bit
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ASOE::Enabled
    }
}
///Field `ASOE` writer - Alarm or second output enable
pub type ASOE_W<'a, REG> = crate::BitWriter<'a, REG, ASOE>;
impl<'a, REG> ASOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ASOE::Disabled)
    }
    ///Setting this bit outputs either the RTC Alarm pulse signal or the Second pulse signal on the TAMPER pin depending on the ASOS bit
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ASOE::Enabled)
    }
}
/**Alarm or second output selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASOS {
    ///0: RTC Alarm pulse output selected
    Alarm = 0,
    ///1: RTC Second pulse output selected
    Second = 1,
}
impl From<ASOS> for bool {
    #[inline(always)]
    fn from(variant: ASOS) -> Self {
        variant as u8 != 0
    }
}
///Field `ASOS` reader - Alarm or second output selection
pub type ASOS_R = crate::BitReader<ASOS>;
impl ASOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASOS {
        match self.bits {
            false => ASOS::Alarm,
            true => ASOS::Second,
        }
    }
    ///RTC Alarm pulse output selected
    #[inline(always)]
    pub fn is_alarm(&self) -> bool {
        *self == ASOS::Alarm
    }
    ///RTC Second pulse output selected
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        *self == ASOS::Second
    }
}
///Field `ASOS` writer - Alarm or second output selection
pub type ASOS_W<'a, REG> = crate::BitWriter<'a, REG, ASOS>;
impl<'a, REG> ASOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RTC Alarm pulse output selected
    #[inline(always)]
    pub fn alarm(self) -> &'a mut crate::W<REG> {
        self.variant(ASOS::Alarm)
    }
    ///RTC Second pulse output selected
    #[inline(always)]
    pub fn second(self) -> &'a mut crate::W<REG> {
        self.variant(ASOS::Second)
    }
}
impl R {
    ///Bits 0:6 - Calibration value
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 7 - Calibration Clock Output
    #[inline(always)]
    pub fn cco(&self) -> CCO_R {
        CCO_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Alarm or second output enable
    #[inline(always)]
    pub fn asoe(&self) -> ASOE_R {
        ASOE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Alarm or second output selection
    #[inline(always)]
    pub fn asos(&self) -> ASOS_R {
        ASOS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTCCR")
            .field("cal", &self.cal())
            .field("cco", &self.cco())
            .field("asoe", &self.asoe())
            .field("asos", &self.asos())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Calibration value
    #[inline(always)]
    pub fn cal(&mut self) -> CAL_W<'_, RTCCRrs> {
        CAL_W::new(self, 0)
    }
    ///Bit 7 - Calibration Clock Output
    #[inline(always)]
    pub fn cco(&mut self) -> CCO_W<'_, RTCCRrs> {
        CCO_W::new(self, 7)
    }
    ///Bit 8 - Alarm or second output enable
    #[inline(always)]
    pub fn asoe(&mut self) -> ASOE_W<'_, RTCCRrs> {
        ASOE_W::new(self, 8)
    }
    ///Bit 9 - Alarm or second output selection
    #[inline(always)]
    pub fn asos(&mut self) -> ASOS_W<'_, RTCCRrs> {
        ASOS_W::new(self, 9)
    }
}
/**RTC clock calibration register (BKP_RTCCR)

You can [`read`](crate::Reg::read) this register and get [`rtccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#BKP:RTCCR)*/
pub struct RTCCRrs;
impl crate::RegisterSpec for RTCCRrs {
    type Ux = u32;
}
///`read()` method returns [`rtccr::R`](R) reader structure
impl crate::Readable for RTCCRrs {}
///`write(|w| ..)` method takes [`rtccr::W`](W) writer structure
impl crate::Writable for RTCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RTCCR to value 0
impl crate::Resettable for RTCCRrs {}
