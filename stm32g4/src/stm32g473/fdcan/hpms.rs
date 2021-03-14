#[doc = "Reader of register HPMS"]
pub type R = crate::R<u32, super::HPMS>;
#[doc = "Reader of field `BIDX`"]
pub type BIDX_R = crate::R<u8, u8>;
#[doc = "Reader of field `MSI`"]
pub type MSI_R = crate::R<u8, u8>;
#[doc = "Reader of field `FIDX`"]
pub type FIDX_R = crate::R<u8, u8>;
#[doc = "Reader of field `FLST`"]
pub type FLST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:5 - BIDX"]
    #[inline(always)]
    pub fn bidx(&self) -> BIDX_R {
        BIDX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - MSI"]
    #[inline(always)]
    pub fn msi(&self) -> MSI_R {
        MSI_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:14 - FIDX"]
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - FLST"]
    #[inline(always)]
    pub fn flst(&self) -> FLST_R {
        FLST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
