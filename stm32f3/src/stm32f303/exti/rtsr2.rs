///Register `RTSR2` reader
pub type R = crate::R<RTSR2rs>;
///Register `RTSR2` writer
pub type W = crate::W<RTSR2rs>;
/**Rising trigger event configuration bit of line 32

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
///Field `TR32` reader - Rising trigger event configuration bit of line 32
pub type TR32_R = crate::BitReader<RISING_TRIGGER>;
impl TR32_R {
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
///Field `TR32` writer - Rising trigger event configuration bit of line 32
pub type TR32_W<'a, REG> = crate::BitWriter<'a, REG, RISING_TRIGGER>;
impl<'a, REG> TR32_W<'a, REG>
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
///Field `TR33` reader - Rising trigger event configuration bit of line 33
pub use TR32_R as TR33_R;
///Field `TR33` writer - Rising trigger event configuration bit of line 33
pub use TR32_W as TR33_W;
impl R {
    ///Bit 0 - Rising trigger event configuration bit of line 32
    #[inline(always)]
    pub fn tr32(&self) -> TR32_R {
        TR32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Rising trigger event configuration bit of line 33
    #[inline(always)]
    pub fn tr33(&self) -> TR33_R {
        TR33_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTSR2")
            .field("tr32", &self.tr32())
            .field("tr33", &self.tr33())
            .finish()
    }
}
impl W {
    ///Bit 0 - Rising trigger event configuration bit of line 32
    #[inline(always)]
    pub fn tr32(&mut self) -> TR32_W<'_, RTSR2rs> {
        TR32_W::new(self, 0)
    }
    ///Bit 1 - Rising trigger event configuration bit of line 33
    #[inline(always)]
    pub fn tr33(&mut self) -> TR33_W<'_, RTSR2rs> {
        TR33_W::new(self, 1)
    }
}
/**Rising Trigger selection register

You can [`read`](crate::Reg::read) this register and get [`rtsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F303.html#EXTI:RTSR2)*/
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
