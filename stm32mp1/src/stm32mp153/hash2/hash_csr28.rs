#[doc = "Register `HASH_CSR28` reader"]
pub type R = crate::R<HASH_CSR28rs>;
#[doc = "Register `HASH_CSR28` writer"]
pub type W = crate::W<HASH_CSR28rs>;
#[doc = "Field `CS28` reader - CS28"]
pub type CS28_R = crate::FieldReader<u32>;
#[doc = "Field `CS28` writer - CS28"]
pub type CS28_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS28"]
    #[inline(always)]
    pub fn cs28(&self) -> CS28_R {
        CS28_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS28"]
    #[inline(always)]
    #[must_use]
    pub fn cs28(&mut self) -> CS28_W<HASH_CSR28rs> {
        CS28_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr28::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr28::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR28rs;
impl crate::RegisterSpec for HASH_CSR28rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr28::R`](R) reader structure"]
impl crate::Readable for HASH_CSR28rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr28::W`](W) writer structure"]
impl crate::Writable for HASH_CSR28rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR28 to value 0"]
impl crate::Resettable for HASH_CSR28rs {
    const RESET_VALUE: u32 = 0;
}
