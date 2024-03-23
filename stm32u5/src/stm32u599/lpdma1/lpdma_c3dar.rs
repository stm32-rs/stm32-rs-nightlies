#[doc = "Register `LPDMA_C3DAR` reader"]
pub type R = crate::R<LPDMA_C3DARrs>;
#[doc = "Register `LPDMA_C3DAR` writer"]
pub type W = crate::W<LPDMA_C3DARrs>;
#[doc = "Field `DA` reader - destination address"]
pub type DA_R = crate::FieldReader<u32>;
#[doc = "Field `DA` writer - destination address"]
pub type DA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - destination address"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - destination address"]
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DA_W<LPDMA_C3DARrs> {
        DA_W::new(self, 0)
    }
}
#[doc = "LPDMA channel x destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c3dar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c3dar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPDMA_C3DARrs;
impl crate::RegisterSpec for LPDMA_C3DARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpdma_c3dar::R`](R) reader structure"]
impl crate::Readable for LPDMA_C3DARrs {}
#[doc = "`write(|w| ..)` method takes [`lpdma_c3dar::W`](W) writer structure"]
impl crate::Writable for LPDMA_C3DARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPDMA_C3DAR to value 0"]
impl crate::Resettable for LPDMA_C3DARrs {
    const RESET_VALUE: u32 = 0;
}
