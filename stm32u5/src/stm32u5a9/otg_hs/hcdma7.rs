#[doc = "Register `HCDMA7` reader"]
pub type R = crate::R<HCDMA7rs>;
#[doc = "Register `HCDMA7` writer"]
pub type W = crate::W<HCDMA7rs>;
#[doc = "Field `DMAADDR` reader - DMA address This field holds the start address in the external memory from which the data for the endpoint must be fetched or to which it must be stored. This register is incremented on every AHB transaction."]
pub type DMAADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR` writer - DMA address This field holds the start address in the external memory from which the data for the endpoint must be fetched or to which it must be stored. This register is incremented on every AHB transaction."]
pub type DMAADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA address This field holds the start address in the external memory from which the data for the endpoint must be fetched or to which it must be stored. This register is incremented on every AHB transaction."]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA address This field holds the start address in the external memory from which the data for the endpoint must be fetched or to which it must be stored. This register is incremented on every AHB transaction."]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<HCDMA7rs> {
        DMAADDR_W::new(self, 0)
    }
}
#[doc = "OTG host channel 7 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCDMA7rs;
impl crate::RegisterSpec for HCDMA7rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdma7::R`](R) reader structure"]
impl crate::Readable for HCDMA7rs {}
#[doc = "`write(|w| ..)` method takes [`hcdma7::W`](W) writer structure"]
impl crate::Writable for HCDMA7rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCDMA7 to value 0"]
impl crate::Resettable for HCDMA7rs {
    const RESET_VALUE: u32 = 0;
}
