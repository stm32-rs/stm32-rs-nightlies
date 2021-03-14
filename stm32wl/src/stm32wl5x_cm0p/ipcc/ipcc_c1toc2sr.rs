#[doc = "Reader of register IPCC_C1TOC2SR"]
pub type R = crate::R<u32, super::IPCC_C1TOC2SR>;
#[doc = "Reader of field `CH1F`"]
pub type CH1F_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH2F`"]
pub type CH2F_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH3F`"]
pub type CH3F_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH4F`"]
pub type CH4F_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH5F`"]
pub type CH5F_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH6F`"]
pub type CH6F_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - CH1F"]
    #[inline(always)]
    pub fn ch1f(&self) -> CH1F_R {
        CH1F_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CH2F"]
    #[inline(always)]
    pub fn ch2f(&self) -> CH2F_R {
        CH2F_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CH3F"]
    #[inline(always)]
    pub fn ch3f(&self) -> CH3F_R {
        CH3F_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CH4F"]
    #[inline(always)]
    pub fn ch4f(&self) -> CH4F_R {
        CH4F_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CH5F"]
    #[inline(always)]
    pub fn ch5f(&self) -> CH5F_R {
        CH5F_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CH6F"]
    #[inline(always)]
    pub fn ch6f(&self) -> CH6F_R {
        CH6F_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
