///Register `FTSR2` reader
pub type R = crate::R<FTSR2rs>;
///Register `FTSR2` writer
pub type W = crate::W<FTSR2rs>;
/**Falling trigger event configuration bit of configurable line 34

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT2 {
    ///0: Falling edge trigger is disabled
    Disabled = 0,
    ///1: Falling edge trigger is enabled
    Enabled = 1,
}
impl From<FT2> for bool {
    #[inline(always)]
    fn from(variant: FT2) -> Self {
        variant as u8 != 0
    }
}
///Field `FT2` reader - Falling trigger event configuration bit of configurable line 34
pub type FT2_R = crate::BitReader<FT2>;
impl FT2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FT2 {
        match self.bits {
            false => FT2::Disabled,
            true => FT2::Enabled,
        }
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FT2::Disabled
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FT2::Enabled
    }
}
///Field `FT2` writer - Falling trigger event configuration bit of configurable line 34
pub type FT2_W<'a, REG> = crate::BitWriter<'a, REG, FT2>;
impl<'a, REG> FT2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FT2::Disabled)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FT2::Enabled)
    }
}
impl R {
    ///Bit 2 - Falling trigger event configuration bit of configurable line 34
    #[inline(always)]
    pub fn ft2(&self) -> FT2_R {
        FT2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FTSR2").field("ft2", &self.ft2()).finish()
    }
}
impl W {
    ///Bit 2 - Falling trigger event configuration bit of configurable line 34
    #[inline(always)]
    #[must_use]
    pub fn ft2(&mut self) -> FT2_W<FTSR2rs> {
        FT2_W::new(self, 2)
    }
}
/**EXTI falling trigger selection register 2

You can [`read`](crate::Reg::read) this register and get [`ftsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B1.html#EXTI:FTSR2)*/
pub struct FTSR2rs;
impl crate::RegisterSpec for FTSR2rs {
    type Ux = u32;
}
///`read()` method returns [`ftsr2::R`](R) reader structure
impl crate::Readable for FTSR2rs {}
///`write(|w| ..)` method takes [`ftsr2::W`](W) writer structure
impl crate::Writable for FTSR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FTSR2 to value 0
impl crate::Resettable for FTSR2rs {
    const RESET_VALUE: u32 = 0;
}
