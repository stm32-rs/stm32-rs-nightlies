#[doc = "Register `ARR` reader"]
pub type R = crate::R<ARRrs>;
#[doc = "Register `ARR` writer"]
pub type W = crate::W<ARRrs>;
#[doc = "Field `ARR` reader - Auto-reload value ARR is the value to be loaded into the actual auto-reload register. Refer to for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null. Non-dithering mode (DITHEN = 0) The register holds the auto-reload value in ARR\\[15:0\\]. The ARR\\[19:16\\]
bits are reserved. Dithering mode (DITHEN = 1) The register holds the integer part in ARR\\[19:4\\]. The ARR\\[3:0\\]
bitfield contains the dithered part."]
pub type ARR_R = crate::FieldReader<u32>;
#[doc = "Field `ARR` writer - Auto-reload value ARR is the value to be loaded into the actual auto-reload register. Refer to for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null. Non-dithering mode (DITHEN = 0) The register holds the auto-reload value in ARR\\[15:0\\]. The ARR\\[19:16\\]
bits are reserved. Dithering mode (DITHEN = 1) The register holds the integer part in ARR\\[19:4\\]. The ARR\\[3:0\\]
bitfield contains the dithered part."]
pub type ARR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 20, u32>;
#[doc = "Field `DITHER` reader - Dithered part in dithering mode"]
pub type DITHER_R = crate::FieldReader;
#[doc = "Field `DITHER` writer - Dithered part in dithering mode"]
pub type DITHER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INTEGER` reader - Integer part in dithering mode"]
pub type INTEGER_R = crate::FieldReader<u16>;
#[doc = "Field `INTEGER` writer - Integer part in dithering mode"]
pub type INTEGER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:19 - Auto-reload value ARR is the value to be loaded into the actual auto-reload register. Refer to for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null. Non-dithering mode (DITHEN = 0) The register holds the auto-reload value in ARR\\[15:0\\]. The ARR\\[19:16\\]
bits are reserved. Dithering mode (DITHEN = 1) The register holds the integer part in ARR\\[19:4\\]. The ARR\\[3:0\\]
bitfield contains the dithered part."]
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 0:3 - Dithered part in dithering mode"]
    #[inline(always)]
    pub fn dither(&self) -> DITHER_R {
        DITHER_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:19 - Integer part in dithering mode"]
    #[inline(always)]
    pub fn integer(&self) -> INTEGER_R {
        INTEGER_R::new(((self.bits >> 4) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - Auto-reload value ARR is the value to be loaded into the actual auto-reload register. Refer to for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null. Non-dithering mode (DITHEN = 0) The register holds the auto-reload value in ARR\\[15:0\\]. The ARR\\[19:16\\]
bits are reserved. Dithering mode (DITHEN = 1) The register holds the integer part in ARR\\[19:4\\]. The ARR\\[3:0\\]
bitfield contains the dithered part."]
    #[inline(always)]
    #[must_use]
    pub fn arr(&mut self) -> ARR_W<ARRrs> {
        ARR_W::new(self, 0)
    }
    #[doc = "Bits 0:3 - Dithered part in dithering mode"]
    #[inline(always)]
    #[must_use]
    pub fn dither(&mut self) -> DITHER_W<ARRrs> {
        DITHER_W::new(self, 0)
    }
    #[doc = "Bits 4:19 - Integer part in dithering mode"]
    #[inline(always)]
    #[must_use]
    pub fn integer(&mut self) -> INTEGER_W<ARRrs> {
        INTEGER_W::new(self, 4)
    }
}
#[doc = "TIM7 auto-reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARRrs;
impl crate::RegisterSpec for ARRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arr::R`](R) reader structure"]
impl crate::Readable for ARRrs {}
#[doc = "`write(|w| ..)` method takes [`arr::W`](W) writer structure"]
impl crate::Writable for ARRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARR to value 0xffff"]
impl crate::Resettable for ARRrs {
    const RESET_VALUE: u32 = 0xffff;
}
