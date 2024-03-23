#[doc = "Register `CSR15` reader"]
pub type R = crate::R<CSR15rs>;
#[doc = "Register `CSR15` writer"]
pub type W = crate::W<CSR15rs>;
#[doc = "Field `CSR15` reader - CSR15"]
pub type CSR15_R = crate::FieldReader<u32>;
#[doc = "Field `CSR15` writer - CSR15"]
pub type CSR15_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR15"]
    #[inline(always)]
    pub fn csr15(&self) -> CSR15_R {
        CSR15_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR15"]
    #[inline(always)]
    #[must_use]
    pub fn csr15(&mut self) -> CSR15_W<CSR15rs> {
        CSR15_W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR15rs;
impl crate::RegisterSpec for CSR15rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr15::R`](R) reader structure"]
impl crate::Readable for CSR15rs {}
#[doc = "`write(|w| ..)` method takes [`csr15::W`](W) writer structure"]
impl crate::Writable for CSR15rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR15 to value 0"]
impl crate::Resettable for CSR15rs {
    const RESET_VALUE: u32 = 0;
}
