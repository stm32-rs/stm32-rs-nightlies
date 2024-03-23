#[doc = "Register `CSR35` reader"]
pub type R = crate::R<CSR35rs>;
#[doc = "Register `CSR35` writer"]
pub type W = crate::W<CSR35rs>;
#[doc = "Field `CS35` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS35_R = crate::FieldReader<u32>;
#[doc = "Field `CS35` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS35_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn cs35(&self) -> CS35_R {
        CS35_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    #[must_use]
    pub fn cs35(&mut self) -> CS35_W<CSR35rs> {
        CS35_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr35::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr35::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR35rs;
impl crate::RegisterSpec for CSR35rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr35::R`](R) reader structure"]
impl crate::Readable for CSR35rs {}
#[doc = "`write(|w| ..)` method takes [`csr35::W`](W) writer structure"]
impl crate::Writable for CSR35rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR35 to value 0"]
impl crate::Resettable for CSR35rs {
    const RESET_VALUE: u32 = 0;
}
