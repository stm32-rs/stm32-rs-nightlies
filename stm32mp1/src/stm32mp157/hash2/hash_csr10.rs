#[doc = "Register `HASH_CSR10` reader"]
pub type R = crate::R<HASH_CSR10rs>;
#[doc = "Register `HASH_CSR10` writer"]
pub type W = crate::W<HASH_CSR10rs>;
#[doc = "Field `CS10` reader - CS10"]
pub type CS10_R = crate::FieldReader<u32>;
#[doc = "Field `CS10` writer - CS10"]
pub type CS10_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS10"]
    #[inline(always)]
    pub fn cs10(&self) -> CS10_R {
        CS10_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS10"]
    #[inline(always)]
    #[must_use]
    pub fn cs10(&mut self) -> CS10_W<HASH_CSR10rs> {
        CS10_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR10rs;
impl crate::RegisterSpec for HASH_CSR10rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr10::R`](R) reader structure"]
impl crate::Readable for HASH_CSR10rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr10::W`](W) writer structure"]
impl crate::Writable for HASH_CSR10rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR10 to value 0"]
impl crate::Resettable for HASH_CSR10rs {
    const RESET_VALUE: u32 = 0;
}
