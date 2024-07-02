///Register `C2EMR2` reader
pub type R = crate::R<C2EMR2rs>;
///Register `C2EMR2` writer
pub type W = crate::W<C2EMR2rs>;
/**Wakeup with event generation Mask on Event input

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM40 {
    ///0: Interrupt request line is masked
    Masked = 0,
    ///1: Interrupt request line is unmasked
    Unmasked = 1,
}
impl From<EM40> for bool {
    #[inline(always)]
    fn from(variant: EM40) -> Self {
        variant as u8 != 0
    }
}
///Field `EM40` reader - Wakeup with event generation Mask on Event input
pub type EM40_R = crate::BitReader<EM40>;
impl EM40_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EM40 {
        match self.bits {
            false => EM40::Masked,
            true => EM40::Unmasked,
        }
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == EM40::Masked
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == EM40::Unmasked
    }
}
///Field `EM40` writer - Wakeup with event generation Mask on Event input
pub type EM40_W<'a, REG> = crate::BitWriter<'a, REG, EM40>;
impl<'a, REG> EM40_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(EM40::Masked)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(EM40::Unmasked)
    }
}
///Field `EM41` reader - Wakeup with event generation Mask on Event input
pub use EM40_R as EM41_R;
///Field `EM41` writer - Wakeup with event generation Mask on Event input
pub use EM40_W as EM41_W;
impl R {
    ///Bit 8 - Wakeup with event generation Mask on Event input
    #[inline(always)]
    pub fn em40(&self) -> EM40_R {
        EM40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Wakeup with event generation Mask on Event input
    #[inline(always)]
    pub fn em41(&self) -> EM41_R {
        EM41_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2EMR2")
            .field("em40", &self.em40())
            .field("em41", &self.em41())
            .finish()
    }
}
impl W {
    ///Bit 8 - Wakeup with event generation Mask on Event input
    #[inline(always)]
    #[must_use]
    pub fn em40(&mut self) -> EM40_W<C2EMR2rs> {
        EM40_W::new(self, 8)
    }
    ///Bit 9 - Wakeup with event generation Mask on Event input
    #[inline(always)]
    #[must_use]
    pub fn em41(&mut self) -> EM41_W<C2EMR2rs> {
        EM41_W::new(self, 9)
    }
}
/**wakeup with event mask register

You can [`read`](crate::Reg::read) this register and get [`c2emr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2emr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#EXTI:C2EMR2)*/
pub struct C2EMR2rs;
impl crate::RegisterSpec for C2EMR2rs {
    type Ux = u32;
}
///`read()` method returns [`c2emr2::R`](R) reader structure
impl crate::Readable for C2EMR2rs {}
///`write(|w| ..)` method takes [`c2emr2::W`](W) writer structure
impl crate::Writable for C2EMR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets C2EMR2 to value 0
impl crate::Resettable for C2EMR2rs {
    const RESET_VALUE: u32 = 0;
}
