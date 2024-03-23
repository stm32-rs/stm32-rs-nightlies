#[doc = "Register `KEYR4` reader"]
pub type R = crate::R<KEYR4rs>;
#[doc = "Register `KEYR4` writer"]
pub type W = crate::W<KEYR4rs>;
#[doc = "Field `AES_KEYR4` reader - AES key register (MSB key \\[159:128\\])"]
pub type AES_KEYR4_R = crate::FieldReader<u32>;
#[doc = "Field `AES_KEYR4` writer - AES key register (MSB key \\[159:128\\])"]
pub type AES_KEYR4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AES key register (MSB key \\[159:128\\])"]
    #[inline(always)]
    pub fn aes_keyr4(&self) -> AES_KEYR4_R {
        AES_KEYR4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES key register (MSB key \\[159:128\\])"]
    #[inline(always)]
    #[must_use]
    pub fn aes_keyr4(&mut self) -> AES_KEYR4_W<KEYR4rs> {
        AES_KEYR4_W::new(self, 0)
    }
}
#[doc = "key register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYR4rs;
impl crate::RegisterSpec for KEYR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyr4::R`](R) reader structure"]
impl crate::Readable for KEYR4rs {}
#[doc = "`write(|w| ..)` method takes [`keyr4::W`](W) writer structure"]
impl crate::Writable for KEYR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYR4 to value 0"]
impl crate::Resettable for KEYR4rs {
    const RESET_VALUE: u32 = 0;
}
