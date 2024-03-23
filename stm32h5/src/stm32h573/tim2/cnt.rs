#[doc = "Register `CNT` reader"]
pub type R = crate::R<CNTrs>;
#[doc = "Register `CNT` writer"]
pub type W = crate::W<CNTrs>;
#[doc = "Field `CNT` reader - Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register holds the non-dithered part. The fractional part is not available."]
pub type CNT_R = crate::FieldReader<u32>;
#[doc = "Field `CNT` writer - Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register holds the non-dithered part. The fractional part is not available."]
pub type CNT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
#[doc = "Read-only copy of the UIF bit of the TIMx_ISR register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIFCPYR {
    #[doc = "0: No update occurred"]
    NoUpdateOccurred = 0,
    #[doc = "1: Update interrupt pending"]
    UpdatePending = 1,
}
impl From<UIFCPYR> for bool {
    #[inline(always)]
    fn from(variant: UIFCPYR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UIFCPY` reader - Read-only copy of the UIF bit of the TIMx_ISR register"]
pub type UIFCPY_R = crate::BitReader<UIFCPYR>;
impl UIFCPY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UIFCPYR {
        match self.bits {
            false => UIFCPYR::NoUpdateOccurred,
            true => UIFCPYR::UpdatePending,
        }
    }
    #[doc = "No update occurred"]
    #[inline(always)]
    pub fn is_no_update_occurred(&self) -> bool {
        *self == UIFCPYR::NoUpdateOccurred
    }
    #[doc = "Update interrupt pending"]
    #[inline(always)]
    pub fn is_update_pending(&self) -> bool {
        *self == UIFCPYR::UpdatePending
    }
}
impl R {
    #[doc = "Bits 0:31 - Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register holds the non-dithered part. The fractional part is not available."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
    #[doc = "Bit 31 - Read-only copy of the UIF bit of the TIMx_ISR register"]
    #[inline(always)]
    pub fn uifcpy(&self) -> UIFCPY_R {
        UIFCPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:31 - Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register holds the non-dithered part. The fractional part is not available."]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<CNTrs> {
        CNT_W::new(self, 0)
    }
}
#[doc = "TIM2 counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTrs;
impl crate::RegisterSpec for CNTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt::R`](R) reader structure"]
impl crate::Readable for CNTrs {}
#[doc = "`write(|w| ..)` method takes [`cnt::W`](W) writer structure"]
impl crate::Writable for CNTrs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CNTrs {
    const RESET_VALUE: u32 = 0;
}
