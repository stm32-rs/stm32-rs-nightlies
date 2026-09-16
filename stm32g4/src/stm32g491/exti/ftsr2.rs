///Register `FTSR2` reader
pub type R = crate::R<FTSR2rs>;
///Register `FTSR2` writer
pub type W = crate::W<FTSR2rs>;
/**Falling trigger event configuration of line 32

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
///Field `FT32` reader - Falling trigger event configuration of line 32
pub type FT32_R = crate::BitReader<FALLING_TRIGGER>;
impl FT32_R {
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
///Field `FT32` writer - Falling trigger event configuration of line 32
pub type FT32_W<'a, REG> = crate::BitWriter<'a, REG, FALLING_TRIGGER>;
impl<'a, REG> FT32_W<'a, REG>
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
///Field `FT33` reader - Falling trigger event configuration of line 33
pub use FT32_R as FT33_R;
///Field `FT40` reader - Falling trigger event configuration of line 40
pub use FT32_R as FT40_R;
///Field `FT41` reader - Falling trigger event configuration of line 41
pub use FT32_R as FT41_R;
///Field `FT33` writer - Falling trigger event configuration of line 33
pub use FT32_W as FT33_W;
///Field `FT40` writer - Falling trigger event configuration of line 40
pub use FT32_W as FT40_W;
///Field `FT41` writer - Falling trigger event configuration of line 41
pub use FT32_W as FT41_W;
impl R {
    ///Bit 0 - Falling trigger event configuration of line 32
    #[inline(always)]
    pub fn ft32(&self) -> FT32_R {
        FT32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Falling trigger event configuration of line 33
    #[inline(always)]
    pub fn ft33(&self) -> FT33_R {
        FT33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Falling trigger event configuration of line 40
    #[inline(always)]
    pub fn ft40(&self) -> FT40_R {
        FT40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Falling trigger event configuration of line 41
    #[inline(always)]
    pub fn ft41(&self) -> FT41_R {
        FT41_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FTSR2")
            .field("ft32", &self.ft32())
            .field("ft33", &self.ft33())
            .field("ft40", &self.ft40())
            .field("ft41", &self.ft41())
            .finish()
    }
}
impl W {
    ///Bit 0 - Falling trigger event configuration of line 32
    #[inline(always)]
    pub fn ft32(&mut self) -> FT32_W<'_, FTSR2rs> {
        FT32_W::new(self, 0)
    }
    ///Bit 1 - Falling trigger event configuration of line 33
    #[inline(always)]
    pub fn ft33(&mut self) -> FT33_W<'_, FTSR2rs> {
        FT33_W::new(self, 1)
    }
    ///Bit 8 - Falling trigger event configuration of line 40
    #[inline(always)]
    pub fn ft40(&mut self) -> FT40_W<'_, FTSR2rs> {
        FT40_W::new(self, 8)
    }
    ///Bit 9 - Falling trigger event configuration of line 41
    #[inline(always)]
    pub fn ft41(&mut self) -> FT41_W<'_, FTSR2rs> {
        FT41_W::new(self, 9)
    }
}
/**Falling Trigger selection register

You can [`read`](crate::Reg::read) this register and get [`ftsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G491.html#EXTI:FTSR2)*/
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
