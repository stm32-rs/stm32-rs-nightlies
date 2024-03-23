#[doc = "Register `CKDIV` reader"]
pub type R = crate::R<CKDIVrs>;
#[doc = "Register `CKDIV` writer"]
pub type W = crate::W<CKDIVrs>;
#[doc = "Field `PDIV` reader - input clock divider. the APB clock could be divided prior to be used by the CAN sub"]
pub type PDIV_R = crate::FieldReader;
#[doc = "Field `PDIV` writer - input clock divider. the APB clock could be divided prior to be used by the CAN sub"]
pub type PDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - input clock divider. the APB clock could be divided prior to be used by the CAN sub"]
    #[inline(always)]
    pub fn pdiv(&self) -> PDIV_R {
        PDIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - input clock divider. the APB clock could be divided prior to be used by the CAN sub"]
    #[inline(always)]
    #[must_use]
    pub fn pdiv(&mut self) -> PDIV_W<CKDIVrs> {
        PDIV_W::new(self, 0)
    }
}
#[doc = "FDCAN CFG clock divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CKDIVrs;
impl crate::RegisterSpec for CKDIVrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckdiv::R`](R) reader structure"]
impl crate::Readable for CKDIVrs {}
#[doc = "`write(|w| ..)` method takes [`ckdiv::W`](W) writer structure"]
impl crate::Writable for CKDIVrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CKDIV to value 0"]
impl crate::Resettable for CKDIVrs {
    const RESET_VALUE: u32 = 0;
}
