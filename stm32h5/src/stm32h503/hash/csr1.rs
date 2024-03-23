#[doc = "Register `CSR1` reader"]
pub type R = crate::R<CSR1rs>;
#[doc = "Register `CSR1` writer"]
pub type W = crate::W<CSR1rs>;
#[doc = "Field `CS1` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS1_R = crate::FieldReader<u32>;
#[doc = "Field `CS1` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn cs1(&self) -> CS1_R {
        CS1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    #[must_use]
    pub fn cs1(&mut self) -> CS1_W<CSR1rs> {
        CS1_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR1rs;
impl crate::RegisterSpec for CSR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr1::R`](R) reader structure"]
impl crate::Readable for CSR1rs {}
#[doc = "`write(|w| ..)` method takes [`csr1::W`](W) writer structure"]
impl crate::Writable for CSR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR1 to value 0"]
impl crate::Resettable for CSR1rs {
    const RESET_VALUE: u32 = 0;
}
