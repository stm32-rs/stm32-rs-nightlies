#[doc = "Register `CSR6` reader"]
pub type R = crate::R<CSR6rs>;
#[doc = "Register `CSR6` writer"]
pub type W = crate::W<CSR6rs>;
#[doc = "Field `CSR6` reader - CSR6"]
pub type CSR6_R = crate::FieldReader<u32>;
#[doc = "Field `CSR6` writer - CSR6"]
pub type CSR6_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR6"]
    #[inline(always)]
    pub fn csr6(&self) -> CSR6_R {
        CSR6_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR6"]
    #[inline(always)]
    #[must_use]
    pub fn csr6(&mut self) -> CSR6_W<CSR6rs> {
        CSR6_W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR6rs;
impl crate::RegisterSpec for CSR6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr6::R`](R) reader structure"]
impl crate::Readable for CSR6rs {}
#[doc = "`write(|w| ..)` method takes [`csr6::W`](W) writer structure"]
impl crate::Writable for CSR6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR6 to value 0"]
impl crate::Resettable for CSR6rs {
    const RESET_VALUE: u32 = 0;
}
