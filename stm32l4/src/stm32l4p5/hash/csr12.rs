#[doc = "Register `CSR12` reader"]
pub type R = crate::R<CSR12rs>;
#[doc = "Register `CSR12` writer"]
pub type W = crate::W<CSR12rs>;
#[doc = "Field `CSR12` reader - CSR12"]
pub type CSR12_R = crate::FieldReader<u32>;
#[doc = "Field `CSR12` writer - CSR12"]
pub type CSR12_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR12"]
    #[inline(always)]
    pub fn csr12(&self) -> CSR12_R {
        CSR12_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR12"]
    #[inline(always)]
    #[must_use]
    pub fn csr12(&mut self) -> CSR12_W<CSR12rs> {
        CSR12_W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR12rs;
impl crate::RegisterSpec for CSR12rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr12::R`](R) reader structure"]
impl crate::Readable for CSR12rs {}
#[doc = "`write(|w| ..)` method takes [`csr12::W`](W) writer structure"]
impl crate::Writable for CSR12rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR12 to value 0"]
impl crate::Resettable for CSR12rs {
    const RESET_VALUE: u32 = 0;
}
