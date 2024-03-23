#[doc = "Register `HASH_CSR40` reader"]
pub type R = crate::R<HASH_CSR40rs>;
#[doc = "Register `HASH_CSR40` writer"]
pub type W = crate::W<HASH_CSR40rs>;
#[doc = "Field `CS40` reader - CS40"]
pub type CS40_R = crate::FieldReader<u32>;
#[doc = "Field `CS40` writer - CS40"]
pub type CS40_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS40"]
    #[inline(always)]
    pub fn cs40(&self) -> CS40_R {
        CS40_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS40"]
    #[inline(always)]
    #[must_use]
    pub fn cs40(&mut self) -> CS40_W<HASH_CSR40rs> {
        CS40_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr40::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr40::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR40rs;
impl crate::RegisterSpec for HASH_CSR40rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr40::R`](R) reader structure"]
impl crate::Readable for HASH_CSR40rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr40::W`](W) writer structure"]
impl crate::Writable for HASH_CSR40rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR40 to value 0"]
impl crate::Resettable for HASH_CSR40rs {
    const RESET_VALUE: u32 = 0;
}
