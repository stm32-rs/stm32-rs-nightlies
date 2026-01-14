///Register `EMR2` reader
pub type R = crate::R<EMR2rs>;
///Register `EMR2` writer
pub type W = crate::W<EMR2rs>;
/**Event mask on external/internal line 32

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
///Field `MR32` reader - Event mask on external/internal line 32
pub type MR32_R = crate::BitReader<EVENT_MASK>;
impl MR32_R {
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
///Field `MR32` writer - Event mask on external/internal line 32
pub type MR32_W<'a, REG> = crate::BitWriter<'a, REG, EVENT_MASK>;
impl<'a, REG> MR32_W<'a, REG>
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
///Field `MR33` reader - Event mask on external/internal line 33
pub use MR32_R as MR33_R;
///Field `MR34` reader - Event mask on external/internal line 34
pub use MR32_R as MR34_R;
///Field `MR35` reader - Event mask on external/internal line 35
pub use MR32_R as MR35_R;
///Field `MR36` reader - Event mask on external/internal line 36
pub use MR32_R as MR36_R;
///Field `MR37` reader - Event mask on external/internal line 37
pub use MR32_R as MR37_R;
///Field `MR38` reader - Event mask on external/internal line 38
pub use MR32_R as MR38_R;
///Field `MR39` reader - Event mask on external/internal line 39
pub use MR32_R as MR39_R;
///Field `MR33` writer - Event mask on external/internal line 33
pub use MR32_W as MR33_W;
///Field `MR34` writer - Event mask on external/internal line 34
pub use MR32_W as MR34_W;
///Field `MR35` writer - Event mask on external/internal line 35
pub use MR32_W as MR35_W;
///Field `MR36` writer - Event mask on external/internal line 36
pub use MR32_W as MR36_W;
///Field `MR37` writer - Event mask on external/internal line 37
pub use MR32_W as MR37_W;
///Field `MR38` writer - Event mask on external/internal line 38
pub use MR32_W as MR38_W;
///Field `MR39` writer - Event mask on external/internal line 39
pub use MR32_W as MR39_W;
impl R {
    ///Bit 0 - Event mask on external/internal line 32
    #[inline(always)]
    pub fn mr32(&self) -> MR32_R {
        MR32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Event mask on external/internal line 33
    #[inline(always)]
    pub fn mr33(&self) -> MR33_R {
        MR33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Event mask on external/internal line 34
    #[inline(always)]
    pub fn mr34(&self) -> MR34_R {
        MR34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Event mask on external/internal line 35
    #[inline(always)]
    pub fn mr35(&self) -> MR35_R {
        MR35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Event mask on external/internal line 36
    #[inline(always)]
    pub fn mr36(&self) -> MR36_R {
        MR36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Event mask on external/internal line 37
    #[inline(always)]
    pub fn mr37(&self) -> MR37_R {
        MR37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Event mask on external/internal line 38
    #[inline(always)]
    pub fn mr38(&self) -> MR38_R {
        MR38_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Event mask on external/internal line 39
    #[inline(always)]
    pub fn mr39(&self) -> MR39_R {
        MR39_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMR2")
            .field("mr32", &self.mr32())
            .field("mr33", &self.mr33())
            .field("mr34", &self.mr34())
            .field("mr35", &self.mr35())
            .field("mr36", &self.mr36())
            .field("mr37", &self.mr37())
            .field("mr38", &self.mr38())
            .field("mr39", &self.mr39())
            .finish()
    }
}
impl W {
    ///Bit 0 - Event mask on external/internal line 32
    #[inline(always)]
    pub fn mr32(&mut self) -> MR32_W<'_, EMR2rs> {
        MR32_W::new(self, 0)
    }
    ///Bit 1 - Event mask on external/internal line 33
    #[inline(always)]
    pub fn mr33(&mut self) -> MR33_W<'_, EMR2rs> {
        MR33_W::new(self, 1)
    }
    ///Bit 2 - Event mask on external/internal line 34
    #[inline(always)]
    pub fn mr34(&mut self) -> MR34_W<'_, EMR2rs> {
        MR34_W::new(self, 2)
    }
    ///Bit 3 - Event mask on external/internal line 35
    #[inline(always)]
    pub fn mr35(&mut self) -> MR35_W<'_, EMR2rs> {
        MR35_W::new(self, 3)
    }
    ///Bit 4 - Event mask on external/internal line 36
    #[inline(always)]
    pub fn mr36(&mut self) -> MR36_W<'_, EMR2rs> {
        MR36_W::new(self, 4)
    }
    ///Bit 5 - Event mask on external/internal line 37
    #[inline(always)]
    pub fn mr37(&mut self) -> MR37_W<'_, EMR2rs> {
        MR37_W::new(self, 5)
    }
    ///Bit 6 - Event mask on external/internal line 38
    #[inline(always)]
    pub fn mr38(&mut self) -> MR38_W<'_, EMR2rs> {
        MR38_W::new(self, 6)
    }
    ///Bit 7 - Event mask on external/internal line 39
    #[inline(always)]
    pub fn mr39(&mut self) -> MR39_W<'_, EMR2rs> {
        MR39_W::new(self, 7)
    }
}
/**Event mask register

You can [`read`](crate::Reg::read) this register and get [`emr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x5.html#EXTI:EMR2)*/
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
