#[doc = "Register `FPR2` reader"]
pub type R = crate::R<FPR2rs>;
#[doc = "Register `FPR2` writer"]
pub type W = crate::W<FPR2rs>;
#[doc = "configurable event inputs x falling edge pending bit When EXTI_SECCFGR.SECx is disabled, FPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FPIFx can only be accessed with secure access. Non-secure write to this FPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF46R {
    #[doc = "0: No trigger request occurred"]
    NotPending = 0,
    #[doc = "1: Selected trigger request occurred"]
    Pending = 1,
}
impl From<FPIF46R> for bool {
    #[inline(always)]
    fn from(variant: FPIF46R) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPIF46` reader - configurable event inputs x falling edge pending bit When EXTI_SECCFGR.SECx is disabled, FPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FPIFx can only be accessed with secure access. Non-secure write to this FPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF46_R = crate::BitReader<FPIF46R>;
impl FPIF46_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPIF46R {
        match self.bits {
            false => FPIF46R::NotPending,
            true => FPIF46R::Pending,
        }
    }
    #[doc = "No trigger request occurred"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == FPIF46R::NotPending
    }
    #[doc = "Selected trigger request occurred"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == FPIF46R::Pending
    }
}
#[doc = "configurable event inputs x falling edge pending bit When EXTI_SECCFGR.SECx is disabled, FPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FPIFx can only be accessed with secure access. Non-secure write to this FPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF46W {
    #[doc = "1: Clears pending bit"]
    Clear = 1,
}
impl From<FPIF46W> for bool {
    #[inline(always)]
    fn from(variant: FPIF46W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPIF46` writer - configurable event inputs x falling edge pending bit When EXTI_SECCFGR.SECx is disabled, FPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FPIFx can only be accessed with secure access. Non-secure write to this FPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF46_W<'a, REG> = crate::BitWriter1C<'a, REG, FPIF46W>;
impl<'a, REG> FPIF46_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF46W::Clear)
    }
}
#[doc = "Field `FPIF50` reader - configurable event inputs x falling edge pending bit When EXTI_SECCFGR.SECx is disabled, FPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FPIFx can only be accessed with secure access. Non-secure write to this FPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub use FPIF46_R as FPIF50_R;
#[doc = "Field `FPIF53` reader - configurable event inputs x falling edge pending bit When EXTI_SECCFGR.SECx is disabled, FPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FPIFx can only be accessed with secure access. Non-secure write to this FPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub use FPIF46_R as FPIF53_R;
#[doc = "Field `FPIF50` writer - configurable event inputs x falling edge pending bit When EXTI_SECCFGR.SECx is disabled, FPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FPIFx can only be accessed with secure access. Non-secure write to this FPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub use FPIF46_W as FPIF50_W;
#[doc = "Field `FPIF53` writer - configurable event inputs x falling edge pending bit When EXTI_SECCFGR.SECx is disabled, FPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FPIFx can only be accessed with secure access. Non-secure write to this FPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub use FPIF46_W as FPIF53_W;
impl R {
    #[doc = "Bit 14 - configurable event inputs x falling edge pending bit When EXTI_SECCFGR.SECx is disabled, FPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FPIFx can only be accessed with secure access. Non-secure write to this FPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn fpif46(&self) -> FPIF46_R {
        FPIF46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - configurable event inputs x falling edge pending bit When EXTI_SECCFGR.SECx is disabled, FPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FPIFx can only be accessed with secure access. Non-secure write to this FPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn fpif50(&self) -> FPIF50_R {
        FPIF50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - configurable event inputs x falling edge pending bit When EXTI_SECCFGR.SECx is disabled, FPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FPIFx can only be accessed with secure access. Non-secure write to this FPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn fpif53(&self) -> FPIF53_R {
        FPIF53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - configurable event inputs x falling edge pending bit When EXTI_SECCFGR.SECx is disabled, FPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FPIFx can only be accessed with secure access. Non-secure write to this FPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    #[must_use]
    pub fn fpif46(&mut self) -> FPIF46_W<FPR2rs> {
        FPIF46_W::new(self, 14)
    }
    #[doc = "Bit 18 - configurable event inputs x falling edge pending bit When EXTI_SECCFGR.SECx is disabled, FPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FPIFx can only be accessed with secure access. Non-secure write to this FPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    #[must_use]
    pub fn fpif50(&mut self) -> FPIF50_W<FPR2rs> {
        FPIF50_W::new(self, 18)
    }
    #[doc = "Bit 21 - configurable event inputs x falling edge pending bit When EXTI_SECCFGR.SECx is disabled, FPIFx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, FPIFx can only be accessed with secure access. Non-secure write to this FPIFx is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    #[must_use]
    pub fn fpif53(&mut self) -> FPIF53_W<FPR2rs> {
        FPIF53_W::new(self, 21)
    }
}
#[doc = "EXTI falling edge pending register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FPR2rs;
impl crate::RegisterSpec for FPR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpr2::R`](R) reader structure"]
impl crate::Readable for FPR2rs {}
#[doc = "`write(|w| ..)` method takes [`fpr2::W`](W) writer structure"]
impl crate::Writable for FPR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0024_4000;
}
#[doc = "`reset()` method sets FPR2 to value 0"]
impl crate::Resettable for FPR2rs {
    const RESET_VALUE: u32 = 0;
}
