#[doc = "Reader of register MACDR"]
pub type R = crate::R<u32, super::MACDR>;
#[doc = "Reader of field `RPESTS`"]
pub type RPESTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RFCFCSTS`"]
pub type RFCFCSTS_R = crate::R<u8, u8>;
#[doc = "Reader of field `TPESTS`"]
pub type TPESTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFCSTS`"]
pub type TFCSTS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - MAC MII Receive Protocol Engine Status"]
    #[inline(always)]
    pub fn rpests(&self) -> RPESTS_R {
        RPESTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - MAC Receive Packet Controller FIFO Status"]
    #[inline(always)]
    pub fn rfcfcsts(&self) -> RFCFCSTS_R {
        RFCFCSTS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 16 - MAC MII Transmit Protocol Engine Status"]
    #[inline(always)]
    pub fn tpests(&self) -> TPESTS_R {
        TPESTS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - MAC Transmit Packet Controller Status"]
    #[inline(always)]
    pub fn tfcsts(&self) -> TFCSTS_R {
        TFCSTS_R::new(((self.bits >> 17) & 0x03) as u8)
    }
}
