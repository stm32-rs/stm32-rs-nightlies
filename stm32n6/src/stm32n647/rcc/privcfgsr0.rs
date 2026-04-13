///Register `PRIVCFGSR0` writer
pub type W = crate::W<PRIVCFGSR0rs>;
///Field `LSIPVS` writer - Defines the privilege protection of the LSI configuration bits (enable, ready, divider).
pub type LSIPVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSEPVS` writer - Defines the privilege protection of the LSE configuration bits (enable, ready, divider).
pub type LSEPVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSIPVS` writer - Defines the privilege protection of the MSI configuration bits (enable, ready, divider).
pub type MSIPVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIPVS` writer - Defines the privilege protection of the HSI configuration bits (enable, ready, divider).
pub type HSIPVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSEPVS` writer - Defines the privilege protection of the HSE configuration bits (enable, ready, divider).
pub type HSEPVS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PRIVCFGSR0rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the privilege protection of the LSI configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn lsipvs(&mut self) -> LSIPVS_W<'_, PRIVCFGSR0rs> {
        LSIPVS_W::new(self, 0)
    }
    ///Bit 1 - Defines the privilege protection of the LSE configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn lsepvs(&mut self) -> LSEPVS_W<'_, PRIVCFGSR0rs> {
        LSEPVS_W::new(self, 1)
    }
    ///Bit 2 - Defines the privilege protection of the MSI configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn msipvs(&mut self) -> MSIPVS_W<'_, PRIVCFGSR0rs> {
        MSIPVS_W::new(self, 2)
    }
    ///Bit 3 - Defines the privilege protection of the HSI configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn hsipvs(&mut self) -> HSIPVS_W<'_, PRIVCFGSR0rs> {
        HSIPVS_W::new(self, 3)
    }
    ///Bit 4 - Defines the privilege protection of the HSE configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn hsepvs(&mut self) -> HSEPVS_W<'_, PRIVCFGSR0rs> {
        HSEPVS_W::new(self, 4)
    }
}
/**RCC oscillator privilege configuration register0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgsr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PRIVCFGSR0)*/
pub struct PRIVCFGSR0rs;
impl crate::RegisterSpec for PRIVCFGSR0rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`privcfgsr0::W`](W) writer structure
impl crate::Writable for PRIVCFGSR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGSR0 to value 0
impl crate::Resettable for PRIVCFGSR0rs {}
