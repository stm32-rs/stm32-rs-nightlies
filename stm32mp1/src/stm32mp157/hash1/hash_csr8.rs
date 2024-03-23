#[doc = "Register `HASH_CSR8` reader"]
pub type R = crate::R<HASH_CSR8rs>;
#[doc = "Register `HASH_CSR8` writer"]
pub type W = crate::W<HASH_CSR8rs>;
#[doc = "Field `CS8` reader - CS8"]
pub type CS8_R = crate::FieldReader<u32>;
#[doc = "Field `CS8` writer - CS8"]
pub type CS8_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS8"]
    #[inline(always)]
    pub fn cs8(&self) -> CS8_R {
        CS8_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS8"]
    #[inline(always)]
    #[must_use]
    pub fn cs8(&mut self) -> CS8_W<HASH_CSR8rs> {
        CS8_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR8rs;
impl crate::RegisterSpec for HASH_CSR8rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr8::R`](R) reader structure"]
impl crate::Readable for HASH_CSR8rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr8::W`](W) writer structure"]
impl crate::Writable for HASH_CSR8rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR8 to value 0"]
impl crate::Resettable for HASH_CSR8rs {
    const RESET_VALUE: u32 = 0;
}
