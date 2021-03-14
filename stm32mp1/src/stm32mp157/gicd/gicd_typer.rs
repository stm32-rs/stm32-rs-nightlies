#[doc = "Reader of register GICD_TYPER"]
pub type R = crate::R<u32, super::GICD_TYPER>;
#[doc = "Reader of field `ITLINESNUMBER`"]
pub type ITLINESNUMBER_R = crate::R<u8, u8>;
#[doc = "Reader of field `CPUNUMBER`"]
pub type CPUNUMBER_R = crate::R<u8, u8>;
#[doc = "Reader of field `SECURITYEXTN`"]
pub type SECURITYEXTN_R = crate::R<bool, bool>;
#[doc = "Reader of field `LSPI`"]
pub type LSPI_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - ITLINESNUMBER"]
    #[inline(always)]
    pub fn itlinesnumber(&self) -> ITLINESNUMBER_R {
        ITLINESNUMBER_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - CPUNUMBER"]
    #[inline(always)]
    pub fn cpunumber(&self) -> CPUNUMBER_R {
        CPUNUMBER_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 10 - SECURITYEXTN"]
    #[inline(always)]
    pub fn securityextn(&self) -> SECURITYEXTN_R {
        SECURITYEXTN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:15 - LSPI"]
    #[inline(always)]
    pub fn lspi(&self) -> LSPI_R {
        LSPI_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
