#[doc = "Reader of register MTLTxQDR"]
pub type R = crate::R<u32, super::MTLTXQDR>;
#[doc = "Reader of field `TXQPAUSED`"]
pub type TXQPAUSED_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRCSTS`"]
pub type TRCSTS_R = crate::R<u8, u8>;
#[doc = "Reader of field `TWCSTS`"]
pub type TWCSTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXQSTS`"]
pub type TXQSTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXSTSFSTS`"]
pub type TXSTSFSTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `PTXQ`"]
pub type PTXQ_R = crate::R<u8, u8>;
#[doc = "Reader of field `STXSTSF`"]
pub type STXSTSF_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Transmit Queue in Pause"]
    #[inline(always)]
    pub fn txqpaused(&self) -> TXQPAUSED_R {
        TXQPAUSED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - MTL Tx Queue Read Controller Status"]
    #[inline(always)]
    pub fn trcsts(&self) -> TRCSTS_R {
        TRCSTS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - MTL Tx Queue Write Controller Status"]
    #[inline(always)]
    pub fn twcsts(&self) -> TWCSTS_R {
        TWCSTS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MTL Tx Queue Not Empty Status"]
    #[inline(always)]
    pub fn txqsts(&self) -> TXQSTS_R {
        TXQSTS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MTL Tx Status FIFO Full Status"]
    #[inline(always)]
    pub fn txstsfsts(&self) -> TXSTSFSTS_R {
        TXSTSFSTS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Number of Packets in the Transmit Queue"]
    #[inline(always)]
    pub fn ptxq(&self) -> PTXQ_R {
        PTXQ_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Number of Status Words in Tx Status FIFO of Queue"]
    #[inline(always)]
    pub fn stxstsf(&self) -> STXSTSF_R {
        STXSTSF_R::new(((self.bits >> 20) & 0x07) as u8)
    }
}
