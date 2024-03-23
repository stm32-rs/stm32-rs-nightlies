#[doc = "Register `RIS` reader"]
pub type R = crate::R<RISrs>;
#[doc = "Data buffer overrun/underrun raw interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_RIS {
    #[doc = "0: No overrun/underrun occurred"]
    Cleared = 0,
    #[doc = "1: An overrun/underrun occurred: overrun in receive mode, underrun in transmit mode. This bit is cleared by writing a 1 to the OVR_ISC bit in PSSI_ICR"]
    Occurred = 1,
}
impl From<OVR_RIS> for bool {
    #[inline(always)]
    fn from(variant: OVR_RIS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR_RIS` reader - Data buffer overrun/underrun raw interrupt status"]
pub type OVR_RIS_R = crate::BitReader<OVR_RIS>;
impl OVR_RIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVR_RIS {
        match self.bits {
            false => OVR_RIS::Cleared,
            true => OVR_RIS::Occurred,
        }
    }
    #[doc = "No overrun/underrun occurred"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == OVR_RIS::Cleared
    }
    #[doc = "An overrun/underrun occurred: overrun in receive mode, underrun in transmit mode. This bit is cleared by writing a 1 to the OVR_ISC bit in PSSI_ICR"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == OVR_RIS::Occurred
    }
}
impl R {
    #[doc = "Bit 1 - Data buffer overrun/underrun raw interrupt status"]
    #[inline(always)]
    pub fn ovr_ris(&self) -> OVR_RIS_R {
        OVR_RIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "PSSI raw interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RISrs;
impl crate::RegisterSpec for RISrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RISrs {}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RISrs {
    const RESET_VALUE: u32 = 0;
}
