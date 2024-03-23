#[doc = "Register `CSR52` reader"]
pub type R = crate::R<CSR52rs>;
#[doc = "Register `CSR52` writer"]
pub type W = crate::W<CSR52rs>;
#[doc = "Field `CSR52` reader - CSR52"]
pub type CSR52_R = crate::FieldReader<u32>;
#[doc = "Field `CSR52` writer - CSR52"]
pub type CSR52_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR52"]
    #[inline(always)]
    pub fn csr52(&self) -> CSR52_R {
        CSR52_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR52"]
    #[inline(always)]
    #[must_use]
    pub fn csr52(&mut self) -> CSR52_W<CSR52rs> {
        CSR52_W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr52::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr52::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR52rs;
impl crate::RegisterSpec for CSR52rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr52::R`](R) reader structure"]
impl crate::Readable for CSR52rs {}
#[doc = "`write(|w| ..)` method takes [`csr52::W`](W) writer structure"]
impl crate::Writable for CSR52rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR52 to value 0"]
impl crate::Resettable for CSR52rs {
    const RESET_VALUE: u32 = 0;
}
