#[doc = "Reader of register C1TO2SR"]
pub type R = crate::R<u32, super::C1TO2SR>;
#[doc = "Reader of field `CH6F`"]
pub type CH6F_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH5F`"]
pub type CH5F_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH4F`"]
pub type CH4F_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH3F`"]
pub type CH3F_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH2F`"]
pub type CH2F_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH1F`"]
pub type CH1F_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 5 - processor 1 transmit to process 2 Receive channel 6 status flag"]
    #[inline(always)]
    pub fn ch6f(&self) -> CH6F_R {
        CH6F_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - processor 1 transmit to process 2 Receive channel 5 status flag"]
    #[inline(always)]
    pub fn ch5f(&self) -> CH5F_R {
        CH5F_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - processor 1 transmit to process 2 Receive channel 4 status flag"]
    #[inline(always)]
    pub fn ch4f(&self) -> CH4F_R {
        CH4F_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - processor 1 transmit to process 2 Receive channel 3 status flag"]
    #[inline(always)]
    pub fn ch3f(&self) -> CH3F_R {
        CH3F_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - processor 1 transmit to process 2 Receive channel 2 status flag"]
    #[inline(always)]
    pub fn ch2f(&self) -> CH2F_R {
        CH2F_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - processor 1 transmit to process 2 Receive channel 1 status flag"]
    #[inline(always)]
    pub fn ch1f(&self) -> CH1F_R {
        CH1F_R::new((self.bits & 0x01) != 0)
    }
}
