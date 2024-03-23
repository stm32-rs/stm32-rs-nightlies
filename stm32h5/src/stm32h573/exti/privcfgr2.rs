#[doc = "Register `PRIVCFGR2` reader"]
pub type R = crate::R<PRIVCFGR2rs>;
#[doc = "Register `PRIVCFGR2` writer"]
pub type W = crate::W<PRIVCFGR2rs>;
#[doc = "Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV32 {
    #[doc = "0: Event privilege disabled"]
    Unprivileged = 0,
    #[doc = "1: Event privilege enabled"]
    Privileged = 1,
}
impl From<PRIV32> for bool {
    #[inline(always)]
    fn from(variant: PRIV32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV32` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV32_R = crate::BitReader<PRIV32>;
impl PRIV32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV32 {
        match self.bits {
            false => PRIV32::Unprivileged,
            true => PRIV32::Privileged,
        }
    }
    #[doc = "Event privilege disabled"]
    #[inline(always)]
    pub fn is_unprivileged(&self) -> bool {
        *self == PRIV32::Unprivileged
    }
    #[doc = "Event privilege enabled"]
    #[inline(always)]
    pub fn is_privileged(&self) -> bool {
        *self == PRIV32::Privileged
    }
}
#[doc = "Field `PRIV32` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV32_W<'a, REG> = crate::BitWriter<'a, REG, PRIV32>;
impl<'a, REG> PRIV32_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event privilege disabled"]
    #[inline(always)]
    pub fn unprivileged(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV32::Unprivileged)
    }
    #[doc = "Event privilege enabled"]
    #[inline(always)]
    pub fn privileged(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV32::Privileged)
    }
}
#[doc = "Field `PRIV33` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_R as PRIV33_R;
#[doc = "Field `PRIV34` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_R as PRIV34_R;
#[doc = "Field `PRIV35` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_R as PRIV35_R;
#[doc = "Field `PRIV36` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_R as PRIV36_R;
#[doc = "Field `PRIV37` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_R as PRIV37_R;
#[doc = "Field `PRIV38` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_R as PRIV38_R;
#[doc = "Field `PRIV39` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_R as PRIV39_R;
#[doc = "Field `PRIV40` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_R as PRIV40_R;
#[doc = "Field `PRIV41` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_R as PRIV41_R;
#[doc = "Field `PRIV42` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_R as PRIV42_R;
#[doc = "Field `PRIV43` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_R as PRIV43_R;
#[doc = "Field `PRIV44` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_R as PRIV44_R;
#[doc = "Field `PRIV45` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_R as PRIV45_R;
#[doc = "Field `PRIV46` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_R as PRIV46_R;
#[doc = "Field `PRIV47` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_R as PRIV47_R;
#[doc = "Field `PRIV48` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_R as PRIV48_R;
#[doc = "Field `PRIV49` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_R as PRIV49_R;
#[doc = "Field `PRIV50` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_R as PRIV50_R;
#[doc = "Field `PRIV51` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_R as PRIV51_R;
#[doc = "Field `PRIV52` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_R as PRIV52_R;
#[doc = "Field `PRIV53` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_R as PRIV53_R;
#[doc = "Field `PRIV54` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_R as PRIV54_R;
#[doc = "Field `PRIV55` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_R as PRIV55_R;
#[doc = "Field `PRIV56` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_R as PRIV56_R;
#[doc = "Field `PRIV57` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_R as PRIV57_R;
#[doc = "Field `PRIV33` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_W as PRIV33_W;
#[doc = "Field `PRIV34` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_W as PRIV34_W;
#[doc = "Field `PRIV35` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_W as PRIV35_W;
#[doc = "Field `PRIV36` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_W as PRIV36_W;
#[doc = "Field `PRIV37` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_W as PRIV37_W;
#[doc = "Field `PRIV38` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_W as PRIV38_W;
#[doc = "Field `PRIV39` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_W as PRIV39_W;
#[doc = "Field `PRIV40` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_W as PRIV40_W;
#[doc = "Field `PRIV41` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_W as PRIV41_W;
#[doc = "Field `PRIV42` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_W as PRIV42_W;
#[doc = "Field `PRIV43` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_W as PRIV43_W;
#[doc = "Field `PRIV44` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_W as PRIV44_W;
#[doc = "Field `PRIV45` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_W as PRIV45_W;
#[doc = "Field `PRIV46` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_W as PRIV46_W;
#[doc = "Field `PRIV47` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_W as PRIV47_W;
#[doc = "Field `PRIV48` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_W as PRIV48_W;
#[doc = "Field `PRIV49` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_W as PRIV49_W;
#[doc = "Field `PRIV50` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_W as PRIV50_W;
#[doc = "Field `PRIV51` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_W as PRIV51_W;
#[doc = "Field `PRIV52` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_W as PRIV52_W;
#[doc = "Field `PRIV53` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_W as PRIV53_W;
#[doc = "Field `PRIV54` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_W as PRIV54_W;
#[doc = "Field `PRIV55` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_W as PRIV55_W;
#[doc = "Field `PRIV56` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_W as PRIV56_W;
#[doc = "Field `PRIV57` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV32_W as PRIV57_W;
impl R {
    #[doc = "Bit 0 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv32(&self) -> PRIV32_R {
        PRIV32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv33(&self) -> PRIV33_R {
        PRIV33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv34(&self) -> PRIV34_R {
        PRIV34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv35(&self) -> PRIV35_R {
        PRIV35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv36(&self) -> PRIV36_R {
        PRIV36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv37(&self) -> PRIV37_R {
        PRIV37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv38(&self) -> PRIV38_R {
        PRIV38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv39(&self) -> PRIV39_R {
        PRIV39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv40(&self) -> PRIV40_R {
        PRIV40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv41(&self) -> PRIV41_R {
        PRIV41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv42(&self) -> PRIV42_R {
        PRIV42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv43(&self) -> PRIV43_R {
        PRIV43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv44(&self) -> PRIV44_R {
        PRIV44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv45(&self) -> PRIV45_R {
        PRIV45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv46(&self) -> PRIV46_R {
        PRIV46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv47(&self) -> PRIV47_R {
        PRIV47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv48(&self) -> PRIV48_R {
        PRIV48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv49(&self) -> PRIV49_R {
        PRIV49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv50(&self) -> PRIV50_R {
        PRIV50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv51(&self) -> PRIV51_R {
        PRIV51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv52(&self) -> PRIV52_R {
        PRIV52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv53(&self) -> PRIV53_R {
        PRIV53_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv54(&self) -> PRIV54_R {
        PRIV54_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv55(&self) -> PRIV55_R {
        PRIV55_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv56(&self) -> PRIV56_R {
        PRIV56_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv57(&self) -> PRIV57_R {
        PRIV57_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv32(&mut self) -> PRIV32_W<PRIVCFGR2rs> {
        PRIV32_W::new(self, 0)
    }
    #[doc = "Bit 1 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv33(&mut self) -> PRIV33_W<PRIVCFGR2rs> {
        PRIV33_W::new(self, 1)
    }
    #[doc = "Bit 2 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv34(&mut self) -> PRIV34_W<PRIVCFGR2rs> {
        PRIV34_W::new(self, 2)
    }
    #[doc = "Bit 3 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv35(&mut self) -> PRIV35_W<PRIVCFGR2rs> {
        PRIV35_W::new(self, 3)
    }
    #[doc = "Bit 4 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv36(&mut self) -> PRIV36_W<PRIVCFGR2rs> {
        PRIV36_W::new(self, 4)
    }
    #[doc = "Bit 5 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv37(&mut self) -> PRIV37_W<PRIVCFGR2rs> {
        PRIV37_W::new(self, 5)
    }
    #[doc = "Bit 6 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv38(&mut self) -> PRIV38_W<PRIVCFGR2rs> {
        PRIV38_W::new(self, 6)
    }
    #[doc = "Bit 7 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv39(&mut self) -> PRIV39_W<PRIVCFGR2rs> {
        PRIV39_W::new(self, 7)
    }
    #[doc = "Bit 8 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv40(&mut self) -> PRIV40_W<PRIVCFGR2rs> {
        PRIV40_W::new(self, 8)
    }
    #[doc = "Bit 9 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv41(&mut self) -> PRIV41_W<PRIVCFGR2rs> {
        PRIV41_W::new(self, 9)
    }
    #[doc = "Bit 10 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv42(&mut self) -> PRIV42_W<PRIVCFGR2rs> {
        PRIV42_W::new(self, 10)
    }
    #[doc = "Bit 11 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv43(&mut self) -> PRIV43_W<PRIVCFGR2rs> {
        PRIV43_W::new(self, 11)
    }
    #[doc = "Bit 12 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv44(&mut self) -> PRIV44_W<PRIVCFGR2rs> {
        PRIV44_W::new(self, 12)
    }
    #[doc = "Bit 13 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv45(&mut self) -> PRIV45_W<PRIVCFGR2rs> {
        PRIV45_W::new(self, 13)
    }
    #[doc = "Bit 14 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv46(&mut self) -> PRIV46_W<PRIVCFGR2rs> {
        PRIV46_W::new(self, 14)
    }
    #[doc = "Bit 15 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv47(&mut self) -> PRIV47_W<PRIVCFGR2rs> {
        PRIV47_W::new(self, 15)
    }
    #[doc = "Bit 16 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv48(&mut self) -> PRIV48_W<PRIVCFGR2rs> {
        PRIV48_W::new(self, 16)
    }
    #[doc = "Bit 17 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv49(&mut self) -> PRIV49_W<PRIVCFGR2rs> {
        PRIV49_W::new(self, 17)
    }
    #[doc = "Bit 18 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv50(&mut self) -> PRIV50_W<PRIVCFGR2rs> {
        PRIV50_W::new(self, 18)
    }
    #[doc = "Bit 19 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv51(&mut self) -> PRIV51_W<PRIVCFGR2rs> {
        PRIV51_W::new(self, 19)
    }
    #[doc = "Bit 20 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv52(&mut self) -> PRIV52_W<PRIVCFGR2rs> {
        PRIV52_W::new(self, 20)
    }
    #[doc = "Bit 21 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv53(&mut self) -> PRIV53_W<PRIVCFGR2rs> {
        PRIV53_W::new(self, 21)
    }
    #[doc = "Bit 22 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv54(&mut self) -> PRIV54_W<PRIVCFGR2rs> {
        PRIV54_W::new(self, 22)
    }
    #[doc = "Bit 23 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv55(&mut self) -> PRIV55_W<PRIVCFGR2rs> {
        PRIV55_W::new(self, 23)
    }
    #[doc = "Bit 24 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv56(&mut self) -> PRIV56_W<PRIVCFGR2rs> {
        PRIV56_W::new(self, 24)
    }
    #[doc = "Bit 25 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv57(&mut self) -> PRIV57_W<PRIVCFGR2rs> {
        PRIV57_W::new(self, 25)
    }
}
#[doc = "EXTI privilege configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privcfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privcfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIVCFGR2rs;
impl crate::RegisterSpec for PRIVCFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`privcfgr2::R`](R) reader structure"]
impl crate::Readable for PRIVCFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`privcfgr2::W`](W) writer structure"]
impl crate::Writable for PRIVCFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIVCFGR2 to value 0"]
impl crate::Resettable for PRIVCFGR2rs {
    const RESET_VALUE: u32 = 0;
}
