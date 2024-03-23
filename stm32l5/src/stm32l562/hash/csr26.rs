#[doc = "Register `CSR26` reader"]
pub type R = crate::R<CSR26rs>;
#[doc = "Register `CSR26` writer"]
pub type W = crate::W<CSR26rs>;
#[doc = "Field `CSR26` reader - CSR26"]
pub type CSR26_R = crate::FieldReader<u32>;
#[doc = "Field `CSR26` writer - CSR26"]
pub type CSR26_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR26"]
    #[inline(always)]
    pub fn csr26(&self) -> CSR26_R {
        CSR26_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR26"]
    #[inline(always)]
    #[must_use]
    pub fn csr26(&mut self) -> CSR26_W<CSR26rs> {
        CSR26_W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr26::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr26::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR26rs;
impl crate::RegisterSpec for CSR26rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr26::R`](R) reader structure"]
impl crate::Readable for CSR26rs {}
#[doc = "`write(|w| ..)` method takes [`csr26::W`](W) writer structure"]
impl crate::Writable for CSR26rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR26 to value 0"]
impl crate::Resettable for CSR26rs {
    const RESET_VALUE: u32 = 0;
}
