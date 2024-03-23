#[doc = "Register `ITLINE11` reader"]
pub type R = crate::R<ITLINE11rs>;
#[doc = "Field `DMAMUX` reader - DMAMUX"]
pub type DMAMUX_R = crate::BitReader;
#[doc = "Field `DMA1_CH4` reader - DMA1_CH4"]
pub type DMA1_CH4_R = crate::BitReader;
#[doc = "Field `DMA1_CH5` reader - DMA1_CH5"]
pub type DMA1_CH5_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DMAMUX"]
    #[inline(always)]
    pub fn dmamux(&self) -> DMAMUX_R {
        DMAMUX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA1_CH4"]
    #[inline(always)]
    pub fn dma1_ch4(&self) -> DMA1_CH4_R {
        DMA1_CH4_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA1_CH5"]
    #[inline(always)]
    pub fn dma1_ch5(&self) -> DMA1_CH5_R {
        DMA1_CH5_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "interrupt line 11 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline11::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE11rs;
impl crate::RegisterSpec for ITLINE11rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline11::R`](R) reader structure"]
impl crate::Readable for ITLINE11rs {}
#[doc = "`reset()` method sets ITLINE11 to value 0"]
impl crate::Resettable for ITLINE11rs {
    const RESET_VALUE: u32 = 0;
}
