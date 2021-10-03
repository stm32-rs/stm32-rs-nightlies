#[doc = "Register `DMAMUX_RGSR` reader"]
pub struct R(crate::R<DMAMUX_RGSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAMUX_RGSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAMUX_RGSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAMUX_RGSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OF0` reader - Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register."]
pub struct OF0_R(crate::FieldReader<bool, bool>);
impl OF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        OF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OF1` reader - Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register."]
pub struct OF1_R(crate::FieldReader<bool, bool>);
impl OF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        OF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OF2` reader - Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register."]
pub struct OF2_R(crate::FieldReader<bool, bool>);
impl OF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        OF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OF3` reader - Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register."]
pub struct OF3_R(crate::FieldReader<bool, bool>);
impl OF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        OF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register."]
    #[inline(always)]
    pub fn of0(&self) -> OF0_R {
        OF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register."]
    #[inline(always)]
    pub fn of1(&self) -> OF1_R {
        OF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register."]
    #[inline(always)]
    pub fn of2(&self) -> OF2_R {
        OF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register."]
    #[inline(always)]
    pub fn of3(&self) -> OF3_R {
        OF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
#[doc = "DMAMUX request generator interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_rgsr](index.html) module"]
pub struct DMAMUX_RGSR_SPEC;
impl crate::RegisterSpec for DMAMUX_RGSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmamux_rgsr::R](R) reader structure"]
impl crate::Readable for DMAMUX_RGSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMAMUX_RGSR to value 0"]
impl crate::Resettable for DMAMUX_RGSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
