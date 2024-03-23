#[doc = "Register `HASH_CSR15` reader"]
pub type R = crate::R<HASH_CSR15rs>;
#[doc = "Register `HASH_CSR15` writer"]
pub type W = crate::W<HASH_CSR15rs>;
#[doc = "Field `CS15` reader - CS15"]
pub type CS15_R = crate::FieldReader<u32>;
#[doc = "Field `CS15` writer - CS15"]
pub type CS15_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS15"]
    #[inline(always)]
    pub fn cs15(&self) -> CS15_R {
        CS15_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS15"]
    #[inline(always)]
    #[must_use]
    pub fn cs15(&mut self) -> CS15_W<HASH_CSR15rs> {
        CS15_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR15rs;
impl crate::RegisterSpec for HASH_CSR15rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr15::R`](R) reader structure"]
impl crate::Readable for HASH_CSR15rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr15::W`](W) writer structure"]
impl crate::Writable for HASH_CSR15rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR15 to value 0"]
impl crate::Resettable for HASH_CSR15rs {
    const RESET_VALUE: u32 = 0;
}
