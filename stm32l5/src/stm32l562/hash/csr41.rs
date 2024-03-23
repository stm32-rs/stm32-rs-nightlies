#[doc = "Register `CSR41` reader"]
pub type R = crate::R<CSR41rs>;
#[doc = "Register `CSR41` writer"]
pub type W = crate::W<CSR41rs>;
#[doc = "Field `CSR41` reader - CSR41"]
pub type CSR41_R = crate::FieldReader<u32>;
#[doc = "Field `CSR41` writer - CSR41"]
pub type CSR41_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR41"]
    #[inline(always)]
    pub fn csr41(&self) -> CSR41_R {
        CSR41_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR41"]
    #[inline(always)]
    #[must_use]
    pub fn csr41(&mut self) -> CSR41_W<CSR41rs> {
        CSR41_W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr41::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr41::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR41rs;
impl crate::RegisterSpec for CSR41rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr41::R`](R) reader structure"]
impl crate::Readable for CSR41rs {}
#[doc = "`write(|w| ..)` method takes [`csr41::W`](W) writer structure"]
impl crate::Writable for CSR41rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR41 to value 0"]
impl crate::Resettable for CSR41rs {
    const RESET_VALUE: u32 = 0;
}
