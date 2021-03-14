#[doc = "Reader of register HWCFGR"]
pub type R = crate::R<u32, super::HWCFGR>;
#[doc = "Reader of field `CRCCFG`"]
pub type CRCCFG_R = crate::R<u8, u8>;
#[doc = "Reader of field `I2SCFG`"]
pub type I2SCFG_R = crate::R<u8, u8>;
#[doc = "Reader of field `I2SCKCFG`"]
pub type I2SCKCFG_R = crate::R<u8, u8>;
#[doc = "Reader of field `DSCFG`"]
pub type DSCFG_R = crate::R<u8, u8>;
#[doc = "Reader of field `NSSCFG`"]
pub type NSSCFG_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - CRC capable at SPI mode"]
    #[inline(always)]
    pub fn crccfg(&self) -> CRCCFG_R {
        CRCCFG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - I2S mode implementation"]
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - I2S master clock generator at I2S mode"]
    #[inline(always)]
    pub fn i2sckcfg(&self) -> I2SCKCFG_R {
        I2SCKCFG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SPI data size configuration"]
    #[inline(always)]
    pub fn dscfg(&self) -> DSCFG_R {
        DSCFG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - NSS pulse feature enhancement at SPI master"]
    #[inline(always)]
    pub fn nsscfg(&self) -> NSSCFG_R {
        NSSCFG_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
