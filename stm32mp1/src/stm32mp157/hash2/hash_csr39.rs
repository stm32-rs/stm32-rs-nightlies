#[doc = "Register `HASH_CSR39` reader"]
pub type R = crate::R<HASH_CSR39rs>;
#[doc = "Register `HASH_CSR39` writer"]
pub type W = crate::W<HASH_CSR39rs>;
#[doc = "Field `CS39` reader - CS39"]
pub type CS39_R = crate::FieldReader<u32>;
#[doc = "Field `CS39` writer - CS39"]
pub type CS39_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS39"]
    #[inline(always)]
    pub fn cs39(&self) -> CS39_R {
        CS39_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS39"]
    #[inline(always)]
    #[must_use]
    pub fn cs39(&mut self) -> CS39_W<HASH_CSR39rs> {
        CS39_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr39::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr39::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR39rs;
impl crate::RegisterSpec for HASH_CSR39rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr39::R`](R) reader structure"]
impl crate::Readable for HASH_CSR39rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr39::W`](W) writer structure"]
impl crate::Writable for HASH_CSR39rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR39 to value 0"]
impl crate::Resettable for HASH_CSR39rs {
    const RESET_VALUE: u32 = 0;
}
