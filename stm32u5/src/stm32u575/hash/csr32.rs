#[doc = "Register `CSR32` reader"]
pub type R = crate::R<CSR32rs>;
#[doc = "Register `CSR32` writer"]
pub type W = crate::W<CSR32rs>;
#[doc = "Field `CSR32` reader - CSR32"]
pub type CSR32_R = crate::FieldReader<u32>;
#[doc = "Field `CSR32` writer - CSR32"]
pub type CSR32_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR32"]
    #[inline(always)]
    pub fn csr32(&self) -> CSR32_R {
        CSR32_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR32"]
    #[inline(always)]
    #[must_use]
    pub fn csr32(&mut self) -> CSR32_W<CSR32rs> {
        CSR32_W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR32rs;
impl crate::RegisterSpec for CSR32rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr32::R`](R) reader structure"]
impl crate::Readable for CSR32rs {}
#[doc = "`write(|w| ..)` method takes [`csr32::W`](W) writer structure"]
impl crate::Writable for CSR32rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR32 to value 0"]
impl crate::Resettable for CSR32rs {
    const RESET_VALUE: u32 = 0;
}
