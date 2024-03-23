#[doc = "Register `CSR25` reader"]
pub type R = crate::R<CSR25rs>;
#[doc = "Register `CSR25` writer"]
pub type W = crate::W<CSR25rs>;
#[doc = "Field `CS25` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS25_R = crate::FieldReader<u32>;
#[doc = "Field `CS25` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS25_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn cs25(&self) -> CS25_R {
        CS25_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    #[must_use]
    pub fn cs25(&mut self) -> CS25_W<CSR25rs> {
        CS25_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr25::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr25::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
