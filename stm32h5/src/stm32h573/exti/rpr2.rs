#[doc = "Register `RPR2` reader"]
pub type R = crate::R<RPR2rs>;
#[doc = "Register `RPR2` writer"]
pub type W = crate::W<RPR2rs>;
#[doc = "configurable event inputs x rising edge pending bit When EXTI_SECCFGR.SECx is disabled, RPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RPIFx can only be accessed with secure access. Non-secure write to this RPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF46R {
    #[doc = "0: No trigger request occurred"]
    NotPending = 0,
    #[doc = "1: Selected trigger request occurred"]
    Pending = 1,
}
impl From<RPIF46R> for bool {
    #[inline(always)]
    fn from(variant: RPIF46R) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF46` reader - configurable event inputs x rising edge pending bit When EXTI_SECCFGR.SECx is disabled, RPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RPIFx can only be accessed with secure access. Non-secure write to this RPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF46_R = crate::BitReader<RPIF46R>;
impl RPIF46_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPIF46R {
        match self.bits {
            false => RPIF46R::NotPending,
            true => RPIF46R::Pending,
        }
    }
    #[doc = "No trigger request occurred"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == RPIF46R::NotPending
    }
    #[doc = "Selected trigger request occurred"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RPIF46R::Pending
    }
}
#[doc = "configurable event inputs x rising edge pending bit When EXTI_SECCFGR.SECx is disabled, RPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RPIFx can only be accessed with secure access. Non-secure write to this RPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF46W {
    #[doc = "1: Clears pending bit"]
    Clear = 1,
}
impl From<RPIF46W> for bool {
    #[inline(always)]
    fn from(variant: RPIF46W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF46` writer - configurable event inputs x rising edge pending bit When EXTI_SECCFGR.SECx is disabled, RPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RPIFx can only be accessed with secure access. Non-secure write to this RPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF46_W<'a, REG> = crate::BitWriter1C<'a, REG, RPIF46W>;
impl<'a, REG> RPIF46_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF46W::Clear)
    }
}
#[doc = "Field `RPIF50` reader - configurable event inputs x rising edge pending bit When EXTI_SECCFGR.SECx is disabled, RPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RPIFx can only be accessed with secure access. Non-secure write to this RPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub use RPIF46_R as RPIF50_R;
#[doc = "Field `RPIF53` reader - configurable event inputs x rising edge pending bit When EXTI_SECCFGR.SECx is disabled, RPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RPIFx can only be accessed with secure access. Non-secure write to this RPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub use RPIF46_R as RPIF53_R;
#[doc = "Field `RPIF50` writer - configurable event inputs x rising edge pending bit When EXTI_SECCFGR.SECx is disabled, RPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RPIFx can only be accessed with secure access. Non-secure write to this RPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub use RPIF46_W as RPIF50_W;
#[doc = "Field `RPIF53` writer - configurable event inputs x rising edge pending bit When EXTI_SECCFGR.SECx is disabled, RPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RPIFx can only be accessed with secure access. Non-secure write to this RPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub use RPIF46_W as RPIF53_W;
impl R {
    #[doc = "Bit 14 - configurable event inputs x rising edge pending bit When EXTI_SECCFGR.SECx is disabled, RPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RPIFx can only be accessed with secure access. Non-secure write to this RPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn rpif46(&self) -> RPIF46_R {
        RPIF46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - configurable event inputs x rising edge pending bit When EXTI_SECCFGR.SECx is disabled, RPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RPIFx can only be accessed with secure access. Non-secure write to this RPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn rpif50(&self) -> RPIF50_R {
        RPIF50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - configurable event inputs x rising edge pending bit When EXTI_SECCFGR.SECx is disabled, RPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RPIFx can only be accessed with secure access. Non-secure write to this RPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn rpif53(&self) -> RPIF53_R {
        RPIF53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - configurable event inputs x rising edge pending bit When EXTI_SECCFGR.SECx is disabled, RPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RPIFx can only be accessed with secure access. Non-secure write to this RPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    #[must_use]
    pub fn rpif46(&mut self) -> RPIF46_W<RPR2rs> {
        RPIF46_W::new(self, 14)
    }
    #[doc = "Bit 18 - configurable event inputs x rising edge pending bit When EXTI_SECCFGR.SECx is disabled, RPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RPIFx can only be accessed with secure access. Non-secure write to this RPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    #[must_use]
    pub fn rpif50(&mut self) -> RPIF50_W<RPR2rs> {
        RPIF50_W::new(self, 18)
    }
    #[doc = "Bit 21 - configurable event inputs x rising edge pending bit When EXTI_SECCFGR.SECx is disabled, RPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, RPIFx can only be accessed with secure access. Non-secure write to this RPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    #[must_use]
    pub fn rpif53(&mut self) -> RPIF53_W<RPR2rs> {
        RPIF53_W::new(self, 21)
    }
}
#[doc = "EXTI rising edge pending register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rpr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rpr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RPR2rs;
impl crate::RegisterSpec for RPR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rpr2::R`](R) reader structure"]
impl crate::Readable for RPR2rs {}
#[doc = "`write(|w| ..)` method takes [`rpr2::W`](W) writer structure"]
impl crate::Writable for RPR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0024_4000;
}
#[doc = "`reset()` method sets RPR2 to value 0"]
impl crate::Resettable for RPR2rs {
    const RESET_VALUE: u32 = 0;
}
