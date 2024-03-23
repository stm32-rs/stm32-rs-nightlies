#[doc = "Register `CSR31` reader"]
pub type R = crate::R<CSR31rs>;
#[doc = "Register `CSR31` writer"]
pub type W = crate::W<CSR31rs>;
#[doc = "Field `CSR31` reader - CSR31"]
pub type CSR31_R = crate::FieldReader<u32>;
#[doc = "Field `CSR31` writer - CSR31"]
pub type CSR31_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR31"]
    #[inline(always)]
    pub fn csr31(&self) -> CSR31_R {
        CSR31_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR31"]
    #[inline(always)]
    #[must_use]
    pub fn csr31(&mut self) -> CSR31_W<CSR31rs> {
        CSR31_W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr31::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr31::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR31rs;
impl crate::RegisterSpec for CSR31rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr31::R`](R) reader structure"]
impl crate::Readable for CSR31rs {}
#[doc = "`write(|w| ..)` method takes [`csr31::W`](W) writer structure"]
impl crate::Writable for CSR31rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR31 to value 0"]
impl crate::Resettable for CSR31rs {
    const RESET_VALUE: u32 = 0;
}
