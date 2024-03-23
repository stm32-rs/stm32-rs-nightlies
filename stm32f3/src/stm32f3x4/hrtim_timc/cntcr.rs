#[doc = "Register `CNTCR` reader"]
pub type R = crate::R<CNTCRrs>;
#[doc = "Register `CNTCR` writer"]
pub type W = crate::W<CNTCRrs>;
#[doc = "Field `CNTx` reader - Timerx Counter value"]
pub type CNTX_R = crate::FieldReader<u16>;
#[doc = "Field `CNTx` writer - Timerx Counter value"]
pub type CNTX_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Counter value"]
    #[inline(always)]
    pub fn cntx(&self) -> CNTX_R {
        CNTX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Counter value"]
    #[inline(always)]
    #[must_use]
    pub fn cntx(&mut self) -> CNTX_W<CNTCRrs> {
        CNTX_W::new(self, 0)
    }
}
#[doc = "Timerx Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTCRrs;
impl crate::RegisterSpec for CNTCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntcr::R`](R) reader structure"]
impl crate::Readable for CNTCRrs {}
#[doc = "`write(|w| ..)` method takes [`cntcr::W`](W) writer structure"]
impl crate::Writable for CNTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNTCR to value 0"]
impl crate::Resettable for CNTCRrs {
    const RESET_VALUE: u32 = 0;
}
