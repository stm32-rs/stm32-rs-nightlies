#[doc = "Register `CSR11` reader"]
pub type R = crate::R<CSR11rs>;
#[doc = "Register `CSR11` writer"]
pub type W = crate::W<CSR11rs>;
#[doc = "Field `CSR11` reader - CSR11"]
pub type CSR11_R = crate::FieldReader<u32>;
#[doc = "Field `CSR11` writer - CSR11"]
pub type CSR11_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR11"]
    #[inline(always)]
    pub fn csr11(&self) -> CSR11_R {
        CSR11_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR11"]
    #[inline(always)]
    #[must_use]
    pub fn csr11(&mut self) -> CSR11_W<CSR11rs> {
        CSR11_W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR11rs;
impl crate::RegisterSpec for CSR11rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr11::R`](R) reader structure"]
impl crate::Readable for CSR11rs {}
#[doc = "`write(|w| ..)` method takes [`csr11::W`](W) writer structure"]
impl crate::Writable for CSR11rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR11 to value 0"]
impl crate::Resettable for CSR11rs {
    const RESET_VALUE: u32 = 0;
}
