#[doc = "Register `HASH_CSR38` reader"]
pub type R = crate::R<HASH_CSR38rs>;
#[doc = "Register `HASH_CSR38` writer"]
pub type W = crate::W<HASH_CSR38rs>;
#[doc = "Field `CS38` reader - CS38"]
pub type CS38_R = crate::FieldReader<u32>;
#[doc = "Field `CS38` writer - CS38"]
pub type CS38_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS38"]
    #[inline(always)]
    pub fn cs38(&self) -> CS38_R {
        CS38_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS38"]
    #[inline(always)]
    #[must_use]
    pub fn cs38(&mut self) -> CS38_W<HASH_CSR38rs> {
        CS38_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr38::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr38::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR38rs;
impl crate::RegisterSpec for HASH_CSR38rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr38::R`](R) reader structure"]
impl crate::Readable for HASH_CSR38rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr38::W`](W) writer structure"]
impl crate::Writable for HASH_CSR38rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR38 to value 0"]
impl crate::Resettable for HASH_CSR38rs {
    const RESET_VALUE: u32 = 0;
}
