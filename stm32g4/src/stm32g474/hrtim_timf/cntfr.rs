#[doc = "Register `CNTFR` reader"]
pub type R = crate::R<CNTFRrs>;
#[doc = "Register `CNTFR` writer"]
pub type W = crate::W<CNTFRrs>;
#[doc = "Field `CNTx` reader - Timerx Counter value"]
pub type CNTX_R = crate::FieldReader<u16>;
#[doc = "Field `CNTx` writer - Timerx Counter value"]
pub type CNTX_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
    pub fn cntx(&mut self) -> CNTX_W<CNTFRrs> {
        CNTX_W::new(self, 0)
    }
}
#[doc = "Timerx Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTFRrs;
impl crate::RegisterSpec for CNTFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntfr::R`](R) reader structure"]
impl crate::Readable for CNTFRrs {}
#[doc = "`write(|w| ..)` method takes [`cntfr::W`](W) writer structure"]
impl crate::Writable for CNTFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNTFR to value 0"]
impl crate::Resettable for CNTFRrs {
    const RESET_VALUE: u32 = 0;
}
