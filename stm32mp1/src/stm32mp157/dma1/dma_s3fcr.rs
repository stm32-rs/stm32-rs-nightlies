#[doc = "Register `DMA_S3FCR` reader"]
pub type R = crate::R<DMA_S3FCRrs>;
#[doc = "Register `DMA_S3FCR` writer"]
pub type W = crate::W<DMA_S3FCRrs>;
#[doc = "Field `FTH` reader - FTH"]
pub type FTH_R = crate::FieldReader;
#[doc = "Field `FTH` writer - FTH"]
pub type FTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMDIS` reader - DMDIS"]
pub type DMDIS_R = crate::BitReader;
#[doc = "Field `DMDIS` writer - DMDIS"]
pub type DMDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FS` reader - FS"]
pub type FS_R = crate::FieldReader;
#[doc = "Field `FEIE` reader - FEIE"]
pub type FEIE_R = crate::BitReader;
#[doc = "Field `FEIE` writer - FEIE"]
pub type FEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - FTH"]
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - DMDIS"]
    #[inline(always)]
    pub fn dmdis(&self) -> DMDIS_R {
        DMDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - FS"]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - FEIE"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FTH"]
    #[inline(always)]
    #[must_use]
    pub fn fth(&mut self) -> FTH_W<DMA_S3FCRrs> {
        FTH_W::new(self, 0)
    }
    #[doc = "Bit 2 - DMDIS"]
    #[inline(always)]
    #[must_use]
    pub fn dmdis(&mut self) -> DMDIS_W<DMA_S3FCRrs> {
        DMDIS_W::new(self, 2)
    }
    #[doc = "Bit 7 - FEIE"]
    #[inline(always)]
    #[must_use]
    pub fn feie(&mut self) -> FEIE_W<DMA_S3FCRrs> {
        FEIE_W::new(self, 7)
    }
}
#[doc = "DMA stream 3 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s3fcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s3fcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_S3FCRrs;
impl crate::RegisterSpec for DMA_S3FCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_s3fcr::R`](R) reader structure"]
impl crate::Readable for DMA_S3FCRrs {}
#[doc = "`write(|w| ..)` method takes [`dma_s3fcr::W`](W) writer structure"]
impl crate::Writable for DMA_S3FCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_S3FCR to value 0x21"]
impl crate::Resettable for DMA_S3FCRrs {
    const RESET_VALUE: u32 = 0x21;
}
