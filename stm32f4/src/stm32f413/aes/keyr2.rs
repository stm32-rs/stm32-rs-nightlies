#[doc = "Register `KEYR2` reader"]
pub type R = crate::R<KEYR2rs>;
#[doc = "Register `KEYR2` writer"]
pub type W = crate::W<KEYR2rs>;
#[doc = "Field `AES_KEYR2` reader - AES key register (key \\[95:64\\])"]
pub type AES_KEYR2_R = crate::FieldReader<u32>;
#[doc = "Field `AES_KEYR2` writer - AES key register (key \\[95:64\\])"]
pub type AES_KEYR2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AES key register (key \\[95:64\\])"]
    #[inline(always)]
    pub fn aes_keyr2(&self) -> AES_KEYR2_R {
        AES_KEYR2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES key register (key \\[95:64\\])"]
    #[inline(always)]
    #[must_use]
    pub fn aes_keyr2(&mut self) -> AES_KEYR2_W<KEYR2rs> {
        AES_KEYR2_W::new(self, 0)
    }
}
#[doc = "key register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYR2rs;
impl crate::RegisterSpec for KEYR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyr2::R`](R) reader structure"]
impl crate::Readable for KEYR2rs {}
#[doc = "`write(|w| ..)` method takes [`keyr2::W`](W) writer structure"]
impl crate::Writable for KEYR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYR2 to value 0"]
impl crate::Resettable for KEYR2rs {
    const RESET_VALUE: u32 = 0;
}
