#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
#[doc = "Data buffer overrun/underrun interrupt status clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_ISC {
    #[doc = "1: Writing this bit to 1 clears the OVR_RIS bit in PSSI_RIS"]
    Clear = 1,
}
impl From<OVR_ISC> for bool {
    #[inline(always)]
    fn from(variant: OVR_ISC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR_ISC` writer - Data buffer overrun/underrun interrupt status clear"]
pub type OVR_ISC_W<'a, REG> = crate::BitWriter<'a, REG, OVR_ISC>;
impl<'a, REG> OVR_ISC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing this bit to 1 clears the OVR_RIS bit in PSSI_RIS"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OVR_ISC::Clear)
    }
}
impl W {
    #[doc = "Bit 1 - Data buffer overrun/underrun interrupt status clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovr_isc(&mut self) -> OVR_ISC_W<ICRrs> {
        OVR_ISC_W::new(self, 1)
    }
}
#[doc = "PSSI interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
