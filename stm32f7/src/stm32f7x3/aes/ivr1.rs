#[doc = "Register `IVR1` reader"]
pub type R = crate::R<IVR1rs>;
#[doc = "Register `IVR1` writer"]
pub type W = crate::W<IVR1rs>;
#[doc = "Field `AES_IVR1` reader - Initialization Vector Register (IVR \\[63:32\\])"]
pub type AES_IVR1_R = crate::FieldReader<u32>;
#[doc = "Field `AES_IVR1` writer - Initialization Vector Register (IVR \\[63:32\\])"]
pub type AES_IVR1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Initialization Vector Register (IVR \\[63:32\\])"]
    #[inline(always)]
    pub fn aes_ivr1(&self) -> AES_IVR1_R {
        AES_IVR1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Initialization Vector Register (IVR \\[63:32\\])"]
    #[inline(always)]
    #[must_use]
    pub fn aes_ivr1(&mut self) -> AES_IVR1_W<IVR1rs> {
        AES_IVR1_W::new(self, 0)
    }
}
#[doc = "initialization vector register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IVR1rs;
impl crate::RegisterSpec for IVR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ivr1::R`](R) reader structure"]
impl crate::Readable for IVR1rs {}
#[doc = "`write(|w| ..)` method takes [`ivr1::W`](W) writer structure"]
impl crate::Writable for IVR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IVR1 to value 0"]
impl crate::Resettable for IVR1rs {
    const RESET_VALUE: u32 = 0;
}
