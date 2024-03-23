#[doc = "Register `HASH_CSR16` reader"]
pub type R = crate::R<HASH_CSR16rs>;
#[doc = "Register `HASH_CSR16` writer"]
pub type W = crate::W<HASH_CSR16rs>;
#[doc = "Field `CS16` reader - CS16"]
pub type CS16_R = crate::FieldReader<u32>;
#[doc = "Field `CS16` writer - CS16"]
pub type CS16_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS16"]
    #[inline(always)]
    pub fn cs16(&self) -> CS16_R {
        CS16_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS16"]
    #[inline(always)]
    #[must_use]
    pub fn cs16(&mut self) -> CS16_W<HASH_CSR16rs> {
        CS16_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR16rs;
impl crate::RegisterSpec for HASH_CSR16rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr16::R`](R) reader structure"]
impl crate::Readable for HASH_CSR16rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr16::W`](W) writer structure"]
impl crate::Writable for HASH_CSR16rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR16 to value 0"]
impl crate::Resettable for HASH_CSR16rs {
    const RESET_VALUE: u32 = 0;
}
