///Register `FTSR3` reader
pub type R = crate::R<FTSR3rs>;
///Register `FTSR3` writer
pub type W = crate::W<FTSR3rs>;
/**Falling trigger event configuration bit of Configurable Event input x+64

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
///Field `TR82` reader - Falling trigger event configuration bit of Configurable Event input x+64
pub type TR82_R = crate::BitReader<FALLING_TRIGGER>;
impl TR82_R {
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
///Field `TR82` writer - Falling trigger event configuration bit of Configurable Event input x+64
pub type TR82_W<'a, REG> = crate::BitWriter<'a, REG, FALLING_TRIGGER>;
impl<'a, REG> TR82_W<'a, REG>
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
///Field `TR84` reader - Falling trigger event configuration bit of Configurable Event input x+64
pub use TR82_R as TR84_R;
///Field `TR85` reader - Falling trigger event configuration bit of Configurable Event input x+64
pub use TR82_R as TR85_R;
///Field `TR86` reader - Falling trigger event configuration bit of Configurable Event input x+64
pub use TR82_R as TR86_R;
///Field `TR84` writer - Falling trigger event configuration bit of Configurable Event input x+64
pub use TR82_W as TR84_W;
///Field `TR85` writer - Falling trigger event configuration bit of Configurable Event input x+64
pub use TR82_W as TR85_W;
///Field `TR86` writer - Falling trigger event configuration bit of Configurable Event input x+64
pub use TR82_W as TR86_W;
impl R {
    ///Bit 18 - Falling trigger event configuration bit of Configurable Event input x+64
    #[inline(always)]
    pub fn tr82(&self) -> TR82_R {
        TR82_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - Falling trigger event configuration bit of Configurable Event input x+64
    #[inline(always)]
    pub fn tr84(&self) -> TR84_R {
        TR84_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Falling trigger event configuration bit of Configurable Event input x+64
    #[inline(always)]
    pub fn tr85(&self) -> TR85_R {
        TR85_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Falling trigger event configuration bit of Configurable Event input x+64
    #[inline(always)]
    pub fn tr86(&self) -> TR86_R {
        TR86_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FTSR3")
            .field("tr82", &self.tr82())
            .field("tr84", &self.tr84())
            .field("tr85", &self.tr85())
            .field("tr86", &self.tr86())
            .finish()
    }
}
impl W {
    ///Bit 18 - Falling trigger event configuration bit of Configurable Event input x+64
    #[inline(always)]
    pub fn tr82(&mut self) -> TR82_W<'_, FTSR3rs> {
        TR82_W::new(self, 18)
    }
    ///Bit 20 - Falling trigger event configuration bit of Configurable Event input x+64
    #[inline(always)]
    pub fn tr84(&mut self) -> TR84_W<'_, FTSR3rs> {
        TR84_W::new(self, 20)
    }
    ///Bit 21 - Falling trigger event configuration bit of Configurable Event input x+64
    #[inline(always)]
    pub fn tr85(&mut self) -> TR85_W<'_, FTSR3rs> {
        TR85_W::new(self, 21)
    }
    ///Bit 22 - Falling trigger event configuration bit of Configurable Event input x+64
    #[inline(always)]
    pub fn tr86(&mut self) -> TR86_W<'_, FTSR3rs> {
        TR86_W::new(self, 22)
    }
}
/**EXTI falling trigger selection register

You can [`read`](crate::Reg::read) this register and get [`ftsr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#EXTI:FTSR3)*/
pub struct FTSR3rs;
impl crate::RegisterSpec for FTSR3rs {
    type Ux = u32;
}
///`read()` method returns [`ftsr3::R`](R) reader structure
impl crate::Readable for FTSR3rs {}
///`write(|w| ..)` method takes [`ftsr3::W`](W) writer structure
impl crate::Writable for FTSR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FTSR3 to value 0
impl crate::Resettable for FTSR3rs {}
