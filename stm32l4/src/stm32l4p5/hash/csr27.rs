#[doc = "Register `CSR27` reader"]
pub type R = crate::R<CSR27rs>;
#[doc = "Register `CSR27` writer"]
pub type W = crate::W<CSR27rs>;
#[doc = "Field `CSR27` reader - CSR27"]
pub type CSR27_R = crate::FieldReader<u32>;
#[doc = "Field `CSR27` writer - CSR27"]
pub type CSR27_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR27"]
    #[inline(always)]
    pub fn csr27(&self) -> CSR27_R {
        CSR27_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR27"]
    #[inline(always)]
    #[must_use]
    pub fn csr27(&mut self) -> CSR27_W<CSR27rs> {
        CSR27_W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr27::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr27::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR27rs;
impl crate::RegisterSpec for CSR27rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr27::R`](R) reader structure"]
impl crate::Readable for CSR27rs {}
#[doc = "`write(|w| ..)` method takes [`csr27::W`](W) writer structure"]
impl crate::Writable for CSR27rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR27 to value 0"]
impl crate::Resettable for CSR27rs {
    const RESET_VALUE: u32 = 0;
}
