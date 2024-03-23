#[doc = "Register `RTSR2` reader"]
pub type R = crate::R<RTSR2rs>;
#[doc = "Register `RTSR2` writer"]
pub type W = crate::W<RTSR2rs>;
#[doc = "Rising trigger event configuration bit of configurable line 34\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT2 {
    #[doc = "0: Rising edge trigger is disabled"]
    Disabled = 0,
    #[doc = "1: Rising edge trigger is enabled"]
    Enabled = 1,
}
impl From<RT2> for bool {
    #[inline(always)]
    fn from(variant: RT2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT2` reader - Rising trigger event configuration bit of configurable line 34"]
pub type RT2_R = crate::BitReader<RT2>;
impl RT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT2 {
        match self.bits {
            false => RT2::Disabled,
            true => RT2::Enabled,
        }
    }
    #[doc = "Rising edge trigger is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RT2::Disabled
    }
    #[doc = "Rising edge trigger is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RT2::Enabled
    }
}
#[doc = "Field `RT2` writer - Rising trigger event configuration bit of configurable line 34"]
pub type RT2_W<'a, REG> = crate::BitWriter<'a, REG, RT2>;
impl<'a, REG> RT2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RT2::Disabled)
    }
    #[doc = "Rising edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RT2::Enabled)
    }
}
impl R {
    #[doc = "Bit 2 - Rising trigger event configuration bit of configurable line 34"]
    #[inline(always)]
    pub fn rt2(&self) -> RT2_R {
        RT2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Rising trigger event configuration bit of configurable line 34"]
    #[inline(always)]
    #[must_use]
    pub fn rt2(&mut self) -> RT2_W<RTSR2rs> {
        RT2_W::new(self, 2)
    }
}
#[doc = "EXTI rising trigger selection register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtsr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtsr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTSR2rs;
impl crate::RegisterSpec for RTSR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtsr2::R`](R) reader structure"]
impl crate::Readable for RTSR2rs {}
#[doc = "`write(|w| ..)` method takes [`rtsr2::W`](W) writer structure"]
impl crate::Writable for RTSR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTSR2 to value 0"]
impl crate::Resettable for RTSR2rs {
    const RESET_VALUE: u32 = 0;
}
