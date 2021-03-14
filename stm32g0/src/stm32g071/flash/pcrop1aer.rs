#[doc = "Reader of register PCROP1AER"]
pub type R = crate::R<u32, super::PCROP1AER>;
#[doc = "Reader of field `PCROP1A_END`"]
pub type PCROP1A_END_R = crate::R<u8, u8>;
#[doc = "Reader of field `PCROP_RDP`"]
pub type PCROP_RDP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:7 - PCROP1A area end offset"]
    #[inline(always)]
    pub fn pcrop1a_end(&self) -> PCROP1A_END_R {
        PCROP1A_END_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - PCROP area preserved when RDP level decreased"]
    #[inline(always)]
    pub fn pcrop_rdp(&self) -> PCROP_RDP_R {
        PCROP_RDP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
