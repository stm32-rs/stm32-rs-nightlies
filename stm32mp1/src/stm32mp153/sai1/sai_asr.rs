#[doc = "Register `SAI_ASR` reader"]
pub type R = crate::R<SAI_ASRrs>;
#[doc = "Field `OVRUDR` reader - OVRUDR"]
pub type OVRUDR_R = crate::BitReader;
#[doc = "Field `MUTEDET` reader - MUTEDET"]
pub type MUTEDET_R = crate::BitReader;
#[doc = "Field `WCKCFG` reader - WCKCFG"]
pub type WCKCFG_R = crate::BitReader;
#[doc = "Field `FREQ` reader - FREQ"]
pub type FREQ_R = crate::BitReader;
#[doc = "Field `CNRDY` reader - CNRDY"]
pub type CNRDY_R = crate::BitReader;
#[doc = "Field `AFSDET` reader - AFSDET"]
pub type AFSDET_R = crate::BitReader;
#[doc = "Field `LFSDET` reader - LFSDET"]
pub type LFSDET_R = crate::BitReader;
#[doc = "Field `FLVL` reader - FLVL"]
pub type FLVL_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - OVRUDR"]
    #[inline(always)]
    pub fn ovrudr(&self) -> OVRUDR_R {
        OVRUDR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MUTEDET"]
    #[inline(always)]
    pub fn mutedet(&self) -> MUTEDET_R {
        MUTEDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WCKCFG"]
    #[inline(always)]
    pub fn wckcfg(&self) -> WCKCFG_R {
        WCKCFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FREQ"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CNRDY"]
    #[inline(always)]
    pub fn cnrdy(&self) -> CNRDY_R {
        CNRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AFSDET"]
    #[inline(always)]
    pub fn afsdet(&self) -> AFSDET_R {
        AFSDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LFSDET"]
    #[inline(always)]
    pub fn lfsdet(&self) -> LFSDET_R {
        LFSDET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:18 - FLVL"]
    #[inline(always)]
    pub fn flvl(&self) -> FLVL_R {
        FLVL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_asr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAI_ASRrs;
impl crate::RegisterSpec for SAI_ASRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sai_asr::R`](R) reader structure"]
impl crate::Readable for SAI_ASRrs {}
#[doc = "`reset()` method sets SAI_ASR to value 0x08"]
impl crate::Resettable for SAI_ASRrs {
    const RESET_VALUE: u32 = 0x08;
}
