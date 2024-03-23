#[doc = "Register `SQR4` reader"]
pub type R = crate::R<SQR4rs>;
#[doc = "Register `SQR4` writer"]
pub type W = crate::W<SQR4rs>;
#[doc = "Field `SQ15` reader - 15th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 15th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ15_R = crate::FieldReader;
#[doc = "Field `SQ15` writer - 15th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 15th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ15_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ16` reader - 16th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 16th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ16_R = crate::FieldReader;
#[doc = "Field `SQ16` writer - 16th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 16th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ16_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 15th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 15th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn sq15(&self) -> SQ15_R {
        SQ15_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - 16th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 16th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn sq16(&self) -> SQ16_R {
        SQ16_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 15th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 15th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn sq15(&mut self) -> SQ15_W<SQR4rs> {
        SQ15_W::new(self, 0)
    }
    #[doc = "Bits 6:10 - 16th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 16th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn sq16(&mut self) -> SQ16_W<SQR4rs> {
        SQ16_W::new(self, 6)
    }
}
#[doc = "ADC regular sequence register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sqr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sqr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SQR4rs;
impl crate::RegisterSpec for SQR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sqr4::R`](R) reader structure"]
impl crate::Readable for SQR4rs {}
#[doc = "`write(|w| ..)` method takes [`sqr4::W`](W) writer structure"]
impl crate::Writable for SQR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SQR4 to value 0"]
impl crate::Resettable for SQR4rs {
    const RESET_VALUE: u32 = 0;
}
