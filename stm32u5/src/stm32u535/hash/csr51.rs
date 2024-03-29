#[doc = "Register `CSR51` reader"]
pub type R = crate::R<CSR51rs>;
#[doc = "Register `CSR51` writer"]
pub type W = crate::W<CSR51rs>;
#[doc = "Field `CS51` reader - CS51"]
pub type CS51_R = crate::FieldReader<u32>;
#[doc = "Field `CS51` writer - CS51"]
pub type CS51_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CS51"]
    #[inline(always)]
    pub fn cs51(&self) -> CS51_R {
        CS51_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS51"]
    #[inline(always)]
    #[must_use]
    pub fn cs51(&mut self) -> CS51_W<CSR51rs> {
        CS51_W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr51::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr51::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR51rs;
impl crate::RegisterSpec for CSR51rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr51::R`](R) reader structure"]
impl crate::Readable for CSR51rs {}
#[doc = "`write(|w| ..)` method takes [`csr51::W`](W) writer structure"]
impl crate::Writable for CSR51rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR51 to value 0"]
impl crate::Resettable for CSR51rs {
    const RESET_VALUE: u32 = 0;
}
