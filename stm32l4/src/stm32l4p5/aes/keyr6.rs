#[doc = "Register `KEYR6` reader"]
pub type R = crate::R<KEYR6rs>;
#[doc = "Register `KEYR6` writer"]
pub type W = crate::W<KEYR6rs>;
#[doc = "Field `KEY` reader - Cryptographic key, bits \\[223:192\\]"]
pub type KEY_R = crate::FieldReader<u32>;
#[doc = "Field `KEY` writer - Cryptographic key, bits \\[223:192\\]"]
pub type KEY_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cryptographic key, bits \\[223:192\\]"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cryptographic key, bits \\[223:192\\]"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<KEYR6rs> {
        KEY_W::new(self, 0)
    }
}
#[doc = "key register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYR6rs;
impl crate::RegisterSpec for KEYR6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyr6::R`](R) reader structure"]
impl crate::Readable for KEYR6rs {}
#[doc = "`write(|w| ..)` method takes [`keyr6::W`](W) writer structure"]
impl crate::Writable for KEYR6rs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYR6 to value 0"]
impl crate::Resettable for KEYR6rs {
    const RESET_VALUE: u32 = 0;
}
