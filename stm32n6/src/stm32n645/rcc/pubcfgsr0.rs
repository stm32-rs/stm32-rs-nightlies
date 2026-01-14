///Register `PUBCFGSR0` writer
pub type W = crate::W<PUBCFGSR0rs>;
///Field `LSIPUBS` writer - Defines the public protection of the LSI configuration bits (enable, ready, divider).
pub type LSIPUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSEPUBS` writer - Defines the public protection of the LSE configuration bits (enable, ready, divider).
pub type LSEPUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSIPUBS` writer - Defines the public protection of the MSI configuration bits (enable, ready, divider).
pub type MSIPUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIPUBS` writer - Defines the public protection of the HSI configuration bits (enable, ready, divider).
pub type HSIPUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSEPUBS` writer - Defines the public protection of the HSE configuration bits (enable, ready, divider).
pub type HSEPUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PUBCFGSR0rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the public protection of the LSI configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn lsipubs(&mut self) -> LSIPUBS_W<'_, PUBCFGSR0rs> {
        LSIPUBS_W::new(self, 0)
    }
    ///Bit 1 - Defines the public protection of the LSE configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn lsepubs(&mut self) -> LSEPUBS_W<'_, PUBCFGSR0rs> {
        LSEPUBS_W::new(self, 1)
    }
    ///Bit 2 - Defines the public protection of the MSI configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn msipubs(&mut self) -> MSIPUBS_W<'_, PUBCFGSR0rs> {
        MSIPUBS_W::new(self, 2)
    }
    ///Bit 3 - Defines the public protection of the HSI configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn hsipubs(&mut self) -> HSIPUBS_W<'_, PUBCFGSR0rs> {
        HSIPUBS_W::new(self, 3)
    }
    ///Bit 4 - Defines the public protection of the HSE configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn hsepubs(&mut self) -> HSEPUBS_W<'_, PUBCFGSR0rs> {
        HSEPUBS_W::new(self, 4)
    }
}
/**RCC oscillator public configuration register0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgsr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:PUBCFGSR0)*/
pub struct PUBCFGSR0rs;
impl crate::RegisterSpec for PUBCFGSR0rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`pubcfgsr0::W`](W) writer structure
impl crate::Writable for PUBCFGSR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUBCFGSR0 to value 0
impl crate::Resettable for PUBCFGSR0rs {}
