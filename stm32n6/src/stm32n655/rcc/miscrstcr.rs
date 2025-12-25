///Register `MISCRSTCR` writer
pub type W = crate::W<MISCRSTCRrs>;
///Field `DBGRSTC` writer - DBG reset
pub type DBGRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPIPHY1RSTC` writer - XSPIPHY1 reset
pub type XSPIPHY1RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPIPHY2RSTC` writer - XSPIPHY2 reset
pub type XSPIPHY2RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC1DLLRSTC` writer - SDMMC1DLL reset
pub type SDMMC1DLLRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC2DLLRSTC` writer - SDMMC2DLL reset
pub type SDMMC2DLLRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<MISCRSTCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - DBG reset
    #[inline(always)]
    pub fn dbgrstc(&mut self) -> DBGRSTC_W<'_, MISCRSTCRrs> {
        DBGRSTC_W::new(self, 0)
    }
    ///Bit 4 - XSPIPHY1 reset
    #[inline(always)]
    pub fn xspiphy1rstc(&mut self) -> XSPIPHY1RSTC_W<'_, MISCRSTCRrs> {
        XSPIPHY1RSTC_W::new(self, 4)
    }
    ///Bit 5 - XSPIPHY2 reset
    #[inline(always)]
    pub fn xspiphy2rstc(&mut self) -> XSPIPHY2RSTC_W<'_, MISCRSTCRrs> {
        XSPIPHY2RSTC_W::new(self, 5)
    }
    ///Bit 7 - SDMMC1DLL reset
    #[inline(always)]
    pub fn sdmmc1dllrstc(&mut self) -> SDMMC1DLLRSTC_W<'_, MISCRSTCRrs> {
        SDMMC1DLLRSTC_W::new(self, 7)
    }
    ///Bit 8 - SDMMC2DLL reset
    #[inline(always)]
    pub fn sdmmc2dllrstc(&mut self) -> SDMMC2DLLRSTC_W<'_, MISCRSTCRrs> {
        SDMMC2DLLRSTC_W::new(self, 8)
    }
}
/**RCC miscellaneous reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miscrstcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:MISCRSTCR)*/
pub struct MISCRSTCRrs;
impl crate::RegisterSpec for MISCRSTCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`miscrstcr::W`](W) writer structure
impl crate::Writable for MISCRSTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MISCRSTCR to value 0
impl crate::Resettable for MISCRSTCRrs {}
