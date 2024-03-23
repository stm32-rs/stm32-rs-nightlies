#[doc = "Register `MIS` reader"]
pub type R = crate::R<MISrs>;
#[doc = "Data buffer overrun/underrun masked interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_MIS {
    #[doc = "0: No interrupt is generated when an overrun/underrun error occurs"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated if there is either an overrun or an underrun error and the OVR_IE bit is set in PSSI_IER"]
    Enabled = 1,
}
impl From<OVR_MIS> for bool {
    #[inline(always)]
    fn from(variant: OVR_MIS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR_MIS` reader - Data buffer overrun/underrun masked interrupt status"]
pub type OVR_MIS_R = crate::BitReader<OVR_MIS>;
impl OVR_MIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVR_MIS {
        match self.bits {
            false => OVR_MIS::Disabled,
            true => OVR_MIS::Enabled,
        }
    }
    #[doc = "No interrupt is generated when an overrun/underrun error occurs"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVR_MIS::Disabled
    }
    #[doc = "An interrupt is generated if there is either an overrun or an underrun error and the OVR_IE bit is set in PSSI_IER"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVR_MIS::Enabled
    }
}
impl R {
    #[doc = "Bit 1 - Data buffer overrun/underrun masked interrupt status"]
    #[inline(always)]
    pub fn ovr_mis(&self) -> OVR_MIS_R {
        OVR_MIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "PSSI masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISrs;
impl crate::RegisterSpec for MISrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MISrs {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MISrs {
    const RESET_VALUE: u32 = 0;
}
