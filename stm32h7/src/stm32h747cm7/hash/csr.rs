#[doc = "Register `CSR%s` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Register `CSR%s` writer"]
pub type W = crate::W<CSRrs>;
#[doc = "Field `CSR` reader - CSR0"]
pub type CSR_R = crate::FieldReader<u32>;
#[doc = "Field `CSR` writer - CSR0"]
pub type CSR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR0"]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR0"]
    #[inline(always)]
    #[must_use]
    pub fn csr(&mut self) -> CSR_W<CSRrs> {
        CSR_W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSRrs {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR%s to value 0"]
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0;
}
