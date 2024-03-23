#[doc = "Register `CSR38` reader"]
pub type R = crate::R<CSR38rs>;
#[doc = "Register `CSR38` writer"]
pub type W = crate::W<CSR38rs>;
#[doc = "Field `CSR38` reader - CSR38"]
pub type CSR38_R = crate::FieldReader<u32>;
#[doc = "Field `CSR38` writer - CSR38"]
pub type CSR38_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR38"]
    #[inline(always)]
    pub fn csr38(&self) -> CSR38_R {
        CSR38_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR38"]
    #[inline(always)]
    #[must_use]
    pub fn csr38(&mut self) -> CSR38_W<CSR38rs> {
        CSR38_W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr38::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr38::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR38rs;
impl crate::RegisterSpec for CSR38rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr38::R`](R) reader structure"]
impl crate::Readable for CSR38rs {}
#[doc = "`write(|w| ..)` method takes [`csr38::W`](W) writer structure"]
impl crate::Writable for CSR38rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR38 to value 0"]
impl crate::Resettable for CSR38rs {
    const RESET_VALUE: u32 = 0;
}
