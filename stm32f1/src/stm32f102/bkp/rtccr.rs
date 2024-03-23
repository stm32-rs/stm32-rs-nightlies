#[doc = "Register `RTCCR` reader"]
pub type R = crate::R<RTCCRrs>;
#[doc = "Register `RTCCR` writer"]
pub type W = crate::W<RTCCRrs>;
#[doc = "Field `CAL` reader - Calibration value"]
pub type CAL_R = crate::FieldReader;
#[doc = "Field `CAL` writer - Calibration value"]
pub type CAL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CCO` reader - Calibration Clock Output"]
pub type CCO_R = crate::BitReader;
#[doc = "Field `CCO` writer - Calibration Clock Output"]
pub type CCO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Alarm or second output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASOE {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Setting this bit outputs either the RTC Alarm pulse signal or the Second pulse signal on the TAMPER pin depending on the ASOS bit"]
    Enabled = 1,
}
impl From<ASOE> for bool {
    #[inline(always)]
    fn from(variant: ASOE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASOE` reader - Alarm or second output enable"]
pub type ASOE_R = crate::BitReader<ASOE>;
impl ASOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASOE {
        match self.bits {
            false => ASOE::Disabled,
            true => ASOE::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ASOE::Disabled
    }
    #[doc = "Setting this bit outputs either the RTC Alarm pulse signal or the Second pulse signal on the TAMPER pin depending on the ASOS bit"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ASOE::Enabled
    }
}
#[doc = "Field `ASOE` writer - Alarm or second output enable"]
pub type ASOE_W<'a, REG> = crate::BitWriter<'a, REG, ASOE>;
impl<'a, REG> ASOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ASOE::Disabled)
    }
    #[doc = "Setting this bit outputs either the RTC Alarm pulse signal or the Second pulse signal on the TAMPER pin depending on the ASOS bit"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ASOE::Enabled)
    }
}
#[doc = "Alarm or second output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASOS {
    #[doc = "0: RTC Alarm pulse output selected"]
    Alarm = 0,
    #[doc = "1: RTC Second pulse output selected"]
    Second = 1,
}
impl From<ASOS> for bool {
    #[inline(always)]
    fn from(variant: ASOS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASOS` reader - Alarm or second output selection"]
pub type ASOS_R = crate::BitReader<ASOS>;
impl ASOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASOS {
        match self.bits {
            false => ASOS::Alarm,
            true => ASOS::Second,
        }
    }
    #[doc = "RTC Alarm pulse output selected"]
    #[inline(always)]
    pub fn is_alarm(&self) -> bool {
        *self == ASOS::Alarm
    }
    #[doc = "RTC Second pulse output selected"]
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        *self == ASOS::Second
    }
}
#[doc = "Field `ASOS` writer - Alarm or second output selection"]
pub type ASOS_W<'a, REG> = crate::BitWriter<'a, REG, ASOS>;
impl<'a, REG> ASOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC Alarm pulse output selected"]
    #[inline(always)]
    pub fn alarm(self) -> &'a mut crate::W<REG> {
        self.variant(ASOS::Alarm)
    }
    #[doc = "RTC Second pulse output selected"]
    #[inline(always)]
    pub fn second(self) -> &'a mut crate::W<REG> {
        self.variant(ASOS::Second)
    }
}
impl R {
    #[doc = "Bits 0:6 - Calibration value"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Calibration Clock Output"]
    #[inline(always)]
    pub fn cco(&self) -> CCO_R {
        CCO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm or second output enable"]
    #[inline(always)]
    pub fn asoe(&self) -> ASOE_R {
        ASOE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm or second output selection"]
    #[inline(always)]
    pub fn asos(&self) -> ASOS_R {
        ASOS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration value"]
    #[inline(always)]
    #[must_use]
    pub fn cal(&mut self) -> CAL_W<RTCCRrs> {
        CAL_W::new(self, 0)
    }
    #[doc = "Bit 7 - Calibration Clock Output"]
    #[inline(always)]
    #[must_use]
    pub fn cco(&mut self) -> CCO_W<RTCCRrs> {
        CCO_W::new(self, 7)
    }
    #[doc = "Bit 8 - Alarm or second output enable"]
    #[inline(always)]
    #[must_use]
    pub fn asoe(&mut self) -> ASOE_W<RTCCRrs> {
        ASOE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Alarm or second output selection"]
    #[inline(always)]
    #[must_use]
    pub fn asos(&mut self) -> ASOS_W<RTCCRrs> {
        ASOS_W::new(self, 9)
    }
}
#[doc = "RTC clock calibration register (BKP_RTCCR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCCRrs;
impl crate::RegisterSpec for RTCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtccr::R`](R) reader structure"]
impl crate::Readable for RTCCRrs {}
#[doc = "`write(|w| ..)` method takes [`rtccr::W`](W) writer structure"]
impl crate::Writable for RTCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTCCR to value 0"]
impl crate::Resettable for RTCCRrs {
    const RESET_VALUE: u32 = 0;
}
