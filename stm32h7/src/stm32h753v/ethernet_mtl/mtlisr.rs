#[doc = "Reader of register MTLISR"]
pub type R = crate::R<u32, super::MTLISR>;
#[doc = "Reader of field `Q0IS`"]
pub type Q0IS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Queue interrupt status"]
    #[inline(always)]
    pub fn q0is(&self) -> Q0IS_R {
        Q0IS_R::new((self.bits & 0x01) != 0)
    }
}
