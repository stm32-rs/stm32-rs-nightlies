#[doc = "Register `CSR16` reader"]
pub type R = crate::R<CSR16rs>;
#[doc = "Register `CSR16` writer"]
pub type W = crate::W<CSR16rs>;
#[doc = "Field `CSR16` reader - CSR16"]
pub type CSR16_R = crate::FieldReader<u32>;
#[doc = "Field `CSR16` writer - CSR16"]
pub type CSR16_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR16"]
    #[inline(always)]
    pub fn csr16(&self) -> CSR16_R {
        CSR16_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR16"]
    #[inline(always)]
    #[must_use]
    pub fn csr16(&mut self) -> CSR16_W<CSR16rs> {
        CSR16_W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR16rs;
impl crate::RegisterSpec for CSR16rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr16::R`](R) reader structure"]
impl crate::Readable for CSR16rs {}
#[doc = "`write(|w| ..)` method takes [`csr16::W`](W) writer structure"]
impl crate::Writable for CSR16rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR16 to value 0"]
impl crate::Resettable for CSR16rs {
    const RESET_VALUE: u32 = 0;
}
