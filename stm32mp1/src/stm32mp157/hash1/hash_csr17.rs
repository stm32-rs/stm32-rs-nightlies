#[doc = "Register `HASH_CSR17` reader"]
pub type R = crate::R<HASH_CSR17rs>;
#[doc = "Register `HASH_CSR17` writer"]
pub type W = crate::W<HASH_CSR17rs>;
#[doc = "Field `CS17` reader - CS17"]
pub type CS17_R = crate::FieldReader<u32>;
#[doc = "Field `CS17` writer - CS17"]
pub type CS17_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS17"]
    #[inline(always)]
    pub fn cs17(&self) -> CS17_R {
        CS17_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS17"]
    #[inline(always)]
    #[must_use]
    pub fn cs17(&mut self) -> CS17_W<HASH_CSR17rs> {
        CS17_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr17::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr17::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR17rs;
impl crate::RegisterSpec for HASH_CSR17rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr17::R`](R) reader structure"]
impl crate::Readable for HASH_CSR17rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr17::W`](W) writer structure"]
impl crate::Writable for HASH_CSR17rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR17 to value 0"]
impl crate::Resettable for HASH_CSR17rs {
    const RESET_VALUE: u32 = 0;
}
