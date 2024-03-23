#[doc = "Register `SPI_I2S_HWCFGR` reader"]
pub type R = crate::R<SPI_I2S_HWCFGRrs>;
#[doc = "Field `TXFCFG` reader - TXFCFG"]
pub type TXFCFG_R = crate::FieldReader;
#[doc = "Field `RXFCFG` reader - RXFCFG"]
pub type RXFCFG_R = crate::FieldReader;
#[doc = "Field `CRCCFG` reader - CRCCFG"]
pub type CRCCFG_R = crate::FieldReader;
#[doc = "Field `I2SCFG` reader - I2SCFG"]
pub type I2SCFG_R = crate::FieldReader;
#[doc = "Field `DSCFG` reader - DSCFG"]
pub type DSCFG_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - TXFCFG"]
    #[inline(always)]
    pub fn txfcfg(&self) -> TXFCFG_R {
        TXFCFG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RXFCFG"]
    #[inline(always)]
    pub fn rxfcfg(&self) -> RXFCFG_R {
        RXFCFG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - CRCCFG"]
    #[inline(always)]
    pub fn crccfg(&self) -> CRCCFG_R {
        CRCCFG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - I2SCFG"]
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DSCFG"]
    #[inline(always)]
    pub fn dscfg(&self) -> DSCFG_R {
        DSCFG_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "SPI/I2S hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_i2s_hwcfgr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_I2S_HWCFGRrs;
impl crate::RegisterSpec for SPI_I2S_HWCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_i2s_hwcfgr::R`](R) reader structure"]
impl crate::Readable for SPI_I2S_HWCFGRrs {}
#[doc = "`reset()` method sets SPI_I2S_HWCFGR to value 0"]
impl crate::Resettable for SPI_I2S_HWCFGRrs {
    const RESET_VALUE: u32 = 0;
}
