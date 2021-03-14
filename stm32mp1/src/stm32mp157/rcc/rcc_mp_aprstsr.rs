#[doc = "Reader of register RCC_MP_APRSTSR"]
pub type R = crate::R<u32, super::RCC_MP_APRSTSR>;
#[doc = "Reader of field `RSTTOV`"]
pub type RSTTOV_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 8:14 - RSTTOV"]
    #[inline(always)]
    pub fn rsttov(&self) -> RSTTOV_R {
        RSTTOV_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
