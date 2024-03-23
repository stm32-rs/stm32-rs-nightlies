#[doc = "Register `HASH_CSR4` reader"]
pub type R = crate::R<HASH_CSR4rs>;
#[doc = "Register `HASH_CSR4` writer"]
pub type W = crate::W<HASH_CSR4rs>;
#[doc = "Field `CS4` reader - CS4"]
pub type CS4_R = crate::FieldReader<u32>;
#[doc = "Field `CS4` writer - CS4"]
pub type CS4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS4"]
    #[inline(always)]
    pub fn cs4(&self) -> CS4_R {
        CS4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS4"]
    #[inline(always)]
    #[must_use]
    pub fn cs4(&mut self) -> CS4_W<HASH_CSR4rs> {
        CS4_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR4rs;
impl crate::RegisterSpec for HASH_CSR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr4::R`](R) reader structure"]
impl crate::Readable for HASH_CSR4rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr4::W`](W) writer structure"]
impl crate::Writable for HASH_CSR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR4 to value 0"]
impl crate::Resettable for HASH_CSR4rs {
    const RESET_VALUE: u32 = 0;
}
