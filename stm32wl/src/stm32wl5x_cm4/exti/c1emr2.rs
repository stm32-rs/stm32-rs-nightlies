///Register `C1EMR2` reader
pub type R = crate::R<C1EMR2rs>;
///Register `C1EMR2` writer
pub type W = crate::W<C1EMR2rs>;
/**Wake-up with event generation mask on event input 40

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
///Field `EM40` reader - Wake-up with event generation mask on event input 40
pub type EM40_R = crate::BitReader<EVENT_MASK>;
impl EM40_R {
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
///Field `EM40` writer - Wake-up with event generation mask on event input 40
pub type EM40_W<'a, REG> = crate::BitWriter<'a, REG, EVENT_MASK>;
impl<'a, REG> EM40_W<'a, REG>
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
///Field `EM41` reader - Wake-up with event generation mask on event input 41
pub use EM40_R as EM41_R;
///Field `EM41` writer - Wake-up with event generation mask on event input 41
pub use EM40_W as EM41_W;
impl R {
    ///Bit 8 - Wake-up with event generation mask on event input 40
    #[inline(always)]
    pub fn em40(&self) -> EM40_R {
        EM40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Wake-up with event generation mask on event input 41
    #[inline(always)]
    pub fn em41(&self) -> EM41_R {
        EM41_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1EMR2")
            .field("em40", &self.em40())
            .field("em41", &self.em41())
            .finish()
    }
}
impl W {
    ///Bit 8 - Wake-up with event generation mask on event input 40
    #[inline(always)]
    pub fn em40(&mut self) -> EM40_W<'_, C1EMR2rs> {
        EM40_W::new(self, 8)
    }
    ///Bit 9 - Wake-up with event generation mask on event input 41
    #[inline(always)]
    pub fn em41(&mut self) -> EM41_W<'_, C1EMR2rs> {
        EM41_W::new(self, 9)
    }
}
/**EXTI event mask register

You can [`read`](crate::Reg::read) this register and get [`c1emr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1emr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#EXTI:C1EMR2)*/
pub struct C1EMR2rs;
impl crate::RegisterSpec for C1EMR2rs {
    type Ux = u32;
}
///`read()` method returns [`c1emr2::R`](R) reader structure
impl crate::Readable for C1EMR2rs {}
///`write(|w| ..)` method takes [`c1emr2::W`](W) writer structure
impl crate::Writable for C1EMR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C1EMR2 to value 0
impl crate::Resettable for C1EMR2rs {}
