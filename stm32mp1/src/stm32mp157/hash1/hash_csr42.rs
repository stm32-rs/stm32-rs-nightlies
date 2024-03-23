#[doc = "Register `HASH_CSR42` reader"]
pub type R = crate::R<HASH_CSR42rs>;
#[doc = "Register `HASH_CSR42` writer"]
pub type W = crate::W<HASH_CSR42rs>;
#[doc = "Field `CS42` reader - CS42"]
pub type CS42_R = crate::FieldReader<u32>;
#[doc = "Field `CS42` writer - CS42"]
pub type CS42_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS42"]
    #[inline(always)]
    pub fn cs42(&self) -> CS42_R {
        CS42_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS42"]
    #[inline(always)]
    #[must_use]
    pub fn cs42(&mut self) -> CS42_W<HASH_CSR42rs> {
        CS42_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr42::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr42::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR42rs;
impl crate::RegisterSpec for HASH_CSR42rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr42::R`](R) reader structure"]
impl crate::Readable for HASH_CSR42rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr42::W`](W) writer structure"]
impl crate::Writable for HASH_CSR42rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR42 to value 0"]
impl crate::Resettable for HASH_CSR42rs {
    const RESET_VALUE: u32 = 0;
}
