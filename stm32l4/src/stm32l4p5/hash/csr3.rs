#[doc = "Register `CSR3` reader"]
pub type R = crate::R<CSR3rs>;
#[doc = "Register `CSR3` writer"]
pub type W = crate::W<CSR3rs>;
#[doc = "Field `CSR3` reader - CSR3"]
pub type CSR3_R = crate::FieldReader<u32>;
#[doc = "Field `CSR3` writer - CSR3"]
pub type CSR3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR3"]
    #[inline(always)]
    pub fn csr3(&self) -> CSR3_R {
        CSR3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR3"]
    #[inline(always)]
    #[must_use]
    pub fn csr3(&mut self) -> CSR3_W<CSR3rs> {
        CSR3_W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR3rs;
impl crate::RegisterSpec for CSR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr3::R`](R) reader structure"]
impl crate::Readable for CSR3rs {}
#[doc = "`write(|w| ..)` method takes [`csr3::W`](W) writer structure"]
impl crate::Writable for CSR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR3 to value 0"]
impl crate::Resettable for CSR3rs {
    const RESET_VALUE: u32 = 0;
}
