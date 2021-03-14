#[doc = "Reader of register RX_ORDSET"]
pub type R = crate::R<u32, super::RX_ORDSET>;
#[doc = "Reader of field `RXORDSET`"]
pub type RXORDSET_R = crate::R<u8, u8>;
#[doc = "Reader of field `RXSOP3OF4`"]
pub type RXSOP3OF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXSOPKINVALID`"]
pub type RXSOPKINVALID_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - RXORDSET"]
    #[inline(always)]
    pub fn rxordset(&self) -> RXORDSET_R {
        RXORDSET_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - RXSOP3OF4"]
    #[inline(always)]
    pub fn rxsop3of4(&self) -> RXSOP3OF4_R {
        RXSOP3OF4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - RXSOPKINVALID"]
    #[inline(always)]
    pub fn rxsopkinvalid(&self) -> RXSOPKINVALID_R {
        RXSOPKINVALID_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
