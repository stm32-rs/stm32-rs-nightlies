#[doc = "Register `IVR2` reader"]
pub type R = crate::R<IVR2rs>;
#[doc = "Register `IVR2` writer"]
pub type W = crate::W<IVR2rs>;
#[doc = "Field `IVI` reader - Initialization vector input, bits \\[95:64\\]
Refer to the AES_IVR0 register for description of the IVI\\[128:0\\]
bitfield."]
pub type IVI_R = crate::FieldReader<u32>;
#[doc = "Field `IVI` writer - Initialization vector input, bits \\[95:64\\]
Refer to the AES_IVR0 register for description of the IVI\\[128:0\\]
bitfield."]
pub type IVI_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Initialization vector input, bits \\[95:64\\]
Refer to the AES_IVR0 register for description of the IVI\\[128:0\\]
bitfield."]
    #[inline(always)]
    pub fn ivi(&self) -> IVI_R {
        IVI_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Initialization vector input, bits \\[95:64\\]
Refer to the AES_IVR0 register for description of the IVI\\[128:0\\]
bitfield."]
    #[inline(always)]
    #[must_use]
    pub fn ivi(&mut self) -> IVI_W<IVR2rs> {
        IVI_W::new(self, 0)
    }
}
#[doc = "AES initialization vector register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IVR2rs;
impl crate::RegisterSpec for IVR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ivr2::R`](R) reader structure"]
impl crate::Readable for IVR2rs {}
#[doc = "`write(|w| ..)` method takes [`ivr2::W`](W) writer structure"]
impl crate::Writable for IVR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IVR2 to value 0"]
impl crate::Resettable for IVR2rs {
    const RESET_VALUE: u32 = 0;
}
