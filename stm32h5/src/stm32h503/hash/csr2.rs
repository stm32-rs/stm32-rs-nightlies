#[doc = "Register `CSR2` reader"]
pub type R = crate::R<CSR2rs>;
#[doc = "Register `CSR2` writer"]
pub type W = crate::W<CSR2rs>;
#[doc = "Field `CS2` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS2_R = crate::FieldReader<u32>;
#[doc = "Field `CS2` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn cs2(&self) -> CS2_R {
        CS2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    #[must_use]
    pub fn cs2(&mut self) -> CS2_W<CSR2rs> {
        CS2_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR2rs;
impl crate::RegisterSpec for CSR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr2::R`](R) reader structure"]
impl crate::Readable for CSR2rs {}
#[doc = "`write(|w| ..)` method takes [`csr2::W`](W) writer structure"]
impl crate::Writable for CSR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR2 to value 0"]
impl crate::Resettable for CSR2rs {
    const RESET_VALUE: u32 = 0;
}
