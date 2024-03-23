#[doc = "Register `HASH_CSR18` reader"]
pub type R = crate::R<HASH_CSR18rs>;
#[doc = "Register `HASH_CSR18` writer"]
pub type W = crate::W<HASH_CSR18rs>;
#[doc = "Field `CS18` reader - CS18"]
pub type CS18_R = crate::FieldReader<u32>;
#[doc = "Field `CS18` writer - CS18"]
pub type CS18_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS18"]
    #[inline(always)]
    pub fn cs18(&self) -> CS18_R {
        CS18_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS18"]
    #[inline(always)]
    #[must_use]
    pub fn cs18(&mut self) -> CS18_W<HASH_CSR18rs> {
        CS18_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr18::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr18::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR18rs;
impl crate::RegisterSpec for HASH_CSR18rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr18::R`](R) reader structure"]
impl crate::Readable for HASH_CSR18rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr18::W`](W) writer structure"]
impl crate::Writable for HASH_CSR18rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR18 to value 0"]
impl crate::Resettable for HASH_CSR18rs {
    const RESET_VALUE: u32 = 0;
}
