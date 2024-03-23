#[doc = "Register `HASH_CSR49` reader"]
pub type R = crate::R<HASH_CSR49rs>;
#[doc = "Register `HASH_CSR49` writer"]
pub type W = crate::W<HASH_CSR49rs>;
#[doc = "Field `CS49` reader - CS49"]
pub type CS49_R = crate::FieldReader<u32>;
#[doc = "Field `CS49` writer - CS49"]
pub type CS49_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS49"]
    #[inline(always)]
    pub fn cs49(&self) -> CS49_R {
        CS49_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS49"]
    #[inline(always)]
    #[must_use]
    pub fn cs49(&mut self) -> CS49_W<HASH_CSR49rs> {
        CS49_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr49::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr49::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR49rs;
impl crate::RegisterSpec for HASH_CSR49rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr49::R`](R) reader structure"]
impl crate::Readable for HASH_CSR49rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr49::W`](W) writer structure"]
impl crate::Writable for HASH_CSR49rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR49 to value 0"]
impl crate::Resettable for HASH_CSR49rs {
    const RESET_VALUE: u32 = 0;
}
