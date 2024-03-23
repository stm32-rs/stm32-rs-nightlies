#[doc = "Register `HASH_CSR7` reader"]
pub type R = crate::R<HASH_CSR7rs>;
#[doc = "Register `HASH_CSR7` writer"]
pub type W = crate::W<HASH_CSR7rs>;
#[doc = "Field `CS7` reader - CS7"]
pub type CS7_R = crate::FieldReader<u32>;
#[doc = "Field `CS7` writer - CS7"]
pub type CS7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS7"]
    #[inline(always)]
    pub fn cs7(&self) -> CS7_R {
        CS7_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS7"]
    #[inline(always)]
    #[must_use]
    pub fn cs7(&mut self) -> CS7_W<HASH_CSR7rs> {
        CS7_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR7rs;
impl crate::RegisterSpec for HASH_CSR7rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr7::R`](R) reader structure"]
impl crate::Readable for HASH_CSR7rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr7::W`](W) writer structure"]
impl crate::Writable for HASH_CSR7rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR7 to value 0"]
impl crate::Resettable for HASH_CSR7rs {
    const RESET_VALUE: u32 = 0;
}
