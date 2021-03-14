#[doc = "Reader of register ITLINE11"]
pub type R = crate::R<u32, super::ITLINE11>;
#[doc = "Reader of field `DMAMUX`"]
pub type DMAMUX_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA1_CH4`"]
pub type DMA1_CH4_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA1_CH5`"]
pub type DMA1_CH5_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA1_CH6`"]
pub type DMA1_CH6_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA1_CH7`"]
pub type DMA1_CH7_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - DMAMUX"]
    #[inline(always)]
    pub fn dmamux(&self) -> DMAMUX_R {
        DMAMUX_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA1_CH4"]
    #[inline(always)]
    pub fn dma1_ch4(&self) -> DMA1_CH4_R {
        DMA1_CH4_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA1_CH5"]
    #[inline(always)]
    pub fn dma1_ch5(&self) -> DMA1_CH5_R {
        DMA1_CH5_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA1_CH6"]
    #[inline(always)]
    pub fn dma1_ch6(&self) -> DMA1_CH6_R {
        DMA1_CH6_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DMA1_CH7"]
    #[inline(always)]
    pub fn dma1_ch7(&self) -> DMA1_CH7_R {
        DMA1_CH7_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
