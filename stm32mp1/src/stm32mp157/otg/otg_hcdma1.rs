#[doc = "Register `OTG_HCDMA1` reader"]
pub type R = crate::R<OTG_HCDMA1rs>;
#[doc = "Register `OTG_HCDMA1` writer"]
pub type W = crate::W<OTG_HCDMA1rs>;
#[doc = "Field `DMAADDR` reader - DMAADDR"]
pub type DMAADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR` writer - DMAADDR"]
pub type DMAADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMAADDR"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMAADDR"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<OTG_HCDMA1rs> {
        DMAADDR_W::new(self, 0)
    }
}
#[doc = "OTG host channel 1 DMA address register in buffer DMA \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdma1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcdma1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HCDMA1rs;
impl crate::RegisterSpec for OTG_HCDMA1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hcdma1::R`](R) reader structure"]
impl crate::Readable for OTG_HCDMA1rs {}
#[doc = "`write(|w| ..)` method takes [`otg_hcdma1::W`](W) writer structure"]
impl crate::Writable for OTG_HCDMA1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_HCDMA1 to value 0"]
impl crate::Resettable for OTG_HCDMA1rs {
    const RESET_VALUE: u32 = 0;
}
