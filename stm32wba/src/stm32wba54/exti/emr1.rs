///Register `EMR1` reader
pub type R = crate::R<EMR1rs>;
///Register `EMR1` writer
pub type W = crate::W<EMR1rs>;
///Field `EM0` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM0_R = crate::BitReader;
///Field `EM0` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM1` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM1_R = crate::BitReader;
///Field `EM1` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM2` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM2_R = crate::BitReader;
///Field `EM2` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM3` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM3_R = crate::BitReader;
///Field `EM3` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM4` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM4_R = crate::BitReader;
///Field `EM4` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM5` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM5_R = crate::BitReader;
///Field `EM5` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM6` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM6_R = crate::BitReader;
///Field `EM6` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM7` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM7_R = crate::BitReader;
///Field `EM7` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM8` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM8_R = crate::BitReader;
///Field `EM8` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM9` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM9_R = crate::BitReader;
///Field `EM9` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM10` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM10_R = crate::BitReader;
///Field `EM10` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM11` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM11_R = crate::BitReader;
///Field `EM11` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM12` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM12_R = crate::BitReader;
///Field `EM12` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM13` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM13_R = crate::BitReader;
///Field `EM13` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM14` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM14_R = crate::BitReader;
///Field `EM14` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM15` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM15_R = crate::BitReader;
///Field `EM15` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM16` reader - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM16_R = crate::BitReader;
///Field `EM16` writer - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type EM16_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em0(&self) -> EM0_R {
        EM0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em1(&self) -> EM1_R {
        EM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em2(&self) -> EM2_R {
        EM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em3(&self) -> EM3_R {
        EM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em4(&self) -> EM4_R {
        EM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em5(&self) -> EM5_R {
        EM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em6(&self) -> EM6_R {
        EM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em7(&self) -> EM7_R {
        EM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em8(&self) -> EM8_R {
        EM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em9(&self) -> EM9_R {
        EM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em10(&self) -> EM10_R {
        EM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em11(&self) -> EM11_R {
        EM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em12(&self) -> EM12_R {
        EM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em13(&self) -> EM13_R {
        EM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em14(&self) -> EM14_R {
        EM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em15(&self) -> EM15_R {
        EM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em16(&self) -> EM16_R {
        EM16_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMR1")
            .field("em0", &self.em0())
            .field("em1", &self.em1())
            .field("em2", &self.em2())
            .field("em3", &self.em3())
            .field("em4", &self.em4())
            .field("em5", &self.em5())
            .field("em6", &self.em6())
            .field("em7", &self.em7())
            .field("em8", &self.em8())
            .field("em9", &self.em9())
            .field("em10", &self.em10())
            .field("em11", &self.em11())
            .field("em12", &self.em12())
            .field("em13", &self.em13())
            .field("em14", &self.em14())
            .field("em15", &self.em15())
            .field("em16", &self.em16())
            .finish()
    }
}
impl W {
    ///Bit 0 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em0(&mut self) -> EM0_W<'_, EMR1rs> {
        EM0_W::new(self, 0)
    }
    ///Bit 1 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em1(&mut self) -> EM1_W<'_, EMR1rs> {
        EM1_W::new(self, 1)
    }
    ///Bit 2 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em2(&mut self) -> EM2_W<'_, EMR1rs> {
        EM2_W::new(self, 2)
    }
    ///Bit 3 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em3(&mut self) -> EM3_W<'_, EMR1rs> {
        EM3_W::new(self, 3)
    }
    ///Bit 4 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em4(&mut self) -> EM4_W<'_, EMR1rs> {
        EM4_W::new(self, 4)
    }
    ///Bit 5 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em5(&mut self) -> EM5_W<'_, EMR1rs> {
        EM5_W::new(self, 5)
    }
    ///Bit 6 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em6(&mut self) -> EM6_W<'_, EMR1rs> {
        EM6_W::new(self, 6)
    }
    ///Bit 7 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em7(&mut self) -> EM7_W<'_, EMR1rs> {
        EM7_W::new(self, 7)
    }
    ///Bit 8 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em8(&mut self) -> EM8_W<'_, EMR1rs> {
        EM8_W::new(self, 8)
    }
    ///Bit 9 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em9(&mut self) -> EM9_W<'_, EMR1rs> {
        EM9_W::new(self, 9)
    }
    ///Bit 10 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em10(&mut self) -> EM10_W<'_, EMR1rs> {
        EM10_W::new(self, 10)
    }
    ///Bit 11 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em11(&mut self) -> EM11_W<'_, EMR1rs> {
        EM11_W::new(self, 11)
    }
    ///Bit 12 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em12(&mut self) -> EM12_W<'_, EMR1rs> {
        EM12_W::new(self, 12)
    }
    ///Bit 13 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em13(&mut self) -> EM13_W<'_, EMR1rs> {
        EM13_W::new(self, 13)
    }
    ///Bit 14 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em14(&mut self) -> EM14_W<'_, EMR1rs> {
        EM14_W::new(self, 14)
    }
    ///Bit 15 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em15(&mut self) -> EM15_W<'_, EMR1rs> {
        EM15_W::new(self, 15)
    }
    ///Bit 16 - CPU wakeup with event generation mask on event input x When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access. Non-secure write to this bit x is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn em16(&mut self) -> EM16_W<'_, EMR1rs> {
        EM16_W::new(self, 16)
    }
}
/**EXTI CPU wakeup with event mask register

You can [`read`](crate::Reg::read) this register and get [`emr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#EXTI:EMR1)*/
pub struct EMR1rs;
impl crate::RegisterSpec for EMR1rs {
    type Ux = u32;
}
///`read()` method returns [`emr1::R`](R) reader structure
impl crate::Readable for EMR1rs {}
///`write(|w| ..)` method takes [`emr1::W`](W) writer structure
impl crate::Writable for EMR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EMR1 to value 0
impl crate::Resettable for EMR1rs {}
