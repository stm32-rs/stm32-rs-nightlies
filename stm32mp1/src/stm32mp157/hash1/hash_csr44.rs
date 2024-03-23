#[doc = "Register `HASH_CSR44` reader"]
pub type R = crate::R<HASH_CSR44rs>;
#[doc = "Register `HASH_CSR44` writer"]
pub type W = crate::W<HASH_CSR44rs>;
#[doc = "Field `CS44` reader - CS44"]
pub type CS44_R = crate::FieldReader<u32>;
#[doc = "Field `CS44` writer - CS44"]
pub type CS44_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS44"]
    #[inline(always)]
    pub fn cs44(&self) -> CS44_R {
        CS44_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS44"]
    #[inline(always)]
    #[must_use]
    pub fn cs44(&mut self) -> CS44_W<HASH_CSR44rs> {
        CS44_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr44::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr44::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR44rs;
impl crate::RegisterSpec for HASH_CSR44rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr44::R`](R) reader structure"]
impl crate::Readable for HASH_CSR44rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr44::W`](W) writer structure"]
impl crate::Writable for HASH_CSR44rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR44 to value 0"]
impl crate::Resettable for HASH_CSR44rs {
    const RESET_VALUE: u32 = 0;
}
