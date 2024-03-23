#[doc = "Register `CSR8` reader"]
pub type R = crate::R<CSR8rs>;
#[doc = "Register `CSR8` writer"]
pub type W = crate::W<CSR8rs>;
#[doc = "Field `CSR8` reader - CSR8"]
pub type CSR8_R = crate::FieldReader<u32>;
#[doc = "Field `CSR8` writer - CSR8"]
pub type CSR8_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR8"]
    #[inline(always)]
    pub fn csr8(&self) -> CSR8_R {
        CSR8_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR8"]
    #[inline(always)]
    #[must_use]
    pub fn csr8(&mut self) -> CSR8_W<CSR8rs> {
        CSR8_W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR8rs;
impl crate::RegisterSpec for CSR8rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr8::R`](R) reader structure"]
impl crate::Readable for CSR8rs {}
#[doc = "`write(|w| ..)` method takes [`csr8::W`](W) writer structure"]
impl crate::Writable for CSR8rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR8 to value 0"]
impl crate::Resettable for CSR8rs {
    const RESET_VALUE: u32 = 0;
}
