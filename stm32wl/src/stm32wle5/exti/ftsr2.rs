///Register `FTSR2` reader
pub type R = crate::R<FTSR2rs>;
///Register `FTSR2` writer
pub type W = crate::W<FTSR2rs>;
/**Falling trigger event configuration bit of Configurable Event input

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FALLING_TRIGGER {
    ///0: Falling edge trigger is disabled
    Disabled = 0,
    ///1: Falling edge trigger is enabled
    Enabled = 1,
}
impl From<FALLING_TRIGGER> for bool {
    #[inline(always)]
    fn from(variant: FALLING_TRIGGER) -> Self {
        variant as u8 != 0
    }
}
///Field `FT34` reader - Falling trigger event configuration bit of Configurable Event input
pub type FT34_R = crate::BitReader<FALLING_TRIGGER>;
impl FT34_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FALLING_TRIGGER {
        match self.bits {
            false => FALLING_TRIGGER::Disabled,
            true => FALLING_TRIGGER::Enabled,
        }
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FALLING_TRIGGER::Disabled
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FALLING_TRIGGER::Enabled
    }
}
///Field `FT34` writer - Falling trigger event configuration bit of Configurable Event input
pub type FT34_W<'a, REG> = crate::BitWriter<'a, REG, FALLING_TRIGGER>;
impl<'a, REG> FT34_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FALLING_TRIGGER::Disabled)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FALLING_TRIGGER::Enabled)
    }
}
///Field `FT45` reader - Falling trigger event configuration bit of Configurable Event input
pub use FT34_R as FT45_R;
///Field `FT45` writer - Falling trigger event configuration bit of Configurable Event input
pub use FT34_W as FT45_W;
impl R {
    ///Bit 2 - Falling trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn ft34(&self) -> FT34_R {
        FT34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 13 - Falling trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn ft45(&self) -> FT45_R {
        FT45_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FTSR2")
            .field("ft34", &self.ft34())
            .field("ft45", &self.ft45())
            .finish()
    }
}
impl W {
    ///Bit 2 - Falling trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn ft34(&mut self) -> FT34_W<'_, FTSR2rs> {
        FT34_W::new(self, 2)
    }
    ///Bit 13 - Falling trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn ft45(&mut self) -> FT45_W<'_, FTSR2rs> {
        FT45_W::new(self, 13)
    }
}
/**falling trigger selection register

You can [`read`](crate::Reg::read) this register and get [`ftsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#EXTI:FTSR2)*/
pub struct FTSR2rs;
impl crate::RegisterSpec for FTSR2rs {
    type Ux = u32;
}
///`read()` method returns [`ftsr2::R`](R) reader structure
impl crate::Readable for FTSR2rs {}
///`write(|w| ..)` method takes [`ftsr2::W`](W) writer structure
impl crate::Writable for FTSR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FTSR2 to value 0
impl crate::Resettable for FTSR2rs {}
