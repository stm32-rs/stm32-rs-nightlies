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
///Field `MR33` writer - Event mask on external/internal line 33
pub use MR32_W as MR33_W;
///Field `MR34` writer - Event mask on external/internal line 34
pub use MR32_W as MR34_W;
///Field `MR35` writer - Event mask on external/internal line 35
pub use MR32_W as MR35_W;
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMR2")
            .field("mr32", &self.mr32())
            .field("mr33", &self.mr33())
            .field("mr34", &self.mr34())
            .field("mr35", &self.mr35())
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
}
/**Event mask register

You can [`read`](crate::Reg::read) this register and get [`emr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F302.html#EXTI:EMR2)*/
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
