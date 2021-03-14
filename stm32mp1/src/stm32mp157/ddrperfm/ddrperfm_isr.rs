#[doc = "Reader of register DDRPERFM_ISR"]
pub type R = crate::R<u32, super::DDRPERFM_ISR>;
#[doc = "Reader of field `OVFF`"]
pub type OVFF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - OVFF"]
    #[inline(always)]
    pub fn ovff(&self) -> OVFF_R {
        OVFF_R::new((self.bits & 0x01) != 0)
    }
}
