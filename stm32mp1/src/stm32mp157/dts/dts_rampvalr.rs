#[doc = "Reader of register DTS_RAMPVALR"]
pub type R = crate::R<u32, super::DTS_RAMPVALR>;
#[doc = "Reader of field `TS1_RAMP_COEFF`"]
pub type TS1_RAMP_COEFF_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - TS1_RAMP_COEFF"]
    #[inline(always)]
    pub fn ts1_ramp_coeff(&self) -> TS1_RAMP_COEFF_R {
        TS1_RAMP_COEFF_R::new((self.bits & 0xffff) as u16)
    }
}
