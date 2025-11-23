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
///Field `EM32` reader - Event mask on external/internal line 32
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
///Field `EM32` writer - Event mask on external/internal line 32
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
///Field `EM33` reader - Event mask on external/internal line 33
pub use EM32_R as EM33_R;
///Field `EM34` reader - Event mask on external/internal line 34
pub use EM32_R as EM34_R;
///Field `EM35` reader - Event mask on external/internal line 35
pub use EM32_R as EM35_R;
///Field `EM36` reader - Event mask on external/internal line 36
pub use EM32_R as EM36_R;
///Field `EM37` reader - Event mask on external/internal line 37
pub use EM32_R as EM37_R;
///Field `EM40` reader - Event mask on external/internal line 40
pub use EM32_R as EM40_R;
///Field `EM41` reader - Event mask on external/internal line 41
pub use EM32_R as EM41_R;
///Field `EM42` reader - Event mask on external/internal line 42
pub use EM32_R as EM42_R;
///Field `EM43` reader - Event mask on external/internal line 43
pub use EM32_R as EM43_R;
///Field `EM33` writer - Event mask on external/internal line 33
pub use EM32_W as EM33_W;
///Field `EM34` writer - Event mask on external/internal line 34
pub use EM32_W as EM34_W;
///Field `EM35` writer - Event mask on external/internal line 35
pub use EM32_W as EM35_W;
///Field `EM36` writer - Event mask on external/internal line 36
pub use EM32_W as EM36_W;
///Field `EM37` writer - Event mask on external/internal line 37
pub use EM32_W as EM37_W;
///Field `EM40` writer - Event mask on external/internal line 40
pub use EM32_W as EM40_W;
///Field `EM41` writer - Event mask on external/internal line 41
pub use EM32_W as EM41_W;
///Field `EM42` writer - Event mask on external/internal line 42
pub use EM32_W as EM42_W;
///Field `EM43` writer - Event mask on external/internal line 43
pub use EM32_W as EM43_W;
impl R {
    ///Bit 0 - Event mask on external/internal line 32
    #[inline(always)]
    pub fn em32(&self) -> EM32_R {
        EM32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Event mask on external/internal line 33
    #[inline(always)]
    pub fn em33(&self) -> EM33_R {
        EM33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Event mask on external/internal line 34
    #[inline(always)]
    pub fn em34(&self) -> EM34_R {
        EM34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Event mask on external/internal line 35
    #[inline(always)]
    pub fn em35(&self) -> EM35_R {
        EM35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Event mask on external/internal line 36
    #[inline(always)]
    pub fn em36(&self) -> EM36_R {
        EM36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Event mask on external/internal line 37
    #[inline(always)]
    pub fn em37(&self) -> EM37_R {
        EM37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Event mask on external/internal line 40
    #[inline(always)]
    pub fn em40(&self) -> EM40_R {
        EM40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Event mask on external/internal line 41
    #[inline(always)]
    pub fn em41(&self) -> EM41_R {
        EM41_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Event mask on external/internal line 42
    #[inline(always)]
    pub fn em42(&self) -> EM42_R {
        EM42_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Event mask on external/internal line 43
    #[inline(always)]
    pub fn em43(&self) -> EM43_R {
        EM43_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMR2")
            .field("em32", &self.em32())
            .field("em33", &self.em33())
            .field("em34", &self.em34())
            .field("em35", &self.em35())
            .field("em36", &self.em36())
            .field("em37", &self.em37())
            .field("em40", &self.em40())
            .field("em41", &self.em41())
            .field("em42", &self.em42())
            .field("em43", &self.em43())
            .finish()
    }
}
impl W {
    ///Bit 0 - Event mask on external/internal line 32
    #[inline(always)]
    pub fn em32(&mut self) -> EM32_W<'_, EMR2rs> {
        EM32_W::new(self, 0)
    }
    ///Bit 1 - Event mask on external/internal line 33
    #[inline(always)]
    pub fn em33(&mut self) -> EM33_W<'_, EMR2rs> {
        EM33_W::new(self, 1)
    }
    ///Bit 2 - Event mask on external/internal line 34
    #[inline(always)]
    pub fn em34(&mut self) -> EM34_W<'_, EMR2rs> {
        EM34_W::new(self, 2)
    }
    ///Bit 3 - Event mask on external/internal line 35
    #[inline(always)]
    pub fn em35(&mut self) -> EM35_W<'_, EMR2rs> {
        EM35_W::new(self, 3)
    }
    ///Bit 4 - Event mask on external/internal line 36
    #[inline(always)]
    pub fn em36(&mut self) -> EM36_W<'_, EMR2rs> {
        EM36_W::new(self, 4)
    }
    ///Bit 5 - Event mask on external/internal line 37
    #[inline(always)]
    pub fn em37(&mut self) -> EM37_W<'_, EMR2rs> {
        EM37_W::new(self, 5)
    }
    ///Bit 8 - Event mask on external/internal line 40
    #[inline(always)]
    pub fn em40(&mut self) -> EM40_W<'_, EMR2rs> {
        EM40_W::new(self, 8)
    }
    ///Bit 9 - Event mask on external/internal line 41
    #[inline(always)]
    pub fn em41(&mut self) -> EM41_W<'_, EMR2rs> {
        EM41_W::new(self, 9)
    }
    ///Bit 10 - Event mask on external/internal line 42
    #[inline(always)]
    pub fn em42(&mut self) -> EM42_W<'_, EMR2rs> {
        EM42_W::new(self, 10)
    }
    ///Bit 11 - Event mask on external/internal line 43
    #[inline(always)]
    pub fn em43(&mut self) -> EM43_W<'_, EMR2rs> {
        EM43_W::new(self, 11)
    }
}
/**Event mask register

You can [`read`](crate::Reg::read) this register and get [`emr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G473.html#EXTI:EMR2)*/
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
