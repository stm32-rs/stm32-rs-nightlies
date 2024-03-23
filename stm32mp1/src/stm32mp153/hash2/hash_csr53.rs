#[doc = "Register `HASH_CSR53` reader"]
pub type R = crate::R<HASH_CSR53rs>;
#[doc = "Register `HASH_CSR53` writer"]
pub type W = crate::W<HASH_CSR53rs>;
#[doc = "Field `CS53` reader - CS53"]
pub type CS53_R = crate::FieldReader<u32>;
#[doc = "Field `CS53` writer - CS53"]
pub type CS53_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS53"]
    #[inline(always)]
    pub fn cs53(&self) -> CS53_R {
        CS53_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS53"]
    #[inline(always)]
    #[must_use]
    pub fn cs53(&mut self) -> CS53_W<HASH_CSR53rs> {
        CS53_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr53::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr53::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR53rs;
impl crate::RegisterSpec for HASH_CSR53rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr53::R`](R) reader structure"]
impl crate::Readable for HASH_CSR53rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr53::W`](W) writer structure"]
impl crate::Writable for HASH_CSR53rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR53 to value 0"]
impl crate::Resettable for HASH_CSR53rs {
    const RESET_VALUE: u32 = 0;
}
