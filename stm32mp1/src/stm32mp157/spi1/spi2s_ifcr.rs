///Register `SPI2S_IFCR` writer
pub type W = crate::W<SPI2S_IFCRrs>;
///Field `EOTC` writer - EOTC
pub type EOTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXTFC` writer - TXTFC
pub type TXTFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UDRC` writer - UDRC
pub type UDRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVRC` writer - OVRC
pub type OVRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCEC` writer - CRCEC
pub type CRCEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIFREC` writer - TIFREC
pub type TIFREC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODFC` writer - MODFC
pub type MODFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSERFC` writer - TSERFC
pub type TSERFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUSPC` writer - SUSPC
pub type SUSPC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<SPI2S_IFCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 3 - EOTC
    #[inline(always)]
    pub fn eotc(&mut self) -> EOTC_W<'_, SPI2S_IFCRrs> {
        EOTC_W::new(self, 3)
    }
    ///Bit 4 - TXTFC
    #[inline(always)]
    pub fn txtfc(&mut self) -> TXTFC_W<'_, SPI2S_IFCRrs> {
        TXTFC_W::new(self, 4)
    }
    ///Bit 5 - UDRC
    #[inline(always)]
    pub fn udrc(&mut self) -> UDRC_W<'_, SPI2S_IFCRrs> {
        UDRC_W::new(self, 5)
    }
    ///Bit 6 - OVRC
    #[inline(always)]
    pub fn ovrc(&mut self) -> OVRC_W<'_, SPI2S_IFCRrs> {
        OVRC_W::new(self, 6)
    }
    ///Bit 7 - CRCEC
    #[inline(always)]
    pub fn crcec(&mut self) -> CRCEC_W<'_, SPI2S_IFCRrs> {
        CRCEC_W::new(self, 7)
    }
    ///Bit 8 - TIFREC
    #[inline(always)]
    pub fn tifrec(&mut self) -> TIFREC_W<'_, SPI2S_IFCRrs> {
        TIFREC_W::new(self, 8)
    }
    ///Bit 9 - MODFC
    #[inline(always)]
    pub fn modfc(&mut self) -> MODFC_W<'_, SPI2S_IFCRrs> {
        MODFC_W::new(self, 9)
    }
    ///Bit 10 - TSERFC
    #[inline(always)]
    pub fn tserfc(&mut self) -> TSERFC_W<'_, SPI2S_IFCRrs> {
        TSERFC_W::new(self, 10)
    }
    ///Bit 11 - SUSPC
    #[inline(always)]
    pub fn suspc(&mut self) -> SUSPC_W<'_, SPI2S_IFCRrs> {
        SUSPC_W::new(self, 11)
    }
}
/**SPI/I2S interrupt/status flags clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2s_ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SPI1:SPI2S_IFCR)*/
pub struct SPI2S_IFCRrs;
impl crate::RegisterSpec for SPI2S_IFCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`spi2s_ifcr::W`](W) writer structure
impl crate::Writable for SPI2S_IFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPI2S_IFCR to value 0
impl crate::Resettable for SPI2S_IFCRrs {}
