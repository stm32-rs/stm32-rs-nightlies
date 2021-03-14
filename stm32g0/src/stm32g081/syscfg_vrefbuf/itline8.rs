#[doc = "Reader of register ITLINE8"]
pub type R = crate::R<u32, super::ITLINE8>;
#[doc = "Reader of field `UCPD1`"]
pub type UCPD1_R = crate::R<bool, bool>;
#[doc = "Reader of field `UCPD2`"]
pub type UCPD2_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - UCPD1"]
    #[inline(always)]
    pub fn ucpd1(&self) -> UCPD1_R {
        UCPD1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - UCPD2"]
    #[inline(always)]
    pub fn ucpd2(&self) -> UCPD2_R {
        UCPD2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
