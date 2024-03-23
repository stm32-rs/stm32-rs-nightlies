#[doc = "Register `TIM1_ARR` reader"]
pub type R = crate::R<TIM1_ARRrs>;
#[doc = "Register `TIM1_ARR` writer"]
pub type W = crate::W<TIM1_ARRrs>;
#[doc = "Field `ARR` reader - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null. Non-dithering mode (DITHEN = 0) The register holds the auto-reload value. Dithering mode (DITHEN = 1) The register holds the integer part in ARR\\[19:4\\]. The ARR\\[3:0\\]
bitfield contains the dithered part."]
pub type ARR_R = crate::FieldReader<u32>;
#[doc = "Field `ARR` writer - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null. Non-dithering mode (DITHEN = 0) The register holds the auto-reload value. Dithering mode (DITHEN = 1) The register holds the integer part in ARR\\[19:4\\]. The ARR\\[3:0\\]
bitfield contains the dithered part."]
pub type ARR_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null. Non-dithering mode (DITHEN = 0) The register holds the auto-reload value. Dithering mode (DITHEN = 1) The register holds the integer part in ARR\\[19:4\\]. The ARR\\[3:0\\]
bitfield contains the dithered part."]
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null. Non-dithering mode (DITHEN = 0) The register holds the auto-reload value. Dithering mode (DITHEN = 1) The register holds the integer part in ARR\\[19:4\\]. The ARR\\[3:0\\]
bitfield contains the dithered part."]
    #[inline(always)]
    #[must_use]
    pub fn arr(&mut self) -> ARR_W<TIM1_ARRrs> {
        ARR_W::new(self, 0)
    }
}
#[doc = "TIM1 auto-reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_arr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_arr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM1_ARRrs;
impl crate::RegisterSpec for TIM1_ARRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_arr::R`](R) reader structure"]
impl crate::Readable for TIM1_ARRrs {}
#[doc = "`write(|w| ..)` method takes [`tim1_arr::W`](W) writer structure"]
impl crate::Writable for TIM1_ARRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_ARR to value 0xffff"]
impl crate::Resettable for TIM1_ARRrs {
    const RESET_VALUE: u32 = 0xffff;
}
