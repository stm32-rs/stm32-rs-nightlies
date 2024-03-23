#[doc = "Register `CSR0` reader"]
pub type R = crate::R<CSR0rs>;
#[doc = "Register `CSR0` writer"]
pub type W = crate::W<CSR0rs>;
#[doc = "Field `CSR0` reader - CSR0"]
pub type CSR0_R = crate::FieldReader<u32>;
#[doc = "Field `CSR0` writer - CSR0"]
pub type CSR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR0"]
    #[inline(always)]
    pub fn csr0(&self) -> CSR0_R {
        CSR0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR0"]
    #[inline(always)]
    #[must_use]
    pub fn csr0(&mut self) -> CSR0_W<CSR0rs> {
        CSR0_W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR0rs;
impl crate::RegisterSpec for CSR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr0::R`](R) reader structure"]
impl crate::Readable for CSR0rs {}
#[doc = "`write(|w| ..)` method takes [`csr0::W`](W) writer structure"]
impl crate::Writable for CSR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR0 to value 0"]
impl crate::Resettable for CSR0rs {
    const RESET_VALUE: u32 = 0;
}
