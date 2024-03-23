#[doc = "Register `PDMDLY` reader"]
pub type R = crate::R<PDMDLYrs>;
#[doc = "Register `PDMDLY` writer"]
pub type W = crate::W<PDMDLYrs>;
#[doc = "Field `DLYM1L` reader - Delay line adjust for first microphone of pair 1 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D1 line is available.Refer to to check if it is available."]
pub type DLYM1L_R = crate::FieldReader;
#[doc = "Field `DLYM1L` writer - Delay line adjust for first microphone of pair 1 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D1 line is available.Refer to to check if it is available."]
pub type DLYM1L_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM1R` reader - Delay line adjust for second microphone of pair 1 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D1 line is available.Refer to to check if it is available."]
pub type DLYM1R_R = crate::FieldReader;
#[doc = "Field `DLYM1R` writer - Delay line adjust for second microphone of pair 1 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D1 line is available.Refer to to check if it is available."]
pub type DLYM1R_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM2L` reader - Delay line for first microphone of pair 2 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D2 line is available.Refer to to check if it is available."]
pub type DLYM2L_R = crate::FieldReader;
#[doc = "Field `DLYM2L` writer - Delay line for first microphone of pair 2 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D2 line is available.Refer to to check if it is available."]
pub type DLYM2L_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM2R` reader - Delay line for second microphone of pair 2 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D2 line is available.Refer to to check if it is available."]
pub type DLYM2R_R = crate::FieldReader;
#[doc = "Field `DLYM2R` writer - Delay line for second microphone of pair 2 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D2 line is available.Refer to to check if it is available."]
pub type DLYM2R_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM3L` reader - Delay line for first microphone of pair 3 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D3 line is available.Refer to to check if it is available."]
pub type DLYM3L_R = crate::FieldReader;
#[doc = "Field `DLYM3L` writer - Delay line for first microphone of pair 3 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D3 line is available.Refer to to check if it is available."]
pub type DLYM3L_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM3R` reader - Delay line for second microphone of pair 3 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D3 line is available.Refer to to check if it is available."]
pub type DLYM3R_R = crate::FieldReader;
#[doc = "Field `DLYM3R` writer - Delay line for second microphone of pair 3 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D3 line is available.Refer to to check if it is available."]
pub type DLYM3R_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM4L` reader - Delay line for first microphone of pair 4 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D4 line is available.Refer to to check if it is available."]
pub type DLYM4L_R = crate::FieldReader;
#[doc = "Field `DLYM4L` writer - Delay line for first microphone of pair 4 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D4 line is available.Refer to to check if it is available."]
pub type DLYM4L_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM4R` reader - Delay line for second microphone of pair 4 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D4 line is available.Refer to to check if it is available."]
pub type DLYM4R_R = crate::FieldReader;
#[doc = "Field `DLYM4R` writer - Delay line for second microphone of pair 4 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D4 line is available.Refer to to check if it is available."]
pub type DLYM4R_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Delay line adjust for first microphone of pair 1 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D1 line is available.Refer to to check if it is available."]
    #[inline(always)]
    pub fn dlym1l(&self) -> DLYM1L_R {
        DLYM1L_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Delay line adjust for second microphone of pair 1 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D1 line is available.Refer to to check if it is available."]
    #[inline(always)]
    pub fn dlym1r(&self) -> DLYM1R_R {
        DLYM1R_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Delay line for first microphone of pair 2 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D2 line is available.Refer to to check if it is available."]
    #[inline(always)]
    pub fn dlym2l(&self) -> DLYM2L_R {
        DLYM2L_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Delay line for second microphone of pair 2 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D2 line is available.Refer to to check if it is available."]
    #[inline(always)]
    pub fn dlym2r(&self) -> DLYM2R_R {
        DLYM2R_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Delay line for first microphone of pair 3 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D3 line is available.Refer to to check if it is available."]
    #[inline(always)]
    pub fn dlym3l(&self) -> DLYM3L_R {
        DLYM3L_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Delay line for second microphone of pair 3 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D3 line is available.Refer to to check if it is available."]
    #[inline(always)]
    pub fn dlym3r(&self) -> DLYM3R_R {
        DLYM3R_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Delay line for first microphone of pair 4 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D4 line is available.Refer to to check if it is available."]
    #[inline(always)]
    pub fn dlym4l(&self) -> DLYM4L_R {
        DLYM4L_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Delay line for second microphone of pair 4 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D4 line is available.Refer to to check if it is available."]
    #[inline(always)]
    pub fn dlym4r(&self) -> DLYM4R_R {
        DLYM4R_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Delay line adjust for first microphone of pair 1 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D1 line is available.Refer to to check if it is available."]
    #[inline(always)]
    #[must_use]
    pub fn dlym1l(&mut self) -> DLYM1L_W<PDMDLYrs> {
        DLYM1L_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Delay line adjust for second microphone of pair 1 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D1 line is available.Refer to to check if it is available."]
    #[inline(always)]
    #[must_use]
    pub fn dlym1r(&mut self) -> DLYM1R_W<PDMDLYrs> {
        DLYM1R_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Delay line for first microphone of pair 2 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D2 line is available.Refer to to check if it is available."]
    #[inline(always)]
    #[must_use]
    pub fn dlym2l(&mut self) -> DLYM2L_W<PDMDLYrs> {
        DLYM2L_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Delay line for second microphone of pair 2 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D2 line is available.Refer to to check if it is available."]
    #[inline(always)]
    #[must_use]
    pub fn dlym2r(&mut self) -> DLYM2R_W<PDMDLYrs> {
        DLYM2R_W::new(self, 12)
    }
    #[doc = "Bits 16:18 - Delay line for first microphone of pair 3 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D3 line is available.Refer to to check if it is available."]
    #[inline(always)]
    #[must_use]
    pub fn dlym3l(&mut self) -> DLYM3L_W<PDMDLYrs> {
        DLYM3L_W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Delay line for second microphone of pair 3 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D3 line is available.Refer to to check if it is available."]
    #[inline(always)]
    #[must_use]
    pub fn dlym3r(&mut self) -> DLYM3R_W<PDMDLYrs> {
        DLYM3R_W::new(self, 20)
    }
    #[doc = "Bits 24:26 - Delay line for first microphone of pair 4 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D4 line is available.Refer to to check if it is available."]
    #[inline(always)]
    #[must_use]
    pub fn dlym4l(&mut self) -> DLYM4L_W<PDMDLYrs> {
        DLYM4L_W::new(self, 24)
    }
    #[doc = "Bits 28:30 - Delay line for second microphone of pair 4 This bit is set and cleared by software. ... This field can be changed on-the-fly. Note: This field can be used only if D4 line is available.Refer to to check if it is available."]
    #[inline(always)]
    #[must_use]
    pub fn dlym4r(&mut self) -> DLYM4R_W<PDMDLYrs> {
        DLYM4R_W::new(self, 28)
    }
}
#[doc = "SAI PDM delay register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdmdly::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdmdly::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDMDLYrs;
impl crate::RegisterSpec for PDMDLYrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdmdly::R`](R) reader structure"]
impl crate::Readable for PDMDLYrs {}
#[doc = "`write(|w| ..)` method takes [`pdmdly::W`](W) writer structure"]
impl crate::Writable for PDMDLYrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDMDLY to value 0"]
impl crate::Resettable for PDMDLYrs {
    const RESET_VALUE: u32 = 0;
}
