#[doc = "Reader of register ETH_MACTxTSSNR"]
pub type R = crate::R<u32, super::ETH_MACTXTSSNR>;
#[doc = "Reader of field `TXTSSLO`"]
pub type TXTSSLO_R = crate::R<u32, u32>;
#[doc = "Reader of field `TXTSSMIS`"]
pub type TXTSSMIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:30 - TXTSSLO"]
    #[inline(always)]
    pub fn txtsslo(&self) -> TXTSSLO_R {
        TXTSSLO_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - TXTSSMIS"]
    #[inline(always)]
    pub fn txtssmis(&self) -> TXTSSMIS_R {
        TXTSSMIS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
