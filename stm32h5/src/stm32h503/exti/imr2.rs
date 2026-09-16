///Register `IMR2` reader
pub type R = crate::R<IMR2rs>;
///Register `IMR2` writer
pub type W = crate::W<IMR2rs>;
/**CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTERRUPT_MASK {
    ///0: Interrupt request line is masked
    Masked = 0,
    ///1: Interrupt request line is unmasked
    Unmasked = 1,
}
impl From<INTERRUPT_MASK> for bool {
    #[inline(always)]
    fn from(variant: INTERRUPT_MASK) -> Self {
        variant as u8 != 0
    }
}
///Field `IM37` reader - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type IM37_R = crate::BitReader<INTERRUPT_MASK>;
impl IM37_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INTERRUPT_MASK {
        match self.bits {
            false => INTERRUPT_MASK::Masked,
            true => INTERRUPT_MASK::Unmasked,
        }
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == INTERRUPT_MASK::Masked
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == INTERRUPT_MASK::Unmasked
    }
}
///Field `IM37` writer - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub type IM37_W<'a, REG> = crate::BitWriter<'a, REG, INTERRUPT_MASK>;
impl<'a, REG> IM37_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_MASK::Masked)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_MASK::Unmasked)
    }
}
///Field `IM38` reader - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub use IM37_R as IM38_R;
///Field `IM39` reader - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub use IM37_R as IM39_R;
///Field `IM40` reader - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub use IM37_R as IM40_R;
///Field `IM41` reader - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub use IM37_R as IM41_R;
///Field `IM42` reader - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub use IM37_R as IM42_R;
///Field `IM47` reader - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub use IM37_R as IM47_R;
///Field `IM49` reader - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub use IM37_R as IM49_R;
///Field `IM50` reader - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub use IM37_R as IM50_R;
///Field `IM53` reader - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub use IM37_R as IM53_R;
///Field `IM38` writer - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub use IM37_W as IM38_W;
///Field `IM39` writer - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub use IM37_W as IM39_W;
///Field `IM40` writer - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub use IM37_W as IM40_W;
///Field `IM41` writer - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub use IM37_W as IM41_W;
///Field `IM42` writer - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub use IM37_W as IM42_W;
///Field `IM47` writer - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub use IM37_W as IM47_W;
///Field `IM49` writer - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub use IM37_W as IM49_W;
///Field `IM50` writer - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub use IM37_W as IM50_W;
///Field `IM53` writer - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
pub use IM37_W as IM53_W;
impl R {
    ///Bit 5 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn im37(&self) -> IM37_R {
        IM37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn im38(&self) -> IM38_R {
        IM38_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn im39(&self) -> IM39_R {
        IM39_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn im40(&self) -> IM40_R {
        IM40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn im41(&self) -> IM41_R {
        IM41_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn im42(&self) -> IM42_R {
        IM42_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 15 - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn im47(&self) -> IM47_R {
        IM47_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn im49(&self) -> IM49_R {
        IM49_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn im50(&self) -> IM50_R {
        IM50_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 21 - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn im53(&self) -> IM53_R {
        IM53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMR2")
            .field("im37", &self.im37())
            .field("im38", &self.im38())
            .field("im39", &self.im39())
            .field("im40", &self.im40())
            .field("im41", &self.im41())
            .field("im42", &self.im42())
            .field("im47", &self.im47())
            .field("im49", &self.im49())
            .field("im50", &self.im50())
            .field("im53", &self.im53())
            .finish()
    }
}
impl W {
    ///Bit 5 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn im37(&mut self) -> IM37_W<'_, IMR2rs> {
        IM37_W::new(self, 5)
    }
    ///Bit 6 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn im38(&mut self) -> IM38_W<'_, IMR2rs> {
        IM38_W::new(self, 6)
    }
    ///Bit 7 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn im39(&mut self) -> IM39_W<'_, IMR2rs> {
        IM39_W::new(self, 7)
    }
    ///Bit 8 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn im40(&mut self) -> IM40_W<'_, IMR2rs> {
        IM40_W::new(self, 8)
    }
    ///Bit 9 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn im41(&mut self) -> IM41_W<'_, IMR2rs> {
        IM41_W::new(self, 9)
    }
    ///Bit 10 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn im42(&mut self) -> IM42_W<'_, IMR2rs> {
        IM42_W::new(self, 10)
    }
    ///Bit 15 - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn im47(&mut self) -> IM47_W<'_, IMR2rs> {
        IM47_W::new(self, 15)
    }
    ///Bit 17 - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn im49(&mut self) -> IM49_W<'_, IMR2rs> {
        IM49_W::new(self, 17)
    }
    ///Bit 18 - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn im50(&mut self) -> IM50_W<'_, IMR2rs> {
        IM50_W::new(self, 18)
    }
    ///Bit 21 - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.
    #[inline(always)]
    pub fn im53(&mut self) -> IM53_W<'_, IMR2rs> {
        IM53_W::new(self, 21)
    }
}
/**EXTI CPU wakeup with interrupt mask register 2

You can [`read`](crate::Reg::read) this register and get [`imr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#EXTI:IMR2)*/
pub struct IMR2rs;
impl crate::RegisterSpec for IMR2rs {
    type Ux = u32;
}
///`read()` method returns [`imr2::R`](R) reader structure
impl crate::Readable for IMR2rs {}
///`write(|w| ..)` method takes [`imr2::W`](W) writer structure
impl crate::Writable for IMR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IMR2 to value 0x00db_bfff
impl crate::Resettable for IMR2rs {
    const RESET_VALUE: u32 = 0x00db_bfff;
}
