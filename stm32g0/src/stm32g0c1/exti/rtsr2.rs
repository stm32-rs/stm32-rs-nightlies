///Register `RTSR2` reader
pub type R = crate::R<RTSR2rs>;
///Register `RTSR2` writer
pub type W = crate::W<RTSR2rs>;
/**Rising trigger event configuration bit of configurable line 34

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT2 {
    ///0: Rising edge trigger is disabled
    Disabled = 0,
    ///1: Rising edge trigger is enabled
    Enabled = 1,
}
impl From<RT2> for bool {
    #[inline(always)]
    fn from(variant: RT2) -> Self {
        variant as u8 != 0
    }
}
///Field `RT2` reader - Rising trigger event configuration bit of configurable line 34
pub type RT2_R = crate::BitReader<RT2>;
impl RT2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RT2 {
        match self.bits {
            false => RT2::Disabled,
            true => RT2::Enabled,
        }
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RT2::Disabled
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RT2::Enabled
    }
}
///Field `RT2` writer - Rising trigger event configuration bit of configurable line 34
pub type RT2_W<'a, REG> = crate::BitWriter<'a, REG, RT2>;
impl<'a, REG> RT2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RT2::Disabled)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RT2::Enabled)
    }
}
impl R {
    ///Bit 2 - Rising trigger event configuration bit of configurable line 34
    #[inline(always)]
    pub fn rt2(&self) -> RT2_R {
        RT2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTSR2").field("rt2", &self.rt2()).finish()
    }
}
impl W {
    ///Bit 2 - Rising trigger event configuration bit of configurable line 34
    #[inline(always)]
    #[must_use]
    pub fn rt2(&mut self) -> RT2_W<RTSR2rs> {
        RT2_W::new(self, 2)
    }
}
/**EXTI rising trigger selection register 2

You can [`read`](crate::Reg::read) this register and get [`rtsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0C1.html#EXTI:RTSR2)*/
pub struct RTSR2rs;
impl crate::RegisterSpec for RTSR2rs {
    type Ux = u32;
}
///`read()` method returns [`rtsr2::R`](R) reader structure
impl crate::Readable for RTSR2rs {}
///`write(|w| ..)` method takes [`rtsr2::W`](W) writer structure
impl crate::Writable for RTSR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RTSR2 to value 0
impl crate::Resettable for RTSR2rs {
    const RESET_VALUE: u32 = 0;
}
