#[doc = "Reader of register DMAMFBOCR"]
pub type R = crate::R<u32, super::DMAMFBOCR>;
#[doc = "Reader of field `MFC`"]
pub type MFC_R = crate::R<u16, u16>;
#[doc = "Reader of field `OMFC`"]
pub type OMFC_R = crate::R<bool, bool>;
#[doc = "Reader of field `MFA`"]
pub type MFA_R = crate::R<u16, u16>;
#[doc = "Reader of field `OFOC`"]
pub type OFOC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:15 - Missed frames by the controller"]
    #[inline(always)]
    pub fn mfc(&self) -> MFC_R {
        MFC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Overflow bit for missed frame counter"]
    #[inline(always)]
    pub fn omfc(&self) -> OMFC_R {
        OMFC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:27 - Missed frames by the application"]
    #[inline(always)]
    pub fn mfa(&self) -> MFA_R {
        MFA_R::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - Overflow bit for FIFO overflow counter"]
    #[inline(always)]
    pub fn ofoc(&self) -> OFOC_R {
        OFOC_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
