#[doc = "Reader of register ETH_DMADSR"]
pub type R = crate::R<u32, super::ETH_DMADSR>;
#[doc = "Reader of field `AXWHSTS`"]
pub type AXWHSTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `AXRHSTS`"]
pub type AXRHSTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RPS0`"]
pub type RPS0_R = crate::R<u8, u8>;
#[doc = "Reader of field `TPS0`"]
pub type TPS0_R = crate::R<u8, u8>;
#[doc = "Reader of field `RPS1`"]
pub type RPS1_R = crate::R<u8, u8>;
#[doc = "Reader of field `TPS1`"]
pub type TPS1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - AHB Master Write Channel"]
    #[inline(always)]
    pub fn axwhsts(&self) -> AXWHSTS_R {
        AXWHSTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - AXRHSTS"]
    #[inline(always)]
    pub fn axrhsts(&self) -> AXRHSTS_R {
        AXRHSTS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - RPS0"]
    #[inline(always)]
    pub fn rps0(&self) -> RPS0_R {
        RPS0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - TPS0"]
    #[inline(always)]
    pub fn tps0(&self) -> TPS0_R {
        TPS0_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - RPS1"]
    #[inline(always)]
    pub fn rps1(&self) -> RPS1_R {
        RPS1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - TPS1"]
    #[inline(always)]
    pub fn tps1(&self) -> TPS1_R {
        TPS1_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
