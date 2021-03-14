#[doc = "Reader of register ETH_MTLISR"]
pub type R = crate::R<u32, super::ETH_MTLISR>;
#[doc = "Reader of field `Q0IS`"]
pub type Q0IS_R = crate::R<bool, bool>;
#[doc = "Reader of field `Q1IS`"]
pub type Q1IS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Q0IS"]
    #[inline(always)]
    pub fn q0is(&self) -> Q0IS_R {
        Q0IS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Q1IS"]
    #[inline(always)]
    pub fn q1is(&self) -> Q1IS_R {
        Q1IS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
