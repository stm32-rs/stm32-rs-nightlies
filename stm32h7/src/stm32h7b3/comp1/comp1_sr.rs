#[doc = "Reader of register COMP1_SR"]
pub type R = crate::R<u32, super::COMP1_SR>;
#[doc = "Reader of field `C1VAL`"]
pub type C1VAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `C2VAL`"]
pub type C2VAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `C1IF`"]
pub type C1IF_R = crate::R<bool, bool>;
#[doc = "Reader of field `C2IF`"]
pub type C2IF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - COMP channel 1 output status bit"]
    #[inline(always)]
    pub fn c1val(&self) -> C1VAL_R {
        C1VAL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - COMP channel 2 output status bit"]
    #[inline(always)]
    pub fn c2val(&self) -> C2VAL_R {
        C2VAL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - COMP channel 1 Interrupt Flag"]
    #[inline(always)]
    pub fn c1if(&self) -> C1IF_R {
        C1IF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - COMP channel 2 Interrupt Flag"]
    #[inline(always)]
    pub fn c2if(&self) -> C2IF_R {
        C2IF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
