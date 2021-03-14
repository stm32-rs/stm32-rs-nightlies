#[doc = "Reader of register DMADSR"]
pub type R = crate::R<u32, super::DMADSR>;
#[doc = "Reader of field `AXWHSTS`"]
pub type AXWHSTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RPS0`"]
pub type RPS0_R = crate::R<u8, u8>;
#[doc = "Reader of field `TPS0`"]
pub type TPS0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - AHB Master Write Channel"]
    #[inline(always)]
    pub fn axwhsts(&self) -> AXWHSTS_R {
        AXWHSTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - DMA Channel Receive Process State"]
    #[inline(always)]
    pub fn rps0(&self) -> RPS0_R {
        RPS0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DMA Channel Transmit Process State"]
    #[inline(always)]
    pub fn tps0(&self) -> TPS0_R {
        TPS0_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
