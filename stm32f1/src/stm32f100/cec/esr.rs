#[doc = "Reader of register ESR"]
pub type R = crate::R<u32, super::ESR>;
#[doc = "Reader of field `BTE`"]
pub type BTE_R = crate::R<bool, bool>;
#[doc = "Reader of field `BPE`"]
pub type BPE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RBTFE`"]
pub type RBTFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SBE`"]
pub type SBE_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACKE`"]
pub type ACKE_R = crate::R<bool, bool>;
#[doc = "Reader of field `LINE`"]
pub type LINE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TBTFE`"]
pub type TBTFE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Bit timing error"]
    #[inline(always)]
    pub fn bte(&self) -> BTE_R {
        BTE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Bit period error"]
    #[inline(always)]
    pub fn bpe(&self) -> BPE_R {
        BPE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Rx block transfer finished error"]
    #[inline(always)]
    pub fn rbtfe(&self) -> RBTFE_R {
        RBTFE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start bit error"]
    #[inline(always)]
    pub fn sbe(&self) -> SBE_R {
        SBE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Block acknowledge error"]
    #[inline(always)]
    pub fn acke(&self) -> ACKE_R {
        ACKE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Line error"]
    #[inline(always)]
    pub fn line(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Tx block transfer finished error"]
    #[inline(always)]
    pub fn tbtfe(&self) -> TBTFE_R {
        TBTFE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
