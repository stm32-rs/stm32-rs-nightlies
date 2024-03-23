#[doc = "Register `IVR3` reader"]
pub type R = crate::R<IVR3rs>;
#[doc = "Register `IVR3` writer"]
pub type W = crate::W<IVR3rs>;
#[doc = "Field `IVI` reader - Initialization Vector Register (MSB IVR \\[127:96\\])"]
pub type IVI_R = crate::FieldReader<u32>;
#[doc = "Field `IVI` writer - Initialization Vector Register (MSB IVR \\[127:96\\])"]
pub type IVI_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Initialization Vector Register (MSB IVR \\[127:96\\])"]
    #[inline(always)]
    pub fn ivi(&self) -> IVI_R {
        IVI_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Initialization Vector Register (MSB IVR \\[127:96\\])"]
    #[inline(always)]
    #[must_use]
    pub fn ivi(&mut self) -> IVI_W<IVR3rs> {
        IVI_W::new(self, 0)
    }
}
#[doc = "initialization vector register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IVR3rs;
impl crate::RegisterSpec for IVR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ivr3::R`](R) reader structure"]
impl crate::Readable for IVR3rs {}
#[doc = "`write(|w| ..)` method takes [`ivr3::W`](W) writer structure"]
impl crate::Writable for IVR3rs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IVR3 to value 0"]
impl crate::Resettable for IVR3rs {
    const RESET_VALUE: u32 = 0;
}
