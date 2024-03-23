#[doc = "Register `BDMADR` reader"]
pub type R = crate::R<BDMADRrs>;
#[doc = "Register `BDMADR` writer"]
pub type W = crate::W<BDMADRrs>;
#[doc = "Field `BDMADR` reader - Burst DMA Data register"]
pub type BDMADR_R = crate::FieldReader<u32>;
#[doc = "Field `BDMADR` writer - Burst DMA Data register"]
pub type BDMADR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Burst DMA Data register"]
    #[inline(always)]
    pub fn bdmadr(&self) -> BDMADR_R {
        BDMADR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Burst DMA Data register"]
    #[inline(always)]
    #[must_use]
    pub fn bdmadr(&mut self) -> BDMADR_W<BDMADRrs> {
        BDMADR_W::new(self, 0)
    }
}
#[doc = "Burst DMA Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdmadr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdmadr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BDMADRrs;
impl crate::RegisterSpec for BDMADRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdmadr::R`](R) reader structure"]
impl crate::Readable for BDMADRrs {}
#[doc = "`write(|w| ..)` method takes [`bdmadr::W`](W) writer structure"]
impl crate::Writable for BDMADRrs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BDMADR to value 0"]
impl crate::Resettable for BDMADRrs {
    const RESET_VALUE: u32 = 0;
}
