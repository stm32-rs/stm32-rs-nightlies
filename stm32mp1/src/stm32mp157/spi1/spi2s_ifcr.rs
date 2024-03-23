#[doc = "Register `SPI2S_IFCR` writer"]
pub type W = crate::W<SPI2S_IFCRrs>;
#[doc = "Field `EOTC` writer - EOTC"]
pub type EOTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTFC` writer - TXTFC"]
pub type TXTFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDRC` writer - UDRC"]
pub type UDRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRC` writer - OVRC"]
pub type OVRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEC` writer - CRCEC"]
pub type CRCEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIFREC` writer - TIFREC"]
pub type TIFREC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODFC` writer - MODFC"]
pub type MODFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSERFC` writer - TSERFC"]
pub type TSERFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPC` writer - SUSPC"]
pub type SUSPC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 3 - EOTC"]
    #[inline(always)]
    #[must_use]
    pub fn eotc(&mut self) -> EOTC_W<SPI2S_IFCRrs> {
        EOTC_W::new(self, 3)
    }
    #[doc = "Bit 4 - TXTFC"]
    #[inline(always)]
    #[must_use]
    pub fn txtfc(&mut self) -> TXTFC_W<SPI2S_IFCRrs> {
        TXTFC_W::new(self, 4)
    }
    #[doc = "Bit 5 - UDRC"]
    #[inline(always)]
    #[must_use]
    pub fn udrc(&mut self) -> UDRC_W<SPI2S_IFCRrs> {
        UDRC_W::new(self, 5)
    }
    #[doc = "Bit 6 - OVRC"]
    #[inline(always)]
    #[must_use]
    pub fn ovrc(&mut self) -> OVRC_W<SPI2S_IFCRrs> {
        OVRC_W::new(self, 6)
    }
    #[doc = "Bit 7 - CRCEC"]
    #[inline(always)]
    #[must_use]
    pub fn crcec(&mut self) -> CRCEC_W<SPI2S_IFCRrs> {
        CRCEC_W::new(self, 7)
    }
    #[doc = "Bit 8 - TIFREC"]
    #[inline(always)]
    #[must_use]
    pub fn tifrec(&mut self) -> TIFREC_W<SPI2S_IFCRrs> {
        TIFREC_W::new(self, 8)
    }
    #[doc = "Bit 9 - MODFC"]
    #[inline(always)]
    #[must_use]
    pub fn modfc(&mut self) -> MODFC_W<SPI2S_IFCRrs> {
        MODFC_W::new(self, 9)
    }
    #[doc = "Bit 10 - TSERFC"]
    #[inline(always)]
    #[must_use]
    pub fn tserfc(&mut self) -> TSERFC_W<SPI2S_IFCRrs> {
        TSERFC_W::new(self, 10)
    }
    #[doc = "Bit 11 - SUSPC"]
    #[inline(always)]
    #[must_use]
    pub fn suspc(&mut self) -> SUSPC_W<SPI2S_IFCRrs> {
        SUSPC_W::new(self, 11)
    }
}
#[doc = "SPI/I2S interrupt/status flags clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi2s_ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI2S_IFCRrs;
impl crate::RegisterSpec for SPI2S_IFCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`spi2s_ifcr::W`](W) writer structure"]
impl crate::Writable for SPI2S_IFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI2S_IFCR to value 0"]
impl crate::Resettable for SPI2S_IFCRrs {
    const RESET_VALUE: u32 = 0;
}
