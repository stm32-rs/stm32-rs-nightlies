#[doc = "Reader of register ITLINE9"]
pub type R = crate::R<u32, super::ITLINE9>;
#[doc = "Reader of field `DMA1_CH1`"]
pub type DMA1_CH1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - DMA1_CH1"]
    #[inline(always)]
    pub fn dma1_ch1(&self) -> DMA1_CH1_R {
        DMA1_CH1_R::new((self.bits & 0x01) != 0)
    }
}
