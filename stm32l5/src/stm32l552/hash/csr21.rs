#[doc = "Register `CSR21` reader"]
pub type R = crate::R<CSR21rs>;
#[doc = "Register `CSR21` writer"]
pub type W = crate::W<CSR21rs>;
#[doc = "Field `CSR21` reader - CSR21"]
pub type CSR21_R = crate::FieldReader<u32>;
#[doc = "Field `CSR21` writer - CSR21"]
pub type CSR21_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR21"]
    #[inline(always)]
    pub fn csr21(&self) -> CSR21_R {
        CSR21_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR21"]
    #[inline(always)]
    #[must_use]
    pub fn csr21(&mut self) -> CSR21_W<CSR21rs> {
        CSR21_W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr21::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr21::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR21rs;
impl crate::RegisterSpec for CSR21rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr21::R`](R) reader structure"]
impl crate::Readable for CSR21rs {}
#[doc = "`write(|w| ..)` method takes [`csr21::W`](W) writer structure"]
impl crate::Writable for CSR21rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR21 to value 0"]
impl crate::Resettable for CSR21rs {
    const RESET_VALUE: u32 = 0;
}
