#[doc = "Register `HASH_CSR23` reader"]
pub type R = crate::R<HASH_CSR23rs>;
#[doc = "Register `HASH_CSR23` writer"]
pub type W = crate::W<HASH_CSR23rs>;
#[doc = "Field `CS23` reader - CS23"]
pub type CS23_R = crate::FieldReader<u32>;
#[doc = "Field `CS23` writer - CS23"]
pub type CS23_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS23"]
    #[inline(always)]
    pub fn cs23(&self) -> CS23_R {
        CS23_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS23"]
    #[inline(always)]
    #[must_use]
    pub fn cs23(&mut self) -> CS23_W<HASH_CSR23rs> {
        CS23_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR23rs;
impl crate::RegisterSpec for HASH_CSR23rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr23::R`](R) reader structure"]
impl crate::Readable for HASH_CSR23rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr23::W`](W) writer structure"]
impl crate::Writable for HASH_CSR23rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR23 to value 0"]
impl crate::Resettable for HASH_CSR23rs {
    const RESET_VALUE: u32 = 0;
}
