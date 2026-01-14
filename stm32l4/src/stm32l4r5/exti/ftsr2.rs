///Register `FTSR2` reader
pub type R = crate::R<FTSR2rs>;
///Register `FTSR2` writer
pub type W = crate::W<FTSR2rs>;
/**Falling trigger event configuration bit of line 35

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
///Field `FT35` reader - Falling trigger event configuration bit of line 35
pub type FT35_R = crate::BitReader<FALLING_TRIGGER>;
impl FT35_R {
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
///Field `FT35` writer - Falling trigger event configuration bit of line 35
pub type FT35_W<'a, REG> = crate::BitWriter<'a, REG, FALLING_TRIGGER>;
impl<'a, REG> FT35_W<'a, REG>
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
///Field `FT36` reader - Falling trigger event configuration bit of line 36
pub use FT35_R as FT36_R;
///Field `FT37` reader - Falling trigger event configuration bit of line 37
pub use FT35_R as FT37_R;
///Field `FT38` reader - Falling trigger event configuration bit of line 38
pub use FT35_R as FT38_R;
///Field `FT36` writer - Falling trigger event configuration bit of line 36
pub use FT35_W as FT36_W;
///Field `FT37` writer - Falling trigger event configuration bit of line 37
pub use FT35_W as FT37_W;
///Field `FT38` writer - Falling trigger event configuration bit of line 38
pub use FT35_W as FT38_W;
impl R {
    ///Bit 3 - Falling trigger event configuration bit of line 35
    #[inline(always)]
    pub fn ft35(&self) -> FT35_R {
        FT35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Falling trigger event configuration bit of line 36
    #[inline(always)]
    pub fn ft36(&self) -> FT36_R {
        FT36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Falling trigger event configuration bit of line 37
    #[inline(always)]
    pub fn ft37(&self) -> FT37_R {
        FT37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Falling trigger event configuration bit of line 38
    #[inline(always)]
    pub fn ft38(&self) -> FT38_R {
        FT38_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FTSR2")
            .field("ft35", &self.ft35())
            .field("ft36", &self.ft36())
            .field("ft37", &self.ft37())
            .field("ft38", &self.ft38())
            .finish()
    }
}
impl W {
    ///Bit 3 - Falling trigger event configuration bit of line 35
    #[inline(always)]
    pub fn ft35(&mut self) -> FT35_W<'_, FTSR2rs> {
        FT35_W::new(self, 3)
    }
    ///Bit 4 - Falling trigger event configuration bit of line 36
    #[inline(always)]
    pub fn ft36(&mut self) -> FT36_W<'_, FTSR2rs> {
        FT36_W::new(self, 4)
    }
    ///Bit 5 - Falling trigger event configuration bit of line 37
    #[inline(always)]
    pub fn ft37(&mut self) -> FT37_W<'_, FTSR2rs> {
        FT37_W::new(self, 5)
    }
    ///Bit 6 - Falling trigger event configuration bit of line 38
    #[inline(always)]
    pub fn ft38(&mut self) -> FT38_W<'_, FTSR2rs> {
        FT38_W::new(self, 6)
    }
}
/**Falling Trigger selection register

You can [`read`](crate::Reg::read) this register and get [`ftsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#EXTI:FTSR2)*/
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
