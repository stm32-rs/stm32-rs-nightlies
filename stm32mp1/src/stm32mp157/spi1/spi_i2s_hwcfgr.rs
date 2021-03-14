#[doc = "Reader of register SPI_I2S_HWCFGR"]
pub type R = crate::R<u32, super::SPI_I2S_HWCFGR>;
#[doc = "Reader of field `TXFCFG`"]
pub type TXFCFG_R = crate::R<u8, u8>;
#[doc = "Reader of field `RXFCFG`"]
pub type RXFCFG_R = crate::R<u8, u8>;
#[doc = "Reader of field `CRCCFG`"]
pub type CRCCFG_R = crate::R<u8, u8>;
#[doc = "Reader of field `I2SCFG`"]
pub type I2SCFG_R = crate::R<u8, u8>;
#[doc = "Reader of field `DSCFG`"]
pub type DSCFG_R = crate::R<u8, u8>;
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
