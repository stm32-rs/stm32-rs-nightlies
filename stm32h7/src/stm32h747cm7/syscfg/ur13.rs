#[doc = "Reader of register UR13"]
pub type R = crate::R<u32, super::UR13>;
#[doc = "Reader of field `SDRS`"]
pub type SDRS_R = crate::R<u8, u8>;
#[doc = "Reader of field `D1SBRST`"]
pub type D1SBRST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:1 - Secured DTCM RAM Size"]
    #[inline(always)]
    pub fn sdrs(&self) -> SDRS_R {
        SDRS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 16 - D1 Standby reset"]
    #[inline(always)]
    pub fn d1sbrst(&self) -> D1SBRST_R {
        D1SBRST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
