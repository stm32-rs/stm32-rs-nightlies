#[doc = "Register `POWER` reader"]
pub type R = crate::R<POWERrs>;
#[doc = "Register `POWER` writer"]
pub type W = crate::W<POWERrs>;
#[doc = "PWRCTRL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWRCTRL {
    #[doc = "0: Power off"]
    PowerOff = 0,
    #[doc = "3: Power on"]
    PowerOn = 3,
}
impl From<PWRCTRL> for u8 {
    #[inline(always)]
    fn from(variant: PWRCTRL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWRCTRL {
    type Ux = u8;
}
#[doc = "Field `PWRCTRL` reader - PWRCTRL"]
pub type PWRCTRL_R = crate::FieldReader<PWRCTRL>;
impl PWRCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWRCTRL> {
        match self.bits {
            0 => Some(PWRCTRL::PowerOff),
            3 => Some(PWRCTRL::PowerOn),
            _ => None,
        }
    }
    #[doc = "Power off"]
    #[inline(always)]
    pub fn is_power_off(&self) -> bool {
        *self == PWRCTRL::PowerOff
    }
    #[doc = "Power on"]
    #[inline(always)]
    pub fn is_power_on(&self) -> bool {
        *self == PWRCTRL::PowerOn
    }
}
#[doc = "Field `PWRCTRL` writer - PWRCTRL"]
pub type PWRCTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PWRCTRL>;
impl<'a, REG> PWRCTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Power off"]
    #[inline(always)]
    pub fn power_off(self) -> &'a mut crate::W<REG> {
        self.variant(PWRCTRL::PowerOff)
    }
    #[doc = "Power on"]
    #[inline(always)]
    pub fn power_on(self) -> &'a mut crate::W<REG> {
        self.variant(PWRCTRL::PowerOn)
    }
}
impl R {
    #[doc = "Bits 0:1 - PWRCTRL"]
    #[inline(always)]
    pub fn pwrctrl(&self) -> PWRCTRL_R {
        PWRCTRL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PWRCTRL"]
    #[inline(always)]
    #[must_use]
    pub fn pwrctrl(&mut self) -> PWRCTRL_W<POWERrs> {
        PWRCTRL_W::new(self, 0)
    }
}
#[doc = "power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWERrs;
impl crate::RegisterSpec for POWERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power::R`](R) reader structure"]
impl crate::Readable for POWERrs {}
#[doc = "`write(|w| ..)` method takes [`power::W`](W) writer structure"]
impl crate::Writable for POWERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POWER to value 0"]
impl crate::Resettable for POWERrs {
    const RESET_VALUE: u32 = 0;
}
