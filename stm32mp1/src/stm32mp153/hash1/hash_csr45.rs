#[doc = "Register `HASH_CSR45` reader"]
pub type R = crate::R<HASH_CSR45rs>;
#[doc = "Register `HASH_CSR45` writer"]
pub type W = crate::W<HASH_CSR45rs>;
#[doc = "Field `CS45` reader - CS45"]
pub type CS45_R = crate::FieldReader<u32>;
#[doc = "Field `CS45` writer - CS45"]
pub type CS45_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS45"]
    #[inline(always)]
    pub fn cs45(&self) -> CS45_R {
        CS45_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS45"]
    #[inline(always)]
    #[must_use]
    pub fn cs45(&mut self) -> CS45_W<HASH_CSR45rs> {
        CS45_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr45::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr45::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR45rs;
impl crate::RegisterSpec for HASH_CSR45rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr45::R`](R) reader structure"]
impl crate::Readable for HASH_CSR45rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr45::W`](W) writer structure"]
impl crate::Writable for HASH_CSR45rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR45 to value 0"]
impl crate::Resettable for HASH_CSR45rs {
    const RESET_VALUE: u32 = 0;
}
