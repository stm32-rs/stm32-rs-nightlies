#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `COREID` writer - COREID"]
pub type COREID_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `KEY` writer - Semaphore clear Key"]
pub type KEY_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 8:11 - COREID"]
    #[inline(always)]
    #[must_use]
    pub fn coreid(&mut self) -> COREID_W<CRrs> {
        COREID_W::new(self, 8)
    }
    #[doc = "Bits 16:31 - Semaphore clear Key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<CRrs> {
        KEY_W::new(self, 16)
    }
}
#[doc = "HSEM Clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
