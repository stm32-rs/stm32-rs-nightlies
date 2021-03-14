#[doc = "Reader of register SAI_ASR"]
pub type R = crate::R<u32, super::SAI_ASR>;
#[doc = "Reader of field `OVRUDR`"]
pub type OVRUDR_R = crate::R<bool, bool>;
#[doc = "Reader of field `MUTEDET`"]
pub type MUTEDET_R = crate::R<bool, bool>;
#[doc = "Reader of field `WCKCFG`"]
pub type WCKCFG_R = crate::R<bool, bool>;
#[doc = "Reader of field `FREQ`"]
pub type FREQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `CNRDY`"]
pub type CNRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `AFSDET`"]
pub type AFSDET_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFSDET`"]
pub type LFSDET_R = crate::R<bool, bool>;
#[doc = "Reader of field `FLVL`"]
pub type FLVL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - OVRUDR"]
    #[inline(always)]
    pub fn ovrudr(&self) -> OVRUDR_R {
        OVRUDR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MUTEDET"]
    #[inline(always)]
    pub fn mutedet(&self) -> MUTEDET_R {
        MUTEDET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WCKCFG"]
    #[inline(always)]
    pub fn wckcfg(&self) -> WCKCFG_R {
        WCKCFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FREQ"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CNRDY"]
    #[inline(always)]
    pub fn cnrdy(&self) -> CNRDY_R {
        CNRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AFSDET"]
    #[inline(always)]
    pub fn afsdet(&self) -> AFSDET_R {
        AFSDET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LFSDET"]
    #[inline(always)]
    pub fn lfsdet(&self) -> LFSDET_R {
        LFSDET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - FLVL"]
    #[inline(always)]
    pub fn flvl(&self) -> FLVL_R {
        FLVL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
