#[doc = "Reader of register ITLINE10"]
pub type R = crate::R<u32, super::ITLINE10>;
#[doc = "Reader of field `DMA1_CH2`"]
pub type DMA1_CH2_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA1_CH3`"]
pub type DMA1_CH3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - DMA1_CH1"]
    #[inline(always)]
    pub fn dma1_ch2(&self) -> DMA1_CH2_R {
        DMA1_CH2_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA1_CH3"]
    #[inline(always)]
    pub fn dma1_ch3(&self) -> DMA1_CH3_R {
        DMA1_CH3_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
