#[doc = "Register `SQR2` reader"]
pub type R = crate::R<SQR2rs>;
#[doc = "Register `SQR2` writer"]
pub type W = crate::W<SQR2rs>;
#[doc = "Field `SQ5` reader - 5th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 5th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ5_R = crate::FieldReader;
#[doc = "Field `SQ5` writer - 5th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 5th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ5_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ6` reader - 6th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 6th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ6_R = crate::FieldReader;
#[doc = "Field `SQ6` writer - 6th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 6th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ6_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ7` reader - 7th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 7th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ7_R = crate::FieldReader;
#[doc = "Field `SQ7` writer - 7th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 7th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ7_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ8` reader - 8th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 8th in the regular conversion sequence Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ8_R = crate::FieldReader;
#[doc = "Field `SQ8` writer - 8th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 8th in the regular conversion sequence Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ8_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ9` reader - 9th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 9th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ9_R = crate::FieldReader;
#[doc = "Field `SQ9` writer - 9th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 9th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ9_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 5th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 5th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn sq5(&self) -> SQ5_R {
        SQ5_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - 6th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 6th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn sq6(&self) -> SQ6_R {
        SQ6_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - 7th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 7th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn sq7(&self) -> SQ7_R {
        SQ7_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - 8th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 8th in the regular conversion sequence Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn sq8(&self) -> SQ8_R {
        SQ8_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 9th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 9th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn sq9(&self) -> SQ9_R {
        SQ9_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 5th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 5th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn sq5(&mut self) -> SQ5_W<SQR2rs> {
        SQ5_W::new(self, 0)
    }
    #[doc = "Bits 6:10 - 6th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 6th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn sq6(&mut self) -> SQ6_W<SQR2rs> {
        SQ6_W::new(self, 6)
    }
    #[doc = "Bits 12:16 - 7th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 7th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn sq7(&mut self) -> SQ7_W<SQR2rs> {
        SQ7_W::new(self, 12)
    }
    #[doc = "Bits 18:22 - 8th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 8th in the regular conversion sequence Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn sq8(&mut self) -> SQ8_W<SQR2rs> {
        SQ8_W::new(self, 18)
    }
    #[doc = "Bits 24:28 - 9th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 9th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn sq9(&mut self) -> SQ9_W<SQR2rs> {
        SQ9_W::new(self, 24)
    }
}
#[doc = "ADC regular sequence register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sqr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sqr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SQR2rs;
impl crate::RegisterSpec for SQR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sqr2::R`](R) reader structure"]
impl crate::Readable for SQR2rs {}
#[doc = "`write(|w| ..)` method takes [`sqr2::W`](W) writer structure"]
impl crate::Writable for SQR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SQR2 to value 0"]
impl crate::Resettable for SQR2rs {
    const RESET_VALUE: u32 = 0;
}
