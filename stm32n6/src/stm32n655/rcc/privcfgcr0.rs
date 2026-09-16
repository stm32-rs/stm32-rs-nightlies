///Register `PRIVCFGCR0` writer
pub type W = crate::W<PRIVCFGCR0rs>;
///Field `LSIPVC` writer - Defines the privilege protection of the LSI configuration bits (enable, ready, divider).
pub type LSIPVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSEPVC` writer - Defines the privilege protection of the LSE configuration bits (enable, ready, divider).
pub type LSEPVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSIPVC` writer - Defines the privilege protection of the MSI configuration bits (enable, ready, divider).
pub type MSIPVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIPVC` writer - Defines the privilege protection of the HSI configuration bits (enable, ready, divider).
pub type HSIPVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSEPVC` writer - Defines the privilege protection of the HSE configuration bits (enable, ready, divider).
pub type HSEPVC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PRIVCFGCR0rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the privilege protection of the LSI configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn lsipvc(&mut self) -> LSIPVC_W<'_, PRIVCFGCR0rs> {
        LSIPVC_W::new(self, 0)
    }
    ///Bit 1 - Defines the privilege protection of the LSE configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn lsepvc(&mut self) -> LSEPVC_W<'_, PRIVCFGCR0rs> {
        LSEPVC_W::new(self, 1)
    }
    ///Bit 2 - Defines the privilege protection of the MSI configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn msipvc(&mut self) -> MSIPVC_W<'_, PRIVCFGCR0rs> {
        MSIPVC_W::new(self, 2)
    }
    ///Bit 3 - Defines the privilege protection of the HSI configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn hsipvc(&mut self) -> HSIPVC_W<'_, PRIVCFGCR0rs> {
        HSIPVC_W::new(self, 3)
    }
    ///Bit 4 - Defines the privilege protection of the HSE configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn hsepvc(&mut self) -> HSEPVC_W<'_, PRIVCFGCR0rs> {
        HSEPVC_W::new(self, 4)
    }
}
/**RCC oscillator privilege configuration register0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgcr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:PRIVCFGCR0)*/
pub struct PRIVCFGCR0rs;
impl crate::RegisterSpec for PRIVCFGCR0rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`privcfgcr0::W`](W) writer structure
impl crate::Writable for PRIVCFGCR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGCR0 to value 0
impl crate::Resettable for PRIVCFGCR0rs {}
