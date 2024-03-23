#[doc = "Register `RTSR2` reader"]
pub type R = crate::R<RTSR2rs>;
#[doc = "Register `RTSR2` writer"]
pub type W = crate::W<RTSR2rs>;
#[doc = "Rising trigger event configuration bit of configurable event input x&lt;sup>(1)&lt;/sup> When EXTI_SECCFGR.SECx is disabled, RTx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RTx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT46 {
    #[doc = "0: Rising edge trigger is disabled"]
    Disabled = 0,
    #[doc = "1: Rising edge trigger is enabled"]
    Enabled = 1,
}
impl From<RT46> for bool {
    #[inline(always)]
    fn from(variant: RT46) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT46` reader - Rising trigger event configuration bit of configurable event input x&lt;sup>(1)&lt;/sup> When EXTI_SECCFGR.SECx is disabled, RTx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RTx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT46_R = crate::BitReader<RT46>;
impl RT46_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT46 {
        match self.bits {
            false => RT46::Disabled,
            true => RT46::Enabled,
        }
    }
    #[doc = "Rising edge trigger is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RT46::Disabled
    }
    #[doc = "Rising edge trigger is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RT46::Enabled
    }
}
#[doc = "Field `RT46` writer - Rising trigger event configuration bit of configurable event input x&lt;sup>(1)&lt;/sup> When EXTI_SECCFGR.SECx is disabled, RTx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RTx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT46_W<'a, REG> = crate::BitWriter<'a, REG, RT46>;
impl<'a, REG> RT46_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RT46::Disabled)
    }
    #[doc = "Rising edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RT46::Enabled)
    }
}
#[doc = "Field `RT50` reader - Rising trigger event configuration bit of configurable event input x&lt;sup>(1)&lt;/sup> When EXTI_SECCFGR.SECx is disabled, RTx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RTx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub use RT46_R as RT50_R;
#[doc = "Field `RT53` reader - Rising trigger event configuration bit of configurable event input x When EXTI_SECCFGR.SECx is disabled, RTx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RTx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub use RT46_R as RT53_R;
#[doc = "Field `RT50` writer - Rising trigger event configuration bit of configurable event input x&lt;sup>(1)&lt;/sup> When EXTI_SECCFGR.SECx is disabled, RTx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RTx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub use RT46_W as RT50_W;
#[doc = "Field `RT53` writer - Rising trigger event configuration bit of configurable event input x When EXTI_SECCFGR.SECx is disabled, RTx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RTx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub use RT46_W as RT53_W;
impl R {
    #[doc = "Bit 14 - Rising trigger event configuration bit of configurable event input x&lt;sup>(1)&lt;/sup> When EXTI_SECCFGR.SECx is disabled, RTx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RTx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn rt46(&self) -> RT46_R {
        RT46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - Rising trigger event configuration bit of configurable event input x&lt;sup>(1)&lt;/sup> When EXTI_SECCFGR.SECx is disabled, RTx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RTx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn rt50(&self) -> RT50_R {
        RT50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - Rising trigger event configuration bit of configurable event input x When EXTI_SECCFGR.SECx is disabled, RTx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RTx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn rt53(&self) -> RT53_R {
        RT53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Rising trigger event configuration bit of configurable event input x&lt;sup>(1)&lt;/sup> When EXTI_SECCFGR.SECx is disabled, RTx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RTx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn rt46(&mut self) -> RT46_W<RTSR2rs> {
        RT46_W::new(self, 14)
    }
    #[doc = "Bit 18 - Rising trigger event configuration bit of configurable event input x&lt;sup>(1)&lt;/sup> When EXTI_SECCFGR.SECx is disabled, RTx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RTx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn rt50(&mut self) -> RT50_W<RTSR2rs> {
        RT50_W::new(self, 18)
    }
    #[doc = "Bit 21 - Rising trigger event configuration bit of configurable event input x When EXTI_SECCFGR.SECx is disabled, RTx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RTx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn rt53(&mut self) -> RT53_W<RTSR2rs> {
        RT53_W::new(self, 21)
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
