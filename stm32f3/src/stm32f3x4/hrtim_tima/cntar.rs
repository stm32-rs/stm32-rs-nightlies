#[doc = "Register `CNTAR` reader"]
pub type R = crate::R<CNTARrs>;
#[doc = "Register `CNTAR` writer"]
pub type W = crate::W<CNTARrs>;
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
    pub fn cntx(&mut self) -> CNTX_W<CNTARrs> {
        CNTX_W::new(self, 0)
    }
}
#[doc = "Timerx Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTARrs;
impl crate::RegisterSpec for CNTARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntar::R`](R) reader structure"]
impl crate::Readable for CNTARrs {}
#[doc = "`write(|w| ..)` method takes [`cntar::W`](W) writer structure"]
impl crate::Writable for CNTARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNTAR to value 0"]
impl crate::Resettable for CNTARrs {
    const RESET_VALUE: u32 = 0;
}
