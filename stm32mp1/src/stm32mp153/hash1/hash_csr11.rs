#[doc = "Register `HASH_CSR11` reader"]
pub type R = crate::R<HASH_CSR11rs>;
#[doc = "Register `HASH_CSR11` writer"]
pub type W = crate::W<HASH_CSR11rs>;
#[doc = "Field `CS11` reader - CS11"]
pub type CS11_R = crate::FieldReader<u32>;
#[doc = "Field `CS11` writer - CS11"]
pub type CS11_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS11"]
    #[inline(always)]
    pub fn cs11(&self) -> CS11_R {
        CS11_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS11"]
    #[inline(always)]
    #[must_use]
    pub fn cs11(&mut self) -> CS11_W<HASH_CSR11rs> {
        CS11_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR11rs;
impl crate::RegisterSpec for HASH_CSR11rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr11::R`](R) reader structure"]
impl crate::Readable for HASH_CSR11rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr11::W`](W) writer structure"]
impl crate::Writable for HASH_CSR11rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR11 to value 0"]
impl crate::Resettable for HASH_CSR11rs {
    const RESET_VALUE: u32 = 0;
}
