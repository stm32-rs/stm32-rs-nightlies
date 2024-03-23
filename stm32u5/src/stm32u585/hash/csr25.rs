#[doc = "Register `CSR25` reader"]
pub type R = crate::R<CSR25rs>;
#[doc = "Register `CSR25` writer"]
pub type W = crate::W<CSR25rs>;
#[doc = "Field `CSR25` reader - CSR25"]
pub type CSR25_R = crate::FieldReader<u32>;
#[doc = "Field `CSR25` writer - CSR25"]
pub type CSR25_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR25"]
    #[inline(always)]
    pub fn csr25(&self) -> CSR25_R {
        CSR25_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR25"]
    #[inline(always)]
    #[must_use]
    pub fn csr25(&mut self) -> CSR25_W<CSR25rs> {
        CSR25_W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr25::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr25::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR25rs;
impl crate::RegisterSpec for CSR25rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr25::R`](R) reader structure"]
impl crate::Readable for CSR25rs {}
#[doc = "`write(|w| ..)` method takes [`csr25::W`](W) writer structure"]
impl crate::Writable for CSR25rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR25 to value 0"]
impl crate::Resettable for CSR25rs {
    const RESET_VALUE: u32 = 0;
}
