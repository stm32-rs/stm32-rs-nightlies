///Register `PUBCFGCR0` writer
pub type W = crate::W<PUBCFGCR0rs>;
///Field `LSIPUBC` writer - Defines the public protection of the LSI configuration bits (enable, ready, divider).
pub type LSIPUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSEPUBC` writer - Defines the public protection of the LSE configuration bits (enable, ready, divider).
pub type LSEPUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSIPUBC` writer - Defines the public protection of the MSI configuration bits (enable, ready, divider).
pub type MSIPUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIPUBC` writer - Defines the public protection of the HSI configuration bits (enable, ready, divider).
pub type HSIPUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSEPUBC` writer - Defines the public protection of the HSE configuration bits (enable, ready, divider).
pub type HSEPUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PUBCFGCR0rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the public protection of the LSI configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn lsipubc(&mut self) -> LSIPUBC_W<'_, PUBCFGCR0rs> {
        LSIPUBC_W::new(self, 0)
    }
    ///Bit 1 - Defines the public protection of the LSE configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn lsepubc(&mut self) -> LSEPUBC_W<'_, PUBCFGCR0rs> {
        LSEPUBC_W::new(self, 1)
    }
    ///Bit 2 - Defines the public protection of the MSI configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn msipubc(&mut self) -> MSIPUBC_W<'_, PUBCFGCR0rs> {
        MSIPUBC_W::new(self, 2)
    }
    ///Bit 3 - Defines the public protection of the HSI configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn hsipubc(&mut self) -> HSIPUBC_W<'_, PUBCFGCR0rs> {
        HSIPUBC_W::new(self, 3)
    }
    ///Bit 4 - Defines the public protection of the HSE configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn hsepubc(&mut self) -> HSEPUBC_W<'_, PUBCFGCR0rs> {
        HSEPUBC_W::new(self, 4)
    }
}
/**RCC oscillator public configuration register0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgcr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PUBCFGCR0)*/
pub struct PUBCFGCR0rs;
impl crate::RegisterSpec for PUBCFGCR0rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`pubcfgcr0::W`](W) writer structure
impl crate::Writable for PUBCFGCR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUBCFGCR0 to value 0
impl crate::Resettable for PUBCFGCR0rs {}
