#[doc = "Register `IVR1` reader"]
pub type R = crate::R<IVR1rs>;
#[doc = "Register `IVR1` writer"]
pub type W = crate::W<IVR1rs>;
#[doc = "Field `IVI` reader - Initialization vector input, bits \\[63:32\\]
Refer to the SAES_IVR0 register for description of the IVI\\[128:0\\]
bitfield."]
pub type IVI_R = crate::FieldReader<u32>;
#[doc = "Field `IVI` writer - Initialization vector input, bits \\[63:32\\]
Refer to the SAES_IVR0 register for description of the IVI\\[128:0\\]
bitfield."]
pub type IVI_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Initialization vector input, bits \\[63:32\\]
Refer to the SAES_IVR0 register for description of the IVI\\[128:0\\]
bitfield."]
    #[inline(always)]
    pub fn ivi(&self) -> IVI_R {
        IVI_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Initialization vector input, bits \\[63:32\\]
Refer to the SAES_IVR0 register for description of the IVI\\[128:0\\]
bitfield."]
    #[inline(always)]
    #[must_use]
    pub fn ivi(&mut self) -> IVI_W<IVR1rs> {
        IVI_W::new(self, 0)
    }
}
#[doc = "SAES initialization vector register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
