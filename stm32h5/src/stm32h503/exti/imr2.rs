#[doc = "Register `IMR2` reader"]
pub type R = crate::R<IMR2rs>;
#[doc = "Register `IMR2` writer"]
pub type W = crate::W<IMR2rs>;
#[doc = "CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM37 {
    #[doc = "0: Interrupt request line is masked"]
    Masked = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    Unmasked = 1,
}
impl From<IM37> for bool {
    #[inline(always)]
    fn from(variant: IM37) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM37` reader - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM37_R = crate::BitReader<IM37>;
impl IM37_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM37 {
        match self.bits {
            false => IM37::Masked,
            true => IM37::Unmasked,
        }
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == IM37::Masked
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == IM37::Unmasked
    }
}
#[doc = "Field `IM37` writer - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM37_W<'a, REG> = crate::BitWriter<'a, REG, IM37>;
impl<'a, REG> IM37_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(IM37::Masked)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(IM37::Unmasked)
    }
}
#[doc = "Field `IM38` reader - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use IM37_R as IM38_R;
#[doc = "Field `IM39` reader - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use IM37_R as IM39_R;
#[doc = "Field `IM40` reader - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use IM37_R as IM40_R;
#[doc = "Field `IM41` reader - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use IM37_R as IM41_R;
#[doc = "Field `IM42` reader - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use IM37_R as IM42_R;
#[doc = "Field `IM47` reader - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use IM37_R as IM47_R;
#[doc = "Field `IM49` reader - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use IM37_R as IM49_R;
#[doc = "Field `IM50` reader - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use IM37_R as IM50_R;
#[doc = "Field `IM53` reader - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use IM37_R as IM53_R;
#[doc = "Field `IM38` writer - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use IM37_W as IM38_W;
#[doc = "Field `IM39` writer - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use IM37_W as IM39_W;
#[doc = "Field `IM40` writer - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use IM37_W as IM40_W;
#[doc = "Field `IM41` writer - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use IM37_W as IM41_W;
#[doc = "Field `IM42` writer - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use IM37_W as IM42_W;
#[doc = "Field `IM47` writer - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use IM37_W as IM47_W;
#[doc = "Field `IM49` writer - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use IM37_W as IM49_W;
#[doc = "Field `IM50` writer - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use IM37_W as IM50_W;
#[doc = "Field `IM53` writer - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub use IM37_W as IM53_W;
impl R {
    #[doc = "Bit 5 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn im37(&self) -> IM37_R {
        IM37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn im38(&self) -> IM38_R {
        IM38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn im39(&self) -> IM39_R {
        IM39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn im40(&self) -> IM40_R {
        IM40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn im41(&self) -> IM41_R {
        IM41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn im42(&self) -> IM42_R {
        IM42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn im47(&self) -> IM47_R {
        IM47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn im49(&self) -> IM49_R {
        IM49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn im50(&self) -> IM50_R {
        IM50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn im53(&self) -> IM53_R {
        IM53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn im37(&mut self) -> IM37_W<IMR2rs> {
        IM37_W::new(self, 5)
    }
    #[doc = "Bit 6 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn im38(&mut self) -> IM38_W<IMR2rs> {
        IM38_W::new(self, 6)
    }
    #[doc = "Bit 7 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn im39(&mut self) -> IM39_W<IMR2rs> {
        IM39_W::new(self, 7)
    }
    #[doc = "Bit 8 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn im40(&mut self) -> IM40_W<IMR2rs> {
        IM40_W::new(self, 8)
    }
    #[doc = "Bit 9 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn im41(&mut self) -> IM41_W<IMR2rs> {
        IM41_W::new(self, 9)
    }
    #[doc = "Bit 10 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn im42(&mut self) -> IM42_W<IMR2rs> {
        IM42_W::new(self, 10)
    }
    #[doc = "Bit 15 - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn im47(&mut self) -> IM47_W<IMR2rs> {
        IM47_W::new(self, 15)
    }
    #[doc = "Bit 17 - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn im49(&mut self) -> IM49_W<IMR2rs> {
        IM49_W::new(self, 17)
    }
    #[doc = "Bit 18 - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn im50(&mut self) -> IM50_W<IMR2rs> {
        IM50_W::new(self, 18)
    }
    #[doc = "Bit 21 - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    #[must_use]
    pub fn im53(&mut self) -> IM53_W<IMR2rs> {
        IM53_W::new(self, 21)
    }
}
#[doc = "EXTI CPU wakeup with interrupt mask register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMR2rs;
impl crate::RegisterSpec for IMR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr2::R`](R) reader structure"]
impl crate::Readable for IMR2rs {}
#[doc = "`write(|w| ..)` method takes [`imr2::W`](W) writer structure"]
impl crate::Writable for IMR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMR2 to value 0x00db_bfff"]
impl crate::Resettable for IMR2rs {
    const RESET_VALUE: u32 = 0x00db_bfff;
}
