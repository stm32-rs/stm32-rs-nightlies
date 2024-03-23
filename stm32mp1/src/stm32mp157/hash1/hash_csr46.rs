#[doc = "Register `HASH_CSR46` reader"]
pub type R = crate::R<HASH_CSR46rs>;
#[doc = "Register `HASH_CSR46` writer"]
pub type W = crate::W<HASH_CSR46rs>;
#[doc = "Field `CS46` reader - CS46"]
pub type CS46_R = crate::FieldReader<u32>;
#[doc = "Field `CS46` writer - CS46"]
pub type CS46_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS46"]
    #[inline(always)]
    pub fn cs46(&self) -> CS46_R {
        CS46_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS46"]
    #[inline(always)]
    #[must_use]
    pub fn cs46(&mut self) -> CS46_W<HASH_CSR46rs> {
        CS46_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr46::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr46::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR46rs;
impl crate::RegisterSpec for HASH_CSR46rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr46::R`](R) reader structure"]
impl crate::Readable for HASH_CSR46rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr46::W`](W) writer structure"]
impl crate::Writable for HASH_CSR46rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR46 to value 0"]
impl crate::Resettable for HASH_CSR46rs {
    const RESET_VALUE: u32 = 0;
}
