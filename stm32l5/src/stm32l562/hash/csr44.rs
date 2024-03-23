#[doc = "Register `CSR44` reader"]
pub type R = crate::R<CSR44rs>;
#[doc = "Register `CSR44` writer"]
pub type W = crate::W<CSR44rs>;
#[doc = "Field `CSR44` reader - CSR44"]
pub type CSR44_R = crate::FieldReader<u32>;
#[doc = "Field `CSR44` writer - CSR44"]
pub type CSR44_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR44"]
    #[inline(always)]
    pub fn csr44(&self) -> CSR44_R {
        CSR44_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR44"]
    #[inline(always)]
    #[must_use]
    pub fn csr44(&mut self) -> CSR44_W<CSR44rs> {
        CSR44_W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr44::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr44::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR44rs;
impl crate::RegisterSpec for CSR44rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr44::R`](R) reader structure"]
impl crate::Readable for CSR44rs {}
#[doc = "`write(|w| ..)` method takes [`csr44::W`](W) writer structure"]
impl crate::Writable for CSR44rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR44 to value 0"]
impl crate::Resettable for CSR44rs {
    const RESET_VALUE: u32 = 0;
}
