///Register `EMR2` reader
pub type R = crate::R<EMR2rs>;
///Register `EMR2` writer
pub type W = crate::W<EMR2rs>;
/**CPU wakeup with event mask on event input

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENT_MASK {
    ///0: Event request line is masked
    Masked = 0,
    ///1: Event request line is unmasked
    Unmasked = 1,
}
impl From<EVENT_MASK> for bool {
    #[inline(always)]
    fn from(variant: EVENT_MASK) -> Self {
        variant as u8 != 0
    }
}
///Field `EM32` reader - CPU wakeup with event mask on event input
pub type EM32_R = crate::BitReader<EVENT_MASK>;
impl EM32_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EVENT_MASK {
        match self.bits {
            false => EVENT_MASK::Masked,
            true => EVENT_MASK::Unmasked,
        }
    }
    ///Event request line is masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == EVENT_MASK::Masked
    }
    ///Event request line is unmasked
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == EVENT_MASK::Unmasked
    }
}
///Field `EM32` writer - CPU wakeup with event mask on event input
pub type EM32_W<'a, REG> = crate::BitWriter<'a, REG, EVENT_MASK>;
impl<'a, REG> EM32_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Event request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(EVENT_MASK::Masked)
    }
    ///Event request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(EVENT_MASK::Unmasked)
    }
}
///Field `EM33` reader - CPU wakeup with event mask on event input
pub use EM32_R as EM33_R;
///Field `EM34` reader - CPU wakeup with event mask on event input
pub use EM32_R as EM34_R;
///Field `EM35` reader - CPU wakeup with event mask on event input
pub use EM32_R as EM35_R;
///Field `EM33` writer - CPU wakeup with event mask on event input
pub use EM32_W as EM33_W;
///Field `EM34` writer - CPU wakeup with event mask on event input
pub use EM32_W as EM34_W;
///Field `EM35` writer - CPU wakeup with event mask on event input
pub use EM32_W as EM35_W;
impl R {
    ///Bit 0 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em32(&self) -> EM32_R {
        EM32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em33(&self) -> EM33_R {
        EM33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em34(&self) -> EM34_R {
        EM34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em35(&self) -> EM35_R {
        EM35_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMR2")
            .field("em32", &self.em32())
            .field("em33", &self.em33())
            .field("em34", &self.em34())
            .field("em35", &self.em35())
            .finish()
    }
}
impl W {
    ///Bit 0 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em32(&mut self) -> EM32_W<'_, EMR2rs> {
        EM32_W::new(self, 0)
    }
    ///Bit 1 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em33(&mut self) -> EM33_W<'_, EMR2rs> {
        EM33_W::new(self, 1)
    }
    ///Bit 2 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em34(&mut self) -> EM34_W<'_, EMR2rs> {
        EM34_W::new(self, 2)
    }
    ///Bit 3 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em35(&mut self) -> EM35_W<'_, EMR2rs> {
        EM35_W::new(self, 3)
    }
}
/**EXTI CPU wakeup with event mask register

You can [`read`](crate::Reg::read) this register and get [`emr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0C1.html#EXTI:EMR2)*/
pub struct EMR2rs;
impl crate::RegisterSpec for EMR2rs {
    type Ux = u32;
}
///`read()` method returns [`emr2::R`](R) reader structure
impl crate::Readable for EMR2rs {}
///`write(|w| ..)` method takes [`emr2::W`](W) writer structure
impl crate::Writable for EMR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EMR2 to value 0
impl crate::Resettable for EMR2rs {}
