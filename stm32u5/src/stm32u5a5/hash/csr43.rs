#[doc = "Register `CSR43` reader"]
pub type R = crate::R<CSR43rs>;
#[doc = "Register `CSR43` writer"]
pub type W = crate::W<CSR43rs>;
#[doc = "Field `CS43` reader - CS43"]
pub type CS43_R = crate::FieldReader<u32>;
#[doc = "Field `CS43` writer - CS43"]
pub type CS43_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS43"]
    #[inline(always)]
    pub fn cs43(&self) -> CS43_R {
        CS43_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS43"]
    #[inline(always)]
    #[must_use]
    pub fn cs43(&mut self) -> CS43_W<CSR43rs> {
        CS43_W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr43::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr43::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR43rs;
impl crate::RegisterSpec for CSR43rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr43::R`](R) reader structure"]
impl crate::Readable for CSR43rs {}
#[doc = "`write(|w| ..)` method takes [`csr43::W`](W) writer structure"]
impl crate::Writable for CSR43rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR43 to value 0"]
impl crate::Resettable for CSR43rs {
    const RESET_VALUE: u32 = 0;
}
