#[doc = "Register `KEYR5` reader"]
pub type R = crate::R<KEYR5rs>;
#[doc = "Register `KEYR5` writer"]
pub type W = crate::W<KEYR5rs>;
#[doc = "Field `KEY` reader - Cryptographic key, bits \\[191:160\\]
Refer to the AES_KEYR0 register for description of the KEY\\[255:0\\]
bitfield."]
pub type KEY_R = crate::FieldReader<u32>;
#[doc = "Field `KEY` writer - Cryptographic key, bits \\[191:160\\]
Refer to the AES_KEYR0 register for description of the KEY\\[255:0\\]
bitfield."]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cryptographic key, bits \\[191:160\\]
Refer to the AES_KEYR0 register for description of the KEY\\[255:0\\]
bitfield."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cryptographic key, bits \\[191:160\\]
Refer to the AES_KEYR0 register for description of the KEY\\[255:0\\]
bitfield."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<KEYR5rs> {
        KEY_W::new(self, 0)
    }
}
#[doc = "AES key register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYR5rs;
impl crate::RegisterSpec for KEYR5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyr5::R`](R) reader structure"]
impl crate::Readable for KEYR5rs {}
#[doc = "`write(|w| ..)` method takes [`keyr5::W`](W) writer structure"]
impl crate::Writable for KEYR5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYR5 to value 0"]
impl crate::Resettable for KEYR5rs {
    const RESET_VALUE: u32 = 0;
}
