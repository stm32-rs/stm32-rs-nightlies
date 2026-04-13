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
///Field `RT32` reader - Rising trigger event configuration bit of line 32
pub type RT32_R = crate::BitReader<RISING_TRIGGER>;
impl RT32_R {
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
///Field `RT32` writer - Rising trigger event configuration bit of line 32
pub type RT32_W<'a, REG> = crate::BitWriter<'a, REG, RISING_TRIGGER>;
impl<'a, REG> RT32_W<'a, REG>
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
///Field `RT33` reader - Rising trigger event configuration bit of line 32
pub use RT32_R as RT33_R;
///Field `RT40` reader - Rising trigger event configuration bit of line 40
pub use RT32_R as RT40_R;
///Field `RT41` reader - Rising trigger event configuration bit of line 41
pub use RT32_R as RT41_R;
///Field `RT33` writer - Rising trigger event configuration bit of line 32
pub use RT32_W as RT33_W;
///Field `RT40` writer - Rising trigger event configuration bit of line 40
pub use RT32_W as RT40_W;
///Field `RT41` writer - Rising trigger event configuration bit of line 41
pub use RT32_W as RT41_W;
impl R {
    ///Bit 0 - Rising trigger event configuration bit of line 32
    #[inline(always)]
    pub fn rt32(&self) -> RT32_R {
        RT32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Rising trigger event configuration bit of line 32
    #[inline(always)]
    pub fn rt33(&self) -> RT33_R {
        RT33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Rising trigger event configuration bit of line 40
    #[inline(always)]
    pub fn rt40(&self) -> RT40_R {
        RT40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Rising trigger event configuration bit of line 41
    #[inline(always)]
    pub fn rt41(&self) -> RT41_R {
        RT41_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTSR2")
            .field("rt32", &self.rt32())
            .field("rt33", &self.rt33())
            .field("rt40", &self.rt40())
            .field("rt41", &self.rt41())
            .finish()
    }
}
impl W {
    ///Bit 0 - Rising trigger event configuration bit of line 32
    #[inline(always)]
    pub fn rt32(&mut self) -> RT32_W<'_, RTSR2rs> {
        RT32_W::new(self, 0)
    }
    ///Bit 1 - Rising trigger event configuration bit of line 32
    #[inline(always)]
    pub fn rt33(&mut self) -> RT33_W<'_, RTSR2rs> {
        RT33_W::new(self, 1)
    }
    ///Bit 8 - Rising trigger event configuration bit of line 40
    #[inline(always)]
    pub fn rt40(&mut self) -> RT40_W<'_, RTSR2rs> {
        RT40_W::new(self, 8)
    }
    ///Bit 9 - Rising trigger event configuration bit of line 41
    #[inline(always)]
    pub fn rt41(&mut self) -> RT41_W<'_, RTSR2rs> {
        RT41_W::new(self, 9)
    }
}
/**Rising Trigger selection register

You can [`read`](crate::Reg::read) this register and get [`rtsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#EXTI:RTSR2)*/
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
