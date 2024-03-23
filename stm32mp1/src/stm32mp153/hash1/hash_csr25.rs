#[doc = "Register `HASH_CSR25` reader"]
pub type R = crate::R<HASH_CSR25rs>;
#[doc = "Register `HASH_CSR25` writer"]
pub type W = crate::W<HASH_CSR25rs>;
#[doc = "Field `CS25` reader - CS25"]
pub type CS25_R = crate::FieldReader<u32>;
#[doc = "Field `CS25` writer - CS25"]
pub type CS25_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS25"]
    #[inline(always)]
    pub fn cs25(&self) -> CS25_R {
        CS25_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS25"]
    #[inline(always)]
    #[must_use]
    pub fn cs25(&mut self) -> CS25_W<HASH_CSR25rs> {
        CS25_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr25::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr25::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR25rs;
impl crate::RegisterSpec for HASH_CSR25rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr25::R`](R) reader structure"]
impl crate::Readable for HASH_CSR25rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr25::W`](W) writer structure"]
impl crate::Writable for HASH_CSR25rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR25 to value 0"]
impl crate::Resettable for HASH_CSR25rs {
    const RESET_VALUE: u32 = 0;
}
