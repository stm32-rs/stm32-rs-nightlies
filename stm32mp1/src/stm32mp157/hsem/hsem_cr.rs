#[doc = "Register `HSEM_CR` writer"]
pub type W = crate::W<HSEM_CRrs>;
#[doc = "Field `COREID` writer - COREID"]
pub type COREID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `KEY` writer - KEY"]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 8:11 - COREID"]
    #[inline(always)]
    #[must_use]
    pub fn coreid(&mut self) -> COREID_W<HSEM_CRrs> {
        COREID_W::new(self, 8)
    }
    #[doc = "Bits 16:31 - KEY"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<HSEM_CRrs> {
        KEY_W::new(self, 16)
    }
}
#[doc = "Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSEM_CRrs;
impl crate::RegisterSpec for HSEM_CRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hsem_cr::W`](W) writer structure"]
impl crate::Writable for HSEM_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSEM_CR to value 0"]
impl crate::Resettable for HSEM_CRrs {
    const RESET_VALUE: u32 = 0;
}
