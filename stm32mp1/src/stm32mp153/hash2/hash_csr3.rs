#[doc = "Register `HASH_CSR3` reader"]
pub type R = crate::R<HASH_CSR3rs>;
#[doc = "Register `HASH_CSR3` writer"]
pub type W = crate::W<HASH_CSR3rs>;
#[doc = "Field `CS3` reader - CS3"]
pub type CS3_R = crate::FieldReader<u32>;
#[doc = "Field `CS3` writer - CS3"]
pub type CS3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS3"]
    #[inline(always)]
    pub fn cs3(&self) -> CS3_R {
        CS3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS3"]
    #[inline(always)]
    #[must_use]
    pub fn cs3(&mut self) -> CS3_W<HASH_CSR3rs> {
        CS3_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR3rs;
impl crate::RegisterSpec for HASH_CSR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr3::R`](R) reader structure"]
impl crate::Readable for HASH_CSR3rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr3::W`](W) writer structure"]
impl crate::Writable for HASH_CSR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR3 to value 0"]
impl crate::Resettable for HASH_CSR3rs {
    const RESET_VALUE: u32 = 0;
}
