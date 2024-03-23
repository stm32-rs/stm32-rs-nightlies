#[doc = "Register `RTSR2` reader"]
pub type R = crate::R<RTSR2rs>;
#[doc = "Register `RTSR2` writer"]
pub type W = crate::W<RTSR2rs>;
#[doc = "Rising trigger event configuration bit of Configurable Event input x+32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TR49 {
    #[doc = "0: Rising edge trigger is disabled"]
    Disabled = 0,
    #[doc = "1: Rising edge trigger is enabled"]
    Enabled = 1,
}
impl From<TR49> for bool {
    #[inline(always)]
    fn from(variant: TR49) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TR49` reader - Rising trigger event configuration bit of Configurable Event input x+32"]
pub type TR49_R = crate::BitReader<TR49>;
impl TR49_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TR49 {
        match self.bits {
            false => TR49::Disabled,
            true => TR49::Enabled,
        }
    }
    #[doc = "Rising edge trigger is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TR49::Disabled
    }
    #[doc = "Rising edge trigger is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TR49::Enabled
    }
}
#[doc = "Field `TR49` writer - Rising trigger event configuration bit of Configurable Event input x+32"]
pub type TR49_W<'a, REG> = crate::BitWriter<'a, REG, TR49>;
impl<'a, REG> TR49_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TR49::Disabled)
    }
    #[doc = "Rising edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TR49::Enabled)
    }
}
#[doc = "Field `TR51` reader - Rising trigger event configuration bit of Configurable Event input x+32"]
pub use TR49_R as TR51_R;
#[doc = "Field `TR51` writer - Rising trigger event configuration bit of Configurable Event input x+32"]
pub use TR49_W as TR51_W;
impl R {
    #[doc = "Bit 17 - Rising trigger event configuration bit of Configurable Event input x+32"]
    #[inline(always)]
    pub fn tr49(&self) -> TR49_R {
        TR49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Rising trigger event configuration bit of Configurable Event input x+32"]
    #[inline(always)]
    pub fn tr51(&self) -> TR51_R {
        TR51_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Rising trigger event configuration bit of Configurable Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn tr49(&mut self) -> TR49_W<RTSR2rs> {
        TR49_W::new(self, 17)
    }
    #[doc = "Bit 19 - Rising trigger event configuration bit of Configurable Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn tr51(&mut self) -> TR51_W<RTSR2rs> {
        TR51_W::new(self, 19)
    }
}
#[doc = "EXTI rising trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtsr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtsr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
