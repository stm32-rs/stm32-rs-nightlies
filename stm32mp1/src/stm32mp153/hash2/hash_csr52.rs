#[doc = "Register `HASH_CSR52` reader"]
pub type R = crate::R<HASH_CSR52rs>;
#[doc = "Register `HASH_CSR52` writer"]
pub type W = crate::W<HASH_CSR52rs>;
#[doc = "Field `CS52` reader - CS52"]
pub type CS52_R = crate::FieldReader<u32>;
#[doc = "Field `CS52` writer - CS52"]
pub type CS52_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS52"]
    #[inline(always)]
    pub fn cs52(&self) -> CS52_R {
        CS52_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS52"]
    #[inline(always)]
    #[must_use]
    pub fn cs52(&mut self) -> CS52_W<HASH_CSR52rs> {
        CS52_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr52::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr52::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR52rs;
impl crate::RegisterSpec for HASH_CSR52rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr52::R`](R) reader structure"]
impl crate::Readable for HASH_CSR52rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr52::W`](W) writer structure"]
impl crate::Writable for HASH_CSR52rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR52 to value 0"]
impl crate::Resettable for HASH_CSR52rs {
    const RESET_VALUE: u32 = 0;
}
