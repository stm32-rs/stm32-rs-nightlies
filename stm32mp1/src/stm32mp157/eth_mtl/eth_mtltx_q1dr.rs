#[doc = "Reader of register ETH_MTLTxQ1DR"]
pub type R = crate::R<u32, super::ETH_MTLTXQ1DR>;
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
    #[doc = "Bit 0 - TXQPAUSED"]
    #[inline(always)]
    pub fn txqpaused(&self) -> TXQPAUSED_R {
        TXQPAUSED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - TRCSTS"]
    #[inline(always)]
    pub fn trcsts(&self) -> TRCSTS_R {
        TRCSTS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - TWCSTS"]
    #[inline(always)]
    pub fn twcsts(&self) -> TWCSTS_R {
        TWCSTS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TXQSTS"]
    #[inline(always)]
    pub fn txqsts(&self) -> TXQSTS_R {
        TXQSTS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TXSTSFSTS"]
    #[inline(always)]
    pub fn txstsfsts(&self) -> TXSTSFSTS_R {
        TXSTSFSTS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - PTXQ"]
    #[inline(always)]
    pub fn ptxq(&self) -> PTXQ_R {
        PTXQ_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - STXSTSF"]
    #[inline(always)]
    pub fn stxstsf(&self) -> STXSTSF_R {
        STXSTSF_R::new(((self.bits >> 20) & 0x07) as u8)
    }
}
