#[doc = "Register `MPCWM4BR` reader"]
pub type R = crate::R<MPCWM4BRrs>;
#[doc = "Register `MPCWM4BR` writer"]
pub type W = crate::W<MPCWM4BRrs>;
#[doc = "Field `SUBB_START` reader - Start of sub-region B in region x This field defines the address offset of the sub-region B, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value)."]
pub type SUBB_START_R = crate::FieldReader<u16>;
#[doc = "Field `SUBB_START` writer - Start of sub-region B in region x This field defines the address offset of the sub-region B, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value)."]
pub type SUBB_START_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SUBB_LENGTH` reader - Length of sub-region B in region x This field defines the length of the sub-region B, to be multiplied by the granularity defined in Table�30. When SUBB_START + SUBB_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBB_LENGTH is applied automatically. If SUBB_LENGTH = 0, the sub-region B is disabled.(SREN bit in GTZC1_TZSC_MPCMWxBCFGR is cleared)."]
pub type SUBB_LENGTH_R = crate::FieldReader<u16>;
#[doc = "Field `SUBB_LENGTH` writer - Length of sub-region B in region x This field defines the length of the sub-region B, to be multiplied by the granularity defined in Table�30. When SUBB_START + SUBB_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBB_LENGTH is applied automatically. If SUBB_LENGTH = 0, the sub-region B is disabled.(SREN bit in GTZC1_TZSC_MPCMWxBCFGR is cleared)."]
pub type SUBB_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:10 - Start of sub-region B in region x This field defines the address offset of the sub-region B, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value)."]
    #[inline(always)]
    pub fn subb_start(&self) -> SUBB_START_R {
        SUBB_START_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:27 - Length of sub-region B in region x This field defines the length of the sub-region B, to be multiplied by the granularity defined in Table�30. When SUBB_START + SUBB_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBB_LENGTH is applied automatically. If SUBB_LENGTH = 0, the sub-region B is disabled.(SREN bit in GTZC1_TZSC_MPCMWxBCFGR is cleared)."]
    #[inline(always)]
    pub fn subb_length(&self) -> SUBB_LENGTH_R {
        SUBB_LENGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Start of sub-region B in region x This field defines the address offset of the sub-region B, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value)."]
    #[inline(always)]
    #[must_use]
    pub fn subb_start(&mut self) -> SUBB_START_W<MPCWM4BRrs> {
        SUBB_START_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Length of sub-region B in region x This field defines the length of the sub-region B, to be multiplied by the granularity defined in Table�30. When SUBB_START + SUBB_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBB_LENGTH is applied automatically. If SUBB_LENGTH = 0, the sub-region B is disabled.(SREN bit in GTZC1_TZSC_MPCMWxBCFGR is cleared)."]
    #[inline(always)]
    #[must_use]
    pub fn subb_length(&mut self) -> SUBB_LENGTH_W<MPCWM4BRrs> {
        SUBB_LENGTH_W::new(self, 16)
    }
}
#[doc = "GTZC1 TZSC memory 4 sub-region B watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm4br::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm4br::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPCWM4BRrs;
impl crate::RegisterSpec for MPCWM4BRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpcwm4br::R`](R) reader structure"]
impl crate::Readable for MPCWM4BRrs {}
#[doc = "`write(|w| ..)` method takes [`mpcwm4br::W`](W) writer structure"]
impl crate::Writable for MPCWM4BRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPCWM4BR to value 0"]
impl crate::Resettable for MPCWM4BRrs {
    const RESET_VALUE: u32 = 0;
}
