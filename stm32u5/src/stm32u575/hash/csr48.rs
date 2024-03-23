#[doc = "Register `CSR48` reader"]
pub type R = crate::R<CSR48rs>;
#[doc = "Register `CSR48` writer"]
pub type W = crate::W<CSR48rs>;
#[doc = "Field `CSR48` reader - CSR48"]
pub type CSR48_R = crate::FieldReader<u32>;
#[doc = "Field `CSR48` writer - CSR48"]
pub type CSR48_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR48"]
    #[inline(always)]
    pub fn csr48(&self) -> CSR48_R {
        CSR48_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR48"]
    #[inline(always)]
    #[must_use]
    pub fn csr48(&mut self) -> CSR48_W<CSR48rs> {
        CSR48_W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr48::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr48::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR48rs;
impl crate::RegisterSpec for CSR48rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr48::R`](R) reader structure"]
impl crate::Readable for CSR48rs {}
#[doc = "`write(|w| ..)` method takes [`csr48::W`](W) writer structure"]
impl crate::Writable for CSR48rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR48 to value 0"]
impl crate::Resettable for CSR48rs {
    const RESET_VALUE: u32 = 0;
}
