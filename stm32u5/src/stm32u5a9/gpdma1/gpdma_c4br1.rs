#[doc = "Register `GPDMA_C4BR1` reader"]
pub type R = crate::R<GPDMA_C4BR1rs>;
#[doc = "Register `GPDMA_C4BR1` writer"]
pub type W = crate::W<GPDMA_C4BR1rs>;
#[doc = "Field `BNDT` reader - block number of data bytes to transfer from the source"]
pub type BNDT_R = crate::FieldReader<u16>;
#[doc = "Field `BNDT` writer - block number of data bytes to transfer from the source"]
pub type BNDT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - block number of data bytes to transfer from the source"]
    #[inline(always)]
    pub fn bndt(&self) -> BNDT_R {
        BNDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - block number of data bytes to transfer from the source"]
    #[inline(always)]
    #[must_use]
    pub fn bndt(&mut self) -> BNDT_W<GPDMA_C4BR1rs> {
        BNDT_W::new(self, 0)
    }
}
#[doc = "GPDMA channel x block register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdma_c4br1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdma_c4br1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPDMA_C4BR1rs;
impl crate::RegisterSpec for GPDMA_C4BR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpdma_c4br1::R`](R) reader structure"]
impl crate::Readable for GPDMA_C4BR1rs {}
#[doc = "`write(|w| ..)` method takes [`gpdma_c4br1::W`](W) writer structure"]
impl crate::Writable for GPDMA_C4BR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPDMA_C4BR1 to value 0"]
impl crate::Resettable for GPDMA_C4BR1rs {
    const RESET_VALUE: u32 = 0;
}
