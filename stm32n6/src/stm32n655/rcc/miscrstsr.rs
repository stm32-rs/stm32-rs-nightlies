///Register `MISCRSTSR` writer
pub type W = crate::W<MISCRSTSRrs>;
///Field `DBGRSTS` writer - DBG reset
pub type DBGRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPIPHY1RSTS` writer - XSPIPHY1 reset
pub type XSPIPHY1RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPIPHY2RSTS` writer - XSPIPHY2 reset
pub type XSPIPHY2RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC1DLLRSTS` writer - SDMMC1DLL reset
pub type SDMMC1DLLRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC2DLLRSTS` writer - SDMMC2DLL reset
pub type SDMMC2DLLRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<MISCRSTSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - DBG reset
    #[inline(always)]
    pub fn dbgrsts(&mut self) -> DBGRSTS_W<'_, MISCRSTSRrs> {
        DBGRSTS_W::new(self, 0)
    }
    ///Bit 4 - XSPIPHY1 reset
    #[inline(always)]
    pub fn xspiphy1rsts(&mut self) -> XSPIPHY1RSTS_W<'_, MISCRSTSRrs> {
        XSPIPHY1RSTS_W::new(self, 4)
    }
    ///Bit 5 - XSPIPHY2 reset
    #[inline(always)]
    pub fn xspiphy2rsts(&mut self) -> XSPIPHY2RSTS_W<'_, MISCRSTSRrs> {
        XSPIPHY2RSTS_W::new(self, 5)
    }
    ///Bit 7 - SDMMC1DLL reset
    #[inline(always)]
    pub fn sdmmc1dllrsts(&mut self) -> SDMMC1DLLRSTS_W<'_, MISCRSTSRrs> {
        SDMMC1DLLRSTS_W::new(self, 7)
    }
    ///Bit 8 - SDMMC2DLL reset
    #[inline(always)]
    pub fn sdmmc2dllrsts(&mut self) -> SDMMC2DLLRSTS_W<'_, MISCRSTSRrs> {
        SDMMC2DLLRSTS_W::new(self, 8)
    }
}
/**RCC miscellaneous reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miscrstsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:MISCRSTSR)*/
pub struct MISCRSTSRrs;
impl crate::RegisterSpec for MISCRSTSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`miscrstsr::W`](W) writer structure
impl crate::Writable for MISCRSTSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MISCRSTSR to value 0
impl crate::Resettable for MISCRSTSRrs {}
