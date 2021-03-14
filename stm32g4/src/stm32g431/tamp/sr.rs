#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `TAMP1F`"]
pub type TAMP1F_R = crate::R<bool, bool>;
#[doc = "Reader of field `TAMP2F`"]
pub type TAMP2F_R = crate::R<bool, bool>;
#[doc = "Reader of field `TAMP3F`"]
pub type TAMP3F_R = crate::R<bool, bool>;
#[doc = "Reader of field `ITAMP3F`"]
pub type ITAMP3F_R = crate::R<bool, bool>;
#[doc = "Reader of field `ITAMP4F`"]
pub type ITAMP4F_R = crate::R<bool, bool>;
#[doc = "Reader of field `ITAMP5F`"]
pub type ITAMP5F_R = crate::R<bool, bool>;
#[doc = "Reader of field `ITAMP6F`"]
pub type ITAMP6F_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TAMP1F"]
    #[inline(always)]
    pub fn tamp1f(&self) -> TAMP1F_R {
        TAMP1F_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TAMP2F"]
    #[inline(always)]
    pub fn tamp2f(&self) -> TAMP2F_R {
        TAMP2F_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TAMP3F"]
    #[inline(always)]
    pub fn tamp3f(&self) -> TAMP3F_R {
        TAMP3F_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ITAMP3F"]
    #[inline(always)]
    pub fn itamp3f(&self) -> ITAMP3F_R {
        ITAMP3F_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ITAMP4F"]
    #[inline(always)]
    pub fn itamp4f(&self) -> ITAMP4F_R {
        ITAMP4F_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ITAMP5F"]
    #[inline(always)]
    pub fn itamp5f(&self) -> ITAMP5F_R {
        ITAMP5F_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ITAMP6F"]
    #[inline(always)]
    pub fn itamp6f(&self) -> ITAMP6F_R {
        ITAMP6F_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
