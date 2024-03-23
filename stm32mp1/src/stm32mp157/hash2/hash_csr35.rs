#[doc = "Register `HASH_CSR35` reader"]
pub type R = crate::R<HASH_CSR35rs>;
#[doc = "Register `HASH_CSR35` writer"]
pub type W = crate::W<HASH_CSR35rs>;
#[doc = "Field `CS35` reader - CS35"]
pub type CS35_R = crate::FieldReader<u32>;
#[doc = "Field `CS35` writer - CS35"]
pub type CS35_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS35"]
    #[inline(always)]
    pub fn cs35(&self) -> CS35_R {
        CS35_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS35"]
    #[inline(always)]
    #[must_use]
    pub fn cs35(&mut self) -> CS35_W<HASH_CSR35rs> {
        CS35_W::new(self, 0)
    }
}
#[doc = "HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr35::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr35::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CSR35rs;
impl crate::RegisterSpec for HASH_CSR35rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_csr35::R`](R) reader structure"]
impl crate::Readable for HASH_CSR35rs {}
#[doc = "`write(|w| ..)` method takes [`hash_csr35::W`](W) writer structure"]
impl crate::Writable for HASH_CSR35rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CSR35 to value 0"]
impl crate::Resettable for HASH_CSR35rs {
    const RESET_VALUE: u32 = 0;
}
