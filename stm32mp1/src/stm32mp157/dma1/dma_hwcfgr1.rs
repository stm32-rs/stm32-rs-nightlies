#[doc = "Register `DMA_HWCFGR1` reader"]
pub type R = crate::R<DMA_HWCFGR1rs>;
#[doc = "Field `DMA_DEF0` reader - DMA_DEF0"]
pub type DMA_DEF0_R = crate::FieldReader;
#[doc = "Field `DMA_DEF1` reader - DMA_DEF1"]
pub type DMA_DEF1_R = crate::FieldReader;
#[doc = "Field `DMA_DEF2` reader - DMA_DEF2"]
pub type DMA_DEF2_R = crate::FieldReader;
#[doc = "Field `DMA_DEF3` reader - DMA_DEF3"]
pub type DMA_DEF3_R = crate::FieldReader;
#[doc = "Field `DMA_DEF4` reader - DMA_DEF4"]
pub type DMA_DEF4_R = crate::FieldReader;
#[doc = "Field `DMA_DEF5` reader - DMA_DEF5"]
pub type DMA_DEF5_R = crate::FieldReader;
#[doc = "Field `DMA_DEF6` reader - DMA_DEF6"]
pub type DMA_DEF6_R = crate::FieldReader;
#[doc = "Field `DMA_DEF7` reader - DMA_DEF7"]
pub type DMA_DEF7_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - DMA_DEF0"]
    #[inline(always)]
    pub fn dma_def0(&self) -> DMA_DEF0_R {
        DMA_DEF0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - DMA_DEF1"]
    #[inline(always)]
    pub fn dma_def1(&self) -> DMA_DEF1_R {
        DMA_DEF1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - DMA_DEF2"]
    #[inline(always)]
    pub fn dma_def2(&self) -> DMA_DEF2_R {
        DMA_DEF2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - DMA_DEF3"]
    #[inline(always)]
    pub fn dma_def3(&self) -> DMA_DEF3_R {
        DMA_DEF3_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - DMA_DEF4"]
    #[inline(always)]
    pub fn dma_def4(&self) -> DMA_DEF4_R {
        DMA_DEF4_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - DMA_DEF5"]
    #[inline(always)]
    pub fn dma_def5(&self) -> DMA_DEF5_R {
        DMA_DEF5_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - DMA_DEF6"]
    #[inline(always)]
    pub fn dma_def6(&self) -> DMA_DEF6_R {
        DMA_DEF6_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - DMA_DEF7"]
    #[inline(always)]
    pub fn dma_def7(&self) -> DMA_DEF7_R {
        DMA_DEF7_R::new(((self.bits >> 28) & 3) as u8)
    }
}
#[doc = "DMA hardware configuration 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_hwcfgr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_HWCFGR1rs;
impl crate::RegisterSpec for DMA_HWCFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_hwcfgr1::R`](R) reader structure"]
impl crate::Readable for DMA_HWCFGR1rs {}
#[doc = "`reset()` method sets DMA_HWCFGR1 to value 0x2222_2222"]
impl crate::Resettable for DMA_HWCFGR1rs {
    const RESET_VALUE: u32 = 0x2222_2222;
}
