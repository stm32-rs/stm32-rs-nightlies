#[doc = "Reader of register MISR"]
pub type R = crate::R<u32, super::MISR>;
#[doc = "Reader of field `MUPD`"]
pub type MUPD_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYNC`"]
pub type SYNC_R = crate::R<bool, bool>;
#[doc = "Reader of field `MREP`"]
pub type MREP_R = crate::R<bool, bool>;
#[doc = "Reader of field `MCMP4`"]
pub type MCMP4_R = crate::R<bool, bool>;
#[doc = "Reader of field `MCMP3`"]
pub type MCMP3_R = crate::R<bool, bool>;
#[doc = "Reader of field `MCMP2`"]
pub type MCMP2_R = crate::R<bool, bool>;
#[doc = "Reader of field `MCMP1`"]
pub type MCMP1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 6 - Master Update Interrupt Flag"]
    #[inline(always)]
    pub fn mupd(&self) -> MUPD_R {
        MUPD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Sync Input Interrupt Flag"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Master Repetition Interrupt Flag"]
    #[inline(always)]
    pub fn mrep(&self) -> MREP_R {
        MREP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Master Compare 4 Interrupt Flag"]
    #[inline(always)]
    pub fn mcmp4(&self) -> MCMP4_R {
        MCMP4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Master Compare 3 Interrupt Flag"]
    #[inline(always)]
    pub fn mcmp3(&self) -> MCMP3_R {
        MCMP3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Master Compare 2 Interrupt Flag"]
    #[inline(always)]
    pub fn mcmp2(&self) -> MCMP2_R {
        MCMP2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Master Compare 1 Interrupt Flag"]
    #[inline(always)]
    pub fn mcmp1(&self) -> MCMP1_R {
        MCMP1_R::new((self.bits & 0x01) != 0)
    }
}
