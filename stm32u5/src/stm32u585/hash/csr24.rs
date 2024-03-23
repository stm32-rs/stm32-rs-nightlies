#[doc = "Register `CSR24` reader"]
pub type R = crate::R<CSR24rs>;
#[doc = "Register `CSR24` writer"]
pub type W = crate::W<CSR24rs>;
#[doc = "Field `CSR24` reader - CSR24"]
pub type CSR24_R = crate::FieldReader<u32>;
#[doc = "Field `CSR24` writer - CSR24"]
pub type CSR24_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR24"]
    #[inline(always)]
    pub fn csr24(&self) -> CSR24_R {
        CSR24_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR24"]
    #[inline(always)]
    #[must_use]
    pub fn csr24(&mut self) -> CSR24_W<CSR24rs> {
        CSR24_W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr24::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr24::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR24rs;
impl crate::RegisterSpec for CSR24rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr24::R`](R) reader structure"]
impl crate::Readable for CSR24rs {}
#[doc = "`write(|w| ..)` method takes [`csr24::W`](W) writer structure"]
impl crate::Writable for CSR24rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR24 to value 0"]
impl crate::Resettable for CSR24rs {
    const RESET_VALUE: u32 = 0;
}
