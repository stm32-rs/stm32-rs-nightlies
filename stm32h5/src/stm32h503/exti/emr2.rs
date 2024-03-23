#[doc = "Register `EMR2` reader"]
pub type R = crate::R<EMR2rs>;
#[doc = "Register `EMR2` writer"]
pub type W = crate::W<EMR2rs>;
#[doc = "CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM37 {
    #[doc = "0: Interrupt request line is masked"]
    Masked = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    Unmasked = 1,
}
impl From<EM37> for bool {
    #[inline(always)]
    fn from(variant: EM37) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM37` reader - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM37_R = crate::BitReader<EM37>;
impl EM37_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM37 {
        match self.bits {
            false => EM37::Masked,
            true => EM37::Unmasked,
        }
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == EM37::Masked
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == EM37::Unmasked
    }
}
#[doc = "Field `EM37` writer - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM37_W<'a, REG> = crate::BitWriter<'a, REG, EM37>;
impl<'a, REG> EM37_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(EM37::Masked)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(EM37::Unmasked)
    }
}
#[doc = "Field `EM38` reader - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use EM37_R as EM38_R;
#[doc = "Field `EM39` reader - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use EM37_R as EM39_R;
#[doc = "Field `EM40` reader - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use EM37_R as EM40_R;
#[doc = "Field `EM41` reader - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use EM37_R as EM41_R;
#[doc = "Field `EM42` reader - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use EM37_R as EM42_R;
#[doc = "Field `EM47` reader - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use EM37_R as EM47_R;
#[doc = "Field `EM49` reader - CPU wakeup with event generation mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use EM37_R as EM49_R;
#[doc = "Field `EM50` reader - CPU wakeup with event generation mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use EM37_R as EM50_R;
#[doc = "Field `EM53` reader - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use EM37_R as EM53_R;
#[doc = "Field `EM38` writer - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use EM37_W as EM38_W;
#[doc = "Field `EM39` writer - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use EM37_W as EM39_W;
#[doc = "Field `EM40` writer - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use EM37_W as EM40_W;
#[doc = "Field `EM41` writer - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use EM37_W as EM41_W;
#[doc = "Field `EM42` writer - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use EM37_W as EM42_W;
#[doc = "Field `EM47` writer - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use EM37_W as EM47_W;
#[doc = "Field `EM49` writer - CPU wakeup with event generation mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use EM37_W as EM49_W;
#[doc = "Field `EM50` writer - CPU wakeup with event generation mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use EM37_W as EM50_W;
#[doc = "Field `EM53` writer - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use EM37_W as EM53_W;
impl R {
    #[doc = "Bit 5 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em37(&self) -> EM37_R {
        EM37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em38(&self) -> EM38_R {
        EM38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em39(&self) -> EM39_R {
        EM39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em40(&self) -> EM40_R {
        EM40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em41(&self) -> EM41_R {
        EM41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em42(&self) -> EM42_R {
        EM42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em47(&self) -> EM47_R {
        EM47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU wakeup with event generation mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em49(&self) -> EM49_R {
        EM49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU wakeup with event generation mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em50(&self) -> EM50_R {
        EM50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn em53(&self) -> EM53_R {
        EM53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em37(&mut self) -> EM37_W<EMR2rs> {
        EM37_W::new(self, 5)
    }
    #[doc = "Bit 6 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em38(&mut self) -> EM38_W<EMR2rs> {
        EM38_W::new(self, 6)
    }
    #[doc = "Bit 7 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em39(&mut self) -> EM39_W<EMR2rs> {
        EM39_W::new(self, 7)
    }
    #[doc = "Bit 8 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em40(&mut self) -> EM40_W<EMR2rs> {
        EM40_W::new(self, 8)
    }
    #[doc = "Bit 9 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em41(&mut self) -> EM41_W<EMR2rs> {
        EM41_W::new(self, 9)
    }
    #[doc = "Bit 10 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em42(&mut self) -> EM42_W<EMR2rs> {
        EM42_W::new(self, 10)
    }
    #[doc = "Bit 15 - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em47(&mut self) -> EM47_W<EMR2rs> {
        EM47_W::new(self, 15)
    }
    #[doc = "Bit 17 - CPU wakeup with event generation mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em49(&mut self) -> EM49_W<EMR2rs> {
        EM49_W::new(self, 17)
    }
    #[doc = "Bit 18 - CPU wakeup with event generation mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em50(&mut self) -> EM50_W<EMR2rs> {
        EM50_W::new(self, 18)
    }
    #[doc = "Bit 21 - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn em53(&mut self) -> EM53_W<EMR2rs> {
        EM53_W::new(self, 21)
    }
}
#[doc = "EXTI CPU wakeup with event mask register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMR2rs;
impl crate::RegisterSpec for EMR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr2::R`](R) reader structure"]
impl crate::Readable for EMR2rs {}
#[doc = "`write(|w| ..)` method takes [`emr2::W`](W) writer structure"]
impl crate::Writable for EMR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMR2 to value 0x00db_bfff"]
impl crate::Resettable for EMR2rs {
    const RESET_VALUE: u32 = 0x00db_bfff;
}
