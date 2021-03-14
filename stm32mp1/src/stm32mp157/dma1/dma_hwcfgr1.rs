#[doc = "Reader of register DMA_HWCFGR1"]
pub type R = crate::R<u32, super::DMA_HWCFGR1>;
#[doc = "Reader of field `DMA_DEF0`"]
pub type DMA_DEF0_R = crate::R<u8, u8>;
#[doc = "Reader of field `DMA_DEF1`"]
pub type DMA_DEF1_R = crate::R<u8, u8>;
#[doc = "Reader of field `DMA_DEF2`"]
pub type DMA_DEF2_R = crate::R<u8, u8>;
#[doc = "Reader of field `DMA_DEF3`"]
pub type DMA_DEF3_R = crate::R<u8, u8>;
#[doc = "Reader of field `DMA_DEF4`"]
pub type DMA_DEF4_R = crate::R<u8, u8>;
#[doc = "Reader of field `DMA_DEF5`"]
pub type DMA_DEF5_R = crate::R<u8, u8>;
#[doc = "Reader of field `DMA_DEF6`"]
pub type DMA_DEF6_R = crate::R<u8, u8>;
#[doc = "Reader of field `DMA_DEF7`"]
pub type DMA_DEF7_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - DMA_DEF0"]
    #[inline(always)]
    pub fn dma_def0(&self) -> DMA_DEF0_R {
        DMA_DEF0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - DMA_DEF1"]
    #[inline(always)]
    pub fn dma_def1(&self) -> DMA_DEF1_R {
        DMA_DEF1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - DMA_DEF2"]
    #[inline(always)]
    pub fn dma_def2(&self) -> DMA_DEF2_R {
        DMA_DEF2_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - DMA_DEF3"]
    #[inline(always)]
    pub fn dma_def3(&self) -> DMA_DEF3_R {
        DMA_DEF3_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - DMA_DEF4"]
    #[inline(always)]
    pub fn dma_def4(&self) -> DMA_DEF4_R {
        DMA_DEF4_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - DMA_DEF5"]
    #[inline(always)]
    pub fn dma_def5(&self) -> DMA_DEF5_R {
        DMA_DEF5_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - DMA_DEF6"]
    #[inline(always)]
    pub fn dma_def6(&self) -> DMA_DEF6_R {
        DMA_DEF6_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - DMA_DEF7"]
    #[inline(always)]
    pub fn dma_def7(&self) -> DMA_DEF7_R {
        DMA_DEF7_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
