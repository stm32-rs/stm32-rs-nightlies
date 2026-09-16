///Register `PWRCR` reader
pub type R = crate::R<PWRCRrs>;
///Register `PWRCR` writer
pub type W = crate::W<PWRCRrs>;
/**Overdrive enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODEN {
    ///0: Overdrive mode disabled
    Disabled = 0,
    ///1: Overdrive mode enabled (the LDO generates VOS0 for VCORE)
    Enabled = 1,
}
impl From<ODEN> for bool {
    #[inline(always)]
    fn from(variant: ODEN) -> Self {
        variant as u8 != 0
    }
}
///Field `ODEN` reader - Overdrive enable
pub type ODEN_R = crate::BitReader<ODEN>;
impl ODEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ODEN {
        match self.bits {
            false => ODEN::Disabled,
            true => ODEN::Enabled,
        }
    }
    ///Overdrive mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ODEN::Disabled
    }
    ///Overdrive mode enabled (the LDO generates VOS0 for VCORE)
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ODEN::Enabled
    }
}
///Field `ODEN` writer - Overdrive enable
pub type ODEN_W<'a, REG> = crate::BitWriter<'a, REG, ODEN>;
impl<'a, REG> ODEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Overdrive mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ODEN::Disabled)
    }
    ///Overdrive mode enabled (the LDO generates VOS0 for VCORE)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ODEN::Enabled)
    }
}
impl R {
    ///Bit 0 - Overdrive enable
    #[inline(always)]
    pub fn oden(&self) -> ODEN_R {
        ODEN_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRCR").field("oden", &self.oden()).finish()
    }
}
impl W {
    ///Bit 0 - Overdrive enable
    #[inline(always)]
    pub fn oden(&mut self) -> ODEN_W<'_, PWRCRrs> {
        ODEN_W::new(self, 0)
    }
}
/**SYSCFG power control register

You can [`read`](crate::Reg::read) this register and get [`pwrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#SYSCFG:PWRCR)*/
pub struct PWRCRrs;
impl crate::RegisterSpec for PWRCRrs {
    type Ux = u32;
}
///`read()` method returns [`pwrcr::R`](R) reader structure
impl crate::Readable for PWRCRrs {}
///`write(|w| ..)` method takes [`pwrcr::W`](W) writer structure
impl crate::Writable for PWRCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PWRCR to value 0
impl crate::Resettable for PWRCRrs {}
