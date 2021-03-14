#[doc = "Reader of register ISR"]
pub type R = crate::R<u32, super::ISR>;
#[doc = "Reader of field `RXBFF`"]
pub type RXBFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXBEF`"]
pub type TXBEF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXBERF`"]
pub type RXBERF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXOVRF`"]
pub type RXOVRF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXUNRF`"]
pub type TXUNRF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXNE`"]
pub type RXNE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXE`"]
pub type TXE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCF`"]
pub type TCF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRF`"]
pub type SRF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUSP`"]
pub type SUSP_R = crate::R<bool, bool>;
#[doc = "Reader of field `DEACTF`"]
pub type DEACTF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Receive buffer full flag"]
    #[inline(always)]
    pub fn rxbff(&self) -> RXBFF_R {
        RXBFF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit buffer empty flag"]
    #[inline(always)]
    pub fn txbef(&self) -> TXBEF_R {
        TXBEF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive CRC error flag"]
    #[inline(always)]
    pub fn rxberf(&self) -> RXBERF_R {
        RXBERF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive overrun error flag"]
    #[inline(always)]
    pub fn rxovrf(&self) -> RXOVRF_R {
        RXOVRF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit underrun error flag"]
    #[inline(always)]
    pub fn txunrf(&self) -> TXUNRF_R {
        TXUNRF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive data register not empty"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmit data register empty"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transfer complete flag"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Slave resume flag"]
    #[inline(always)]
    pub fn srf(&self) -> SRF_R {
        SRF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SUSPEND flag"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DEACTIVATED flag"]
    #[inline(always)]
    pub fn deactf(&self) -> DEACTF_R {
        DEACTF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
