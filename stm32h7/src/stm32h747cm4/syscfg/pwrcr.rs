#[doc = "Register `PWRCR` reader"]
pub type R = crate::R<PWRCRrs>;
#[doc = "Register `PWRCR` writer"]
pub type W = crate::W<PWRCRrs>;
#[doc = "Overdrive enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODEN {
    #[doc = "0: Overdrive mode disabled"]
    Disabled = 0,
    #[doc = "1: Overdrive mode enabled (the LDO generates VOS0 for VCORE)"]
    Enabled = 1,
}
impl From<ODEN> for bool {
    #[inline(always)]
    fn from(variant: ODEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODEN` reader - Overdrive enable"]
pub type ODEN_R = crate::BitReader<ODEN>;
impl ODEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ODEN {
        match self.bits {
            false => ODEN::Disabled,
            true => ODEN::Enabled,
        }
    }
    #[doc = "Overdrive mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ODEN::Disabled
    }
    #[doc = "Overdrive mode enabled (the LDO generates VOS0 for VCORE)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ODEN::Enabled
    }
}
#[doc = "Field `ODEN` writer - Overdrive enable"]
pub type ODEN_W<'a, REG> = crate::BitWriter<'a, REG, ODEN>;
impl<'a, REG> ODEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overdrive mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ODEN::Disabled)
    }
    #[doc = "Overdrive mode enabled (the LDO generates VOS0 for VCORE)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ODEN::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Overdrive enable"]
    #[inline(always)]
    pub fn oden(&self) -> ODEN_R {
        ODEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overdrive enable"]
    #[inline(always)]
    #[must_use]
    pub fn oden(&mut self) -> ODEN_W<PWRCRrs> {
        ODEN_W::new(self, 0)
    }
}
#[doc = "SYSCFG power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWRCRrs;
impl crate::RegisterSpec for PWRCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrcr::R`](R) reader structure"]
impl crate::Readable for PWRCRrs {}
#[doc = "`write(|w| ..)` method takes [`pwrcr::W`](W) writer structure"]
impl crate::Writable for PWRCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRCR to value 0"]
impl crate::Resettable for PWRCRrs {
    const RESET_VALUE: u32 = 0;
}
