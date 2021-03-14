#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `TAMP1F`"]
pub type TAMP1F_R = crate::R<bool, bool>;
#[doc = "Reader of field `TAMP2F`"]
pub type TAMP2F_R = crate::R<bool, bool>;
#[doc = "Reader of field `TAMP3F`"]
pub type TAMP3F_R = crate::R<bool, bool>;
#[doc = "Reader of field `TAMP4F`"]
pub type TAMP4F_R = crate::R<bool, bool>;
#[doc = "Reader of field `TAMP5F`"]
pub type TAMP5F_R = crate::R<bool, bool>;
#[doc = "Reader of field `TAMP6F`"]
pub type TAMP6F_R = crate::R<bool, bool>;
#[doc = "Reader of field `TAMP7F`"]
pub type TAMP7F_R = crate::R<bool, bool>;
#[doc = "Reader of field `TAMP8F`"]
pub type TAMP8F_R = crate::R<bool, bool>;
#[doc = "Reader of field `ITAMP1F`"]
pub type ITAMP1F_R = crate::R<bool, bool>;
#[doc = "Reader of field `ITAMP2F`"]
pub type ITAMP2F_R = crate::R<bool, bool>;
#[doc = "Reader of field `ITAMP3F`"]
pub type ITAMP3F_R = crate::R<bool, bool>;
#[doc = "Reader of field `ITAMP5F`"]
pub type ITAMP5F_R = crate::R<bool, bool>;
#[doc = "Reader of field `ITAMP8F`"]
pub type ITAMP8F_R = crate::R<bool, bool>;
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
    #[doc = "Bit 3 - TAMP4F"]
    #[inline(always)]
    pub fn tamp4f(&self) -> TAMP4F_R {
        TAMP4F_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TAMP5F"]
    #[inline(always)]
    pub fn tamp5f(&self) -> TAMP5F_R {
        TAMP5F_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TAMP6F"]
    #[inline(always)]
    pub fn tamp6f(&self) -> TAMP6F_R {
        TAMP6F_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TAMP7F"]
    #[inline(always)]
    pub fn tamp7f(&self) -> TAMP7F_R {
        TAMP7F_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TAMP8F"]
    #[inline(always)]
    pub fn tamp8f(&self) -> TAMP8F_R {
        TAMP8F_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ITAMP1F"]
    #[inline(always)]
    pub fn itamp1f(&self) -> ITAMP1F_R {
        ITAMP1F_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ITAMP2F"]
    #[inline(always)]
    pub fn itamp2f(&self) -> ITAMP2F_R {
        ITAMP2F_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ITAMP3F"]
    #[inline(always)]
    pub fn itamp3f(&self) -> ITAMP3F_R {
        ITAMP3F_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ITAMP5F"]
    #[inline(always)]
    pub fn itamp5f(&self) -> ITAMP5F_R {
        ITAMP5F_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ITAMP8F"]
    #[inline(always)]
    pub fn itamp8f(&self) -> ITAMP8F_R {
        ITAMP8F_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
