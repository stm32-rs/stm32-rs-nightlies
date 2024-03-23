#[doc = "Register `CSR29` reader"]
pub type R = crate::R<CSR29rs>;
#[doc = "Register `CSR29` writer"]
pub type W = crate::W<CSR29rs>;
#[doc = "Field `CS29` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS29_R = crate::FieldReader<u32>;
#[doc = "Field `CS29` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS29_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn cs29(&self) -> CS29_R {
        CS29_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    #[must_use]
    pub fn cs29(&mut self) -> CS29_W<CSR29rs> {
        CS29_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr29::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr29::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR29rs;
impl crate::RegisterSpec for CSR29rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr29::R`](R) reader structure"]
impl crate::Readable for CSR29rs {}
#[doc = "`write(|w| ..)` method takes [`csr29::W`](W) writer structure"]
impl crate::Writable for CSR29rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR29 to value 0"]
impl crate::Resettable for CSR29rs {
    const RESET_VALUE: u32 = 0;
}
