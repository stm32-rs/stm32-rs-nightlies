#[doc = "Register `PRIVCFGR1` reader"]
pub type R = crate::R<PRIVCFGR1rs>;
#[doc = "Register `PRIVCFGR1` writer"]
pub type W = crate::W<PRIVCFGR1rs>;
#[doc = "Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV0 {
    #[doc = "0: Event privilege disabled"]
    Unprivileged = 0,
    #[doc = "1: Event privilege enabled"]
    Privileged = 1,
}
impl From<PRIV0> for bool {
    #[inline(always)]
    fn from(variant: PRIV0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV0` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV0_R = crate::BitReader<PRIV0>;
impl PRIV0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV0 {
        match self.bits {
            false => PRIV0::Unprivileged,
            true => PRIV0::Privileged,
        }
    }
    #[doc = "Event privilege disabled"]
    #[inline(always)]
    pub fn is_unprivileged(&self) -> bool {
        *self == PRIV0::Unprivileged
    }
    #[doc = "Event privilege enabled"]
    #[inline(always)]
    pub fn is_privileged(&self) -> bool {
        *self == PRIV0::Privileged
    }
}
#[doc = "Field `PRIV0` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub type PRIV0_W<'a, REG> = crate::BitWriter<'a, REG, PRIV0>;
impl<'a, REG> PRIV0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event privilege disabled"]
    #[inline(always)]
    pub fn unprivileged(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV0::Unprivileged)
    }
    #[doc = "Event privilege enabled"]
    #[inline(always)]
    pub fn privileged(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV0::Privileged)
    }
}
#[doc = "Field `PRIV1` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV1_R;
#[doc = "Field `PRIV2` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV2_R;
#[doc = "Field `PRIV3` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV3_R;
#[doc = "Field `PRIV4` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV4_R;
#[doc = "Field `PRIV5` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV5_R;
#[doc = "Field `PRIV6` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV6_R;
#[doc = "Field `PRIV7` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV7_R;
#[doc = "Field `PRIV8` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV8_R;
#[doc = "Field `PRIV9` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV9_R;
#[doc = "Field `PRIV10` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV10_R;
#[doc = "Field `PRIV11` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV11_R;
#[doc = "Field `PRIV12` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV12_R;
#[doc = "Field `PRIV13` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV13_R;
#[doc = "Field `PRIV14` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV14_R;
#[doc = "Field `PRIV15` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV15_R;
#[doc = "Field `PRIV16` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV16_R;
#[doc = "Field `PRIV17` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV17_R;
#[doc = "Field `PRIV18` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV18_R;
#[doc = "Field `PRIV19` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV19_R;
#[doc = "Field `PRIV20` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV20_R;
#[doc = "Field `PRIV21` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV21_R;
#[doc = "Field `PRIV22` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV22_R;
#[doc = "Field `PRIV23` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV23_R;
#[doc = "Field `PRIV24` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV24_R;
#[doc = "Field `PRIV25` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV25_R;
#[doc = "Field `PRIV26` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV26_R;
#[doc = "Field `PRIV27` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV27_R;
#[doc = "Field `PRIV28` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV28_R;
#[doc = "Field `PRIV29` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV29_R;
#[doc = "Field `PRIV30` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV30_R;
#[doc = "Field `PRIV31` reader - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_R as PRIV31_R;
#[doc = "Field `PRIV1` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV1_W;
#[doc = "Field `PRIV2` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV2_W;
#[doc = "Field `PRIV3` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV3_W;
#[doc = "Field `PRIV4` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV4_W;
#[doc = "Field `PRIV5` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV5_W;
#[doc = "Field `PRIV6` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV6_W;
#[doc = "Field `PRIV7` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV7_W;
#[doc = "Field `PRIV8` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV8_W;
#[doc = "Field `PRIV9` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV9_W;
#[doc = "Field `PRIV10` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV10_W;
#[doc = "Field `PRIV11` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV11_W;
#[doc = "Field `PRIV12` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV12_W;
#[doc = "Field `PRIV13` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV13_W;
#[doc = "Field `PRIV14` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV14_W;
#[doc = "Field `PRIV15` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV15_W;
#[doc = "Field `PRIV16` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV16_W;
#[doc = "Field `PRIV17` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV17_W;
#[doc = "Field `PRIV18` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV18_W;
#[doc = "Field `PRIV19` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV19_W;
#[doc = "Field `PRIV20` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV20_W;
#[doc = "Field `PRIV21` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV21_W;
#[doc = "Field `PRIV22` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV22_W;
#[doc = "Field `PRIV23` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV23_W;
#[doc = "Field `PRIV24` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV24_W;
#[doc = "Field `PRIV25` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV25_W;
#[doc = "Field `PRIV26` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV26_W;
#[doc = "Field `PRIV27` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV27_W;
#[doc = "Field `PRIV28` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV28_W;
#[doc = "Field `PRIV29` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV29_W;
#[doc = "Field `PRIV30` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV30_W;
#[doc = "Field `PRIV31` writer - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
pub use PRIV0_W as PRIV31_W;
impl R {
    #[doc = "Bit 0 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv0(&self) -> PRIV0_R {
        PRIV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv1(&self) -> PRIV1_R {
        PRIV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv2(&self) -> PRIV2_R {
        PRIV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv3(&self) -> PRIV3_R {
        PRIV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv4(&self) -> PRIV4_R {
        PRIV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv5(&self) -> PRIV5_R {
        PRIV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv6(&self) -> PRIV6_R {
        PRIV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv7(&self) -> PRIV7_R {
        PRIV7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv8(&self) -> PRIV8_R {
        PRIV8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv9(&self) -> PRIV9_R {
        PRIV9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv10(&self) -> PRIV10_R {
        PRIV10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv11(&self) -> PRIV11_R {
        PRIV11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv12(&self) -> PRIV12_R {
        PRIV12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv13(&self) -> PRIV13_R {
        PRIV13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv14(&self) -> PRIV14_R {
        PRIV14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv15(&self) -> PRIV15_R {
        PRIV15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv16(&self) -> PRIV16_R {
        PRIV16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv17(&self) -> PRIV17_R {
        PRIV17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv18(&self) -> PRIV18_R {
        PRIV18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv19(&self) -> PRIV19_R {
        PRIV19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv20(&self) -> PRIV20_R {
        PRIV20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv21(&self) -> PRIV21_R {
        PRIV21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv22(&self) -> PRIV22_R {
        PRIV22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv23(&self) -> PRIV23_R {
        PRIV23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv24(&self) -> PRIV24_R {
        PRIV24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv25(&self) -> PRIV25_R {
        PRIV25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv26(&self) -> PRIV26_R {
        PRIV26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv27(&self) -> PRIV27_R {
        PRIV27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv28(&self) -> PRIV28_R {
        PRIV28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv29(&self) -> PRIV29_R {
        PRIV29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv30(&self) -> PRIV30_R {
        PRIV30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    pub fn priv31(&self) -> PRIV31_R {
        PRIV31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv0(&mut self) -> PRIV0_W<PRIVCFGR1rs> {
        PRIV0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv1(&mut self) -> PRIV1_W<PRIVCFGR1rs> {
        PRIV1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv2(&mut self) -> PRIV2_W<PRIVCFGR1rs> {
        PRIV2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv3(&mut self) -> PRIV3_W<PRIVCFGR1rs> {
        PRIV3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv4(&mut self) -> PRIV4_W<PRIVCFGR1rs> {
        PRIV4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv5(&mut self) -> PRIV5_W<PRIVCFGR1rs> {
        PRIV5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv6(&mut self) -> PRIV6_W<PRIVCFGR1rs> {
        PRIV6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv7(&mut self) -> PRIV7_W<PRIVCFGR1rs> {
        PRIV7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv8(&mut self) -> PRIV8_W<PRIVCFGR1rs> {
        PRIV8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv9(&mut self) -> PRIV9_W<PRIVCFGR1rs> {
        PRIV9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv10(&mut self) -> PRIV10_W<PRIVCFGR1rs> {
        PRIV10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv11(&mut self) -> PRIV11_W<PRIVCFGR1rs> {
        PRIV11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv12(&mut self) -> PRIV12_W<PRIVCFGR1rs> {
        PRIV12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv13(&mut self) -> PRIV13_W<PRIVCFGR1rs> {
        PRIV13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv14(&mut self) -> PRIV14_W<PRIVCFGR1rs> {
        PRIV14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv15(&mut self) -> PRIV15_W<PRIVCFGR1rs> {
        PRIV15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv16(&mut self) -> PRIV16_W<PRIVCFGR1rs> {
        PRIV16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv17(&mut self) -> PRIV17_W<PRIVCFGR1rs> {
        PRIV17_W::new(self, 17)
    }
    #[doc = "Bit 18 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv18(&mut self) -> PRIV18_W<PRIVCFGR1rs> {
        PRIV18_W::new(self, 18)
    }
    #[doc = "Bit 19 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv19(&mut self) -> PRIV19_W<PRIVCFGR1rs> {
        PRIV19_W::new(self, 19)
    }
    #[doc = "Bit 20 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv20(&mut self) -> PRIV20_W<PRIVCFGR1rs> {
        PRIV20_W::new(self, 20)
    }
    #[doc = "Bit 21 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv21(&mut self) -> PRIV21_W<PRIVCFGR1rs> {
        PRIV21_W::new(self, 21)
    }
    #[doc = "Bit 22 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv22(&mut self) -> PRIV22_W<PRIVCFGR1rs> {
        PRIV22_W::new(self, 22)
    }
    #[doc = "Bit 23 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv23(&mut self) -> PRIV23_W<PRIVCFGR1rs> {
        PRIV23_W::new(self, 23)
    }
    #[doc = "Bit 24 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv24(&mut self) -> PRIV24_W<PRIVCFGR1rs> {
        PRIV24_W::new(self, 24)
    }
    #[doc = "Bit 25 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv25(&mut self) -> PRIV25_W<PRIVCFGR1rs> {
        PRIV25_W::new(self, 25)
    }
    #[doc = "Bit 26 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv26(&mut self) -> PRIV26_W<PRIVCFGR1rs> {
        PRIV26_W::new(self, 26)
    }
    #[doc = "Bit 27 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv27(&mut self) -> PRIV27_W<PRIVCFGR1rs> {
        PRIV27_W::new(self, 27)
    }
    #[doc = "Bit 28 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv28(&mut self) -> PRIV28_W<PRIVCFGR1rs> {
        PRIV28_W::new(self, 28)
    }
    #[doc = "Bit 29 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv29(&mut self) -> PRIV29_W<PRIVCFGR1rs> {
        PRIV29_W::new(self, 29)
    }
    #[doc = "Bit 30 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv30(&mut self) -> PRIV30_W<PRIVCFGR1rs> {
        PRIV30_W::new(self, 30)
    }
    #[doc = "Bit 31 - Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn priv31(&mut self) -> PRIV31_W<PRIVCFGR1rs> {
        PRIV31_W::new(self, 31)
    }
}
#[doc = "EXTI privilege configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privcfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privcfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIVCFGR1rs;
impl crate::RegisterSpec for PRIVCFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`privcfgr1::R`](R) reader structure"]
impl crate::Readable for PRIVCFGR1rs {}
#[doc = "`write(|w| ..)` method takes [`privcfgr1::W`](W) writer structure"]
impl crate::Writable for PRIVCFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIVCFGR1 to value 0"]
impl crate::Resettable for PRIVCFGR1rs {
    const RESET_VALUE: u32 = 0;
}
