#[doc = "Register `HASH_CSR21` reader"]
pub type R = crate::R<HASH_CSR21rs>;
#[doc = "Register `HASH_CSR21` writer"]
pub type W = crate::W<HASH_CSR21rs>;
#[doc = "Field `CS21` reader - CS21"]
pub type CS21_R = crate::FieldReader<u32>;
#[doc = "Field `CS21` writer - CS21"]
pub type CS21_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS21"]
    #[inline(always)]
    pub fn cs21(&self) -> CS21_R {
        CS21_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS21"]
    #[inline(always)]
    #[must_use]
    pub fn cs21(&mut self) -> CS21_W<HASH_CSR21rs> {
        CS21_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr21::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr21::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR21rs;
impl crate::RegisterSpec for HASH_CSR21rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr21::R`](R) reader structure"]
impl crate::Readable for HASH_CSR21rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr21::W`](W) writer structure"]
impl crate::Writable for HASH_CSR21rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR21 to value 0"]
impl crate::Resettable for HASH_CSR21rs {
    const RESET_VALUE: u32 = 0;
}
