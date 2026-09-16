///Register `FTSR2` reader
pub type R = crate::R<FTSR2rs>;
///Register `FTSR2` writer
pub type W = crate::W<FTSR2rs>;
/**Falling trigger event configuration bit of Configurable Event input x+32

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
///Field `TR49` reader - Falling trigger event configuration bit of Configurable Event input x+32
pub type TR49_R = crate::BitReader<FALLING_TRIGGER>;
impl TR49_R {
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
///Field `TR49` writer - Falling trigger event configuration bit of Configurable Event input x+32
pub type TR49_W<'a, REG> = crate::BitWriter<'a, REG, FALLING_TRIGGER>;
impl<'a, REG> TR49_W<'a, REG>
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
///Field `TR51` reader - Falling trigger event configuration bit of Configurable Event input x+32
pub use TR49_R as TR51_R;
///Field `TR51` writer - Falling trigger event configuration bit of Configurable Event input x+32
pub use TR49_W as TR51_W;
impl R {
    ///Bit 17 - Falling trigger event configuration bit of Configurable Event input x+32
    #[inline(always)]
    pub fn tr49(&self) -> TR49_R {
        TR49_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - Falling trigger event configuration bit of Configurable Event input x+32
    #[inline(always)]
    pub fn tr51(&self) -> TR51_R {
        TR51_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FTSR2")
            .field("tr49", &self.tr49())
            .field("tr51", &self.tr51())
            .finish()
    }
}
impl W {
    ///Bit 17 - Falling trigger event configuration bit of Configurable Event input x+32
    #[inline(always)]
    pub fn tr49(&mut self) -> TR49_W<'_, FTSR2rs> {
        TR49_W::new(self, 17)
    }
    ///Bit 19 - Falling trigger event configuration bit of Configurable Event input x+32
    #[inline(always)]
    pub fn tr51(&mut self) -> TR51_W<'_, FTSR2rs> {
        TR51_W::new(self, 19)
    }
}
/**EXTI falling trigger selection register

You can [`read`](crate::Reg::read) this register and get [`ftsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#EXTI:FTSR2)*/
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
