#[doc = "Register `HASH_CSR22` reader"]
pub type R = crate::R<HASH_CSR22rs>;
#[doc = "Register `HASH_CSR22` writer"]
pub type W = crate::W<HASH_CSR22rs>;
#[doc = "Field `CS22` reader - CS22"]
pub type CS22_R = crate::FieldReader<u32>;
#[doc = "Field `CS22` writer - CS22"]
pub type CS22_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS22"]
    #[inline(always)]
    pub fn cs22(&self) -> CS22_R {
        CS22_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS22"]
    #[inline(always)]
    #[must_use]
    pub fn cs22(&mut self) -> CS22_W<HASH_CSR22rs> {
        CS22_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr22::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr22::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR22rs;
impl crate::RegisterSpec for HASH_CSR22rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr22::R`](R) reader structure"]
impl crate::Readable for HASH_CSR22rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr22::W`](W) writer structure"]
impl crate::Writable for HASH_CSR22rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR22 to value 0"]
impl crate::Resettable for HASH_CSR22rs {
    const RESET_VALUE: u32 = 0;
}
