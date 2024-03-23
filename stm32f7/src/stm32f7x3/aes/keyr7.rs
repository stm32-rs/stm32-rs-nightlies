#[doc = "Register `KEYR7` reader"]
pub type R = crate::R<KEYR7rs>;
#[doc = "Register `KEYR7` writer"]
pub type W = crate::W<KEYR7rs>;
#[doc = "Field `AES_KEYR7` reader - AES key register (MSB key \\[255:224\\])"]
pub type AES_KEYR7_R = crate::FieldReader<u32>;
#[doc = "Field `AES_KEYR7` writer - AES key register (MSB key \\[255:224\\])"]
pub type AES_KEYR7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AES key register (MSB key \\[255:224\\])"]
    #[inline(always)]
    pub fn aes_keyr7(&self) -> AES_KEYR7_R {
        AES_KEYR7_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES key register (MSB key \\[255:224\\])"]
    #[inline(always)]
    #[must_use]
    pub fn aes_keyr7(&mut self) -> AES_KEYR7_W<KEYR7rs> {
        AES_KEYR7_W::new(self, 0)
    }
}
#[doc = "key register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYR7rs;
impl crate::RegisterSpec for KEYR7rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyr7::R`](R) reader structure"]
impl crate::Readable for KEYR7rs {}
#[doc = "`write(|w| ..)` method takes [`keyr7::W`](W) writer structure"]
impl crate::Writable for KEYR7rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYR7 to value 0"]
impl crate::Resettable for KEYR7rs {
    const RESET_VALUE: u32 = 0;
}
