///Register `RTSR2` reader
pub type R = crate::R<RTSR2rs>;
///Register `RTSR2` writer
pub type W = crate::W<RTSR2rs>;
/**rising trigger event configuration bit of configurable event input 34

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RISING_TRIGGER {
    ///0: Rising edge trigger is disabled
    Disabled = 0,
    ///1: Rising edge trigger is enabled
    Enabled = 1,
}
impl From<RISING_TRIGGER> for bool {
    #[inline(always)]
    fn from(variant: RISING_TRIGGER) -> Self {
        variant as u8 != 0
    }
}
///Field `RT34` reader - rising trigger event configuration bit of configurable event input 34
pub type RT34_R = crate::BitReader<RISING_TRIGGER>;
impl RT34_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RISING_TRIGGER {
        match self.bits {
            false => RISING_TRIGGER::Disabled,
            true => RISING_TRIGGER::Enabled,
        }
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RISING_TRIGGER::Disabled
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RISING_TRIGGER::Enabled
    }
}
///Field `RT34` writer - rising trigger event configuration bit of configurable event input 34
pub type RT34_W<'a, REG> = crate::BitWriter<'a, REG, RISING_TRIGGER>;
impl<'a, REG> RT34_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RISING_TRIGGER::Disabled)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RISING_TRIGGER::Enabled)
    }
}
///Field `RT40` reader - rising trigger event configuration bit of configurable event input 40
pub use RT34_R as RT40_R;
///Field `RT41` reader - rising trigger event configuration bit of configurable event input 41
pub use RT34_R as RT41_R;
///Field `RT45` reader - rising trigger event configuration bit of configurable event input 45 Note: The configurable event inputs are edge triggered. No glitch must be generated on these inputs. If a rising edge on the configurable event input occurs while writing to the register, the associated pending bit is not set. Note: Rising and falling edge triggers can be set for the same configurable event input. In this case, both edges generate a trigger.
pub use RT34_R as RT45_R;
///Field `RT40` writer - rising trigger event configuration bit of configurable event input 40
pub use RT34_W as RT40_W;
///Field `RT41` writer - rising trigger event configuration bit of configurable event input 41
pub use RT34_W as RT41_W;
///Field `RT45` writer - rising trigger event configuration bit of configurable event input 45 Note: The configurable event inputs are edge triggered. No glitch must be generated on these inputs. If a rising edge on the configurable event input occurs while writing to the register, the associated pending bit is not set. Note: Rising and falling edge triggers can be set for the same configurable event input. In this case, both edges generate a trigger.
pub use RT34_W as RT45_W;
impl R {
    ///Bit 2 - rising trigger event configuration bit of configurable event input 34
    #[inline(always)]
    pub fn rt34(&self) -> RT34_R {
        RT34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - rising trigger event configuration bit of configurable event input 40
    #[inline(always)]
    pub fn rt40(&self) -> RT40_R {
        RT40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - rising trigger event configuration bit of configurable event input 41
    #[inline(always)]
    pub fn rt41(&self) -> RT41_R {
        RT41_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 13 - rising trigger event configuration bit of configurable event input 45 Note: The configurable event inputs are edge triggered. No glitch must be generated on these inputs. If a rising edge on the configurable event input occurs while writing to the register, the associated pending bit is not set. Note: Rising and falling edge triggers can be set for the same configurable event input. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt45(&self) -> RT45_R {
        RT45_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTSR2")
            .field("rt34", &self.rt34())
            .field("rt40", &self.rt40())
            .field("rt41", &self.rt41())
            .field("rt45", &self.rt45())
            .finish()
    }
}
impl W {
    ///Bit 2 - rising trigger event configuration bit of configurable event input 34
    #[inline(always)]
    pub fn rt34(&mut self) -> RT34_W<'_, RTSR2rs> {
        RT34_W::new(self, 2)
    }
    ///Bit 8 - rising trigger event configuration bit of configurable event input 40
    #[inline(always)]
    pub fn rt40(&mut self) -> RT40_W<'_, RTSR2rs> {
        RT40_W::new(self, 8)
    }
    ///Bit 9 - rising trigger event configuration bit of configurable event input 41
    #[inline(always)]
    pub fn rt41(&mut self) -> RT41_W<'_, RTSR2rs> {
        RT41_W::new(self, 9)
    }
    ///Bit 13 - rising trigger event configuration bit of configurable event input 45 Note: The configurable event inputs are edge triggered. No glitch must be generated on these inputs. If a rising edge on the configurable event input occurs while writing to the register, the associated pending bit is not set. Note: Rising and falling edge triggers can be set for the same configurable event input. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt45(&mut self) -> RT45_W<'_, RTSR2rs> {
        RT45_W::new(self, 13)
    }
}
/**EXTI rising trigger selection register

You can [`read`](crate::Reg::read) this register and get [`rtsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#EXTI:RTSR2)*/
pub struct RTSR2rs;
impl crate::RegisterSpec for RTSR2rs {
    type Ux = u32;
}
///`read()` method returns [`rtsr2::R`](R) reader structure
impl crate::Readable for RTSR2rs {}
///`write(|w| ..)` method takes [`rtsr2::W`](W) writer structure
impl crate::Writable for RTSR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RTSR2 to value 0
impl crate::Resettable for RTSR2rs {}
