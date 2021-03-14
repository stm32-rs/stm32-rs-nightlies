#[doc = "Reader of register PRAR_CUR"]
pub type R = crate::R<u32, super::PRAR_CUR>;
#[doc = "Reader of field `PROT_AREA_START`"]
pub type PROT_AREA_START_R = crate::R<u16, u16>;
#[doc = "Reader of field `PROT_AREA_END`"]
pub type PROT_AREA_END_R = crate::R<u16, u16>;
#[doc = "Reader of field `DMEP`"]
pub type DMEP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:11 - Bank 1 lowest PCROP protected address"]
    #[inline(always)]
    pub fn prot_area_start(&self) -> PROT_AREA_START_R {
        PROT_AREA_START_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Bank 1 highest PCROP protected address"]
    #[inline(always)]
    pub fn prot_area_end(&self) -> PROT_AREA_END_R {
        PROT_AREA_END_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Bank 1 PCROP protected erase enable option status bit"]
    #[inline(always)]
    pub fn dmep(&self) -> DMEP_R {
        DMEP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
