#[doc = "Register `MPCWM4AR` reader"]
pub type R = crate::R<MPCWM4ARrs>;
#[doc = "Register `MPCWM4AR` writer"]
pub type W = crate::W<MPCWM4ARrs>;
#[doc = "Field `SUBA_START` reader - Start of sub-region A in region x This field defines the address offset of the sub-region A, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value)."]
pub type SUBA_START_R = crate::FieldReader<u16>;
#[doc = "Field `SUBA_START` writer - Start of sub-region A in region x This field defines the address offset of the sub-region A, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value)."]
pub type SUBA_START_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SUBA_LENGTH` reader - Length of sub-region A in region x This field defines the length of the sub-region A, to be multiplied by the granularity defined in Table�30. When SUBA_START + SUBA_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBA_LENGTH is applied automatically. If SUBA_LENGTH = 0, the sub-region A is disabled.(SREN bit in GTZC1_TZSC_MPCMWxACFGR is cleared)."]
pub type SUBA_LENGTH_R = crate::FieldReader<u16>;
#[doc = "Field `SUBA_LENGTH` writer - Length of sub-region A in region x This field defines the length of the sub-region A, to be multiplied by the granularity defined in Table�30. When SUBA_START + SUBA_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBA_LENGTH is applied automatically. If SUBA_LENGTH = 0, the sub-region A is disabled.(SREN bit in GTZC1_TZSC_MPCMWxACFGR is cleared)."]
pub type SUBA_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:10 - Start of sub-region A in region x This field defines the address offset of the sub-region A, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value)."]
    #[inline(always)]
    pub fn suba_start(&self) -> SUBA_START_R {
        SUBA_START_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:27 - Length of sub-region A in region x This field defines the length of the sub-region A, to be multiplied by the granularity defined in Table�30. When SUBA_START + SUBA_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBA_LENGTH is applied automatically. If SUBA_LENGTH = 0, the sub-region A is disabled.(SREN bit in GTZC1_TZSC_MPCMWxACFGR is cleared)."]
    #[inline(always)]
    pub fn suba_length(&self) -> SUBA_LENGTH_R {
        SUBA_LENGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Start of sub-region A in region x This field defines the address offset of the sub-region A, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value)."]
    #[inline(always)]
    #[must_use]
    pub fn suba_start(&mut self) -> SUBA_START_W<MPCWM4ARrs> {
        SUBA_START_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Length of sub-region A in region x This field defines the length of the sub-region A, to be multiplied by the granularity defined in Table�30. When SUBA_START + SUBA_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBA_LENGTH is applied automatically. If SUBA_LENGTH = 0, the sub-region A is disabled.(SREN bit in GTZC1_TZSC_MPCMWxACFGR is cleared)."]
    #[inline(always)]
    #[must_use]
    pub fn suba_length(&mut self) -> SUBA_LENGTH_W<MPCWM4ARrs> {
        SUBA_LENGTH_W::new(self, 16)
    }
}
#[doc = "GTZC1 TZSC memory 4 sub-region A watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm4ar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm4ar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPCWM4ARrs;
impl crate::RegisterSpec for MPCWM4ARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpcwm4ar::R`](R) reader structure"]
impl crate::Readable for MPCWM4ARrs {}
#[doc = "`write(|w| ..)` method takes [`mpcwm4ar::W`](W) writer structure"]
impl crate::Writable for MPCWM4ARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPCWM4AR to value 0"]
impl crate::Resettable for MPCWM4ARrs {
    const RESET_VALUE: u32 = 0;
}
