///Register `RTSR2` reader
pub type R = crate::R<RTSR2rs>;
///Register `RTSR2` writer
pub type W = crate::W<RTSR2rs>;
/**Rising trigger event configuration bit of line 35

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
///Field `RT35` reader - Rising trigger event configuration bit of line 35
pub type RT35_R = crate::BitReader<RISING_TRIGGER>;
impl RT35_R {
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
///Field `RT35` writer - Rising trigger event configuration bit of line 35
pub type RT35_W<'a, REG> = crate::BitWriter<'a, REG, RISING_TRIGGER>;
impl<'a, REG> RT35_W<'a, REG>
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
///Field `RT36` reader - Rising trigger event configuration bit of line 36
pub use RT35_R as RT36_R;
///Field `RT37` reader - Rising trigger event configuration bit of line 37
pub use RT35_R as RT37_R;
///Field `RT38` reader - Rising trigger event configuration bit of line 38
pub use RT35_R as RT38_R;
///Field `RT36` writer - Rising trigger event configuration bit of line 36
pub use RT35_W as RT36_W;
///Field `RT37` writer - Rising trigger event configuration bit of line 37
pub use RT35_W as RT37_W;
///Field `RT38` writer - Rising trigger event configuration bit of line 38
pub use RT35_W as RT38_W;
impl R {
    ///Bit 3 - Rising trigger event configuration bit of line 35
    #[inline(always)]
    pub fn rt35(&self) -> RT35_R {
        RT35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rising trigger event configuration bit of line 36
    #[inline(always)]
    pub fn rt36(&self) -> RT36_R {
        RT36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rising trigger event configuration bit of line 37
    #[inline(always)]
    pub fn rt37(&self) -> RT37_R {
        RT37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Rising trigger event configuration bit of line 38
    #[inline(always)]
    pub fn rt38(&self) -> RT38_R {
        RT38_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTSR2")
            .field("rt35", &self.rt35())
            .field("rt36", &self.rt36())
            .field("rt37", &self.rt37())
            .field("rt38", &self.rt38())
            .finish()
    }
}
impl W {
    ///Bit 3 - Rising trigger event configuration bit of line 35
    #[inline(always)]
    pub fn rt35(&mut self) -> RT35_W<'_, RTSR2rs> {
        RT35_W::new(self, 3)
    }
    ///Bit 4 - Rising trigger event configuration bit of line 36
    #[inline(always)]
    pub fn rt36(&mut self) -> RT36_W<'_, RTSR2rs> {
        RT36_W::new(self, 4)
    }
    ///Bit 5 - Rising trigger event configuration bit of line 37
    #[inline(always)]
    pub fn rt37(&mut self) -> RT37_W<'_, RTSR2rs> {
        RT37_W::new(self, 5)
    }
    ///Bit 6 - Rising trigger event configuration bit of line 38
    #[inline(always)]
    pub fn rt38(&mut self) -> RT38_W<'_, RTSR2rs> {
        RT38_W::new(self, 6)
    }
}
/**Rising Trigger selection register

You can [`read`](crate::Reg::read) this register and get [`rtsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#EXTI:RTSR2)*/
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
