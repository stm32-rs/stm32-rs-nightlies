#[doc = "Register `CSR53` reader"]
pub type R = crate::R<CSR53rs>;
#[doc = "Register `CSR53` writer"]
pub type W = crate::W<CSR53rs>;
#[doc = "Field `CSR53` reader - CSR53"]
pub type CSR53_R = crate::FieldReader<u32>;
#[doc = "Field `CSR53` writer - CSR53"]
pub type CSR53_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR53"]
    #[inline(always)]
    pub fn csr53(&self) -> CSR53_R {
        CSR53_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR53"]
    #[inline(always)]
    #[must_use]
    pub fn csr53(&mut self) -> CSR53_W<CSR53rs> {
        CSR53_W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr53::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr53::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR53rs;
impl crate::RegisterSpec for CSR53rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr53::R`](R) reader structure"]
impl crate::Readable for CSR53rs {}
#[doc = "`write(|w| ..)` method takes [`csr53::W`](W) writer structure"]
impl crate::Writable for CSR53rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR53 to value 0"]
impl crate::Resettable for CSR53rs {
    const RESET_VALUE: u32 = 0;
}
