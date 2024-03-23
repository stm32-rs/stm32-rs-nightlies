#[doc = "Register `HASH_CSR27` reader"]
pub type R = crate::R<HASH_CSR27rs>;
#[doc = "Register `HASH_CSR27` writer"]
pub type W = crate::W<HASH_CSR27rs>;
#[doc = "Field `CS27` reader - CS27"]
pub type CS27_R = crate::FieldReader<u32>;
#[doc = "Field `CS27` writer - CS27"]
pub type CS27_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS27"]
    #[inline(always)]
    pub fn cs27(&self) -> CS27_R {
        CS27_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS27"]
    #[inline(always)]
    #[must_use]
    pub fn cs27(&mut self) -> CS27_W<HASH_CSR27rs> {
        CS27_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr27::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr27::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR27rs;
impl crate::RegisterSpec for HASH_CSR27rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr27::R`](R) reader structure"]
impl crate::Readable for HASH_CSR27rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr27::W`](W) writer structure"]
impl crate::Writable for HASH_CSR27rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR27 to value 0"]
impl crate::Resettable for HASH_CSR27rs {
    const RESET_VALUE: u32 = 0;
}
