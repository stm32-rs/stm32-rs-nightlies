#[doc = "Register `ASR` reader"]
pub type R = crate::R<ASRrs>;
#[doc = "Field `OVRUDR` reader - Overrun / underrun"]
pub type OVRUDR_R = crate::BitReader;
#[doc = "Field `MUTEDET` reader - Mute detection"]
pub type MUTEDET_R = crate::BitReader;
#[doc = "Field `WCKCFG` reader - Wrong clock configuration flag. This bit is read only"]
pub type WCKCFG_R = crate::BitReader;
#[doc = "Field `FREQ` reader - FIFO request"]
pub type FREQ_R = crate::BitReader;
#[doc = "Field `CNRDY` reader - Codec not ready"]
pub type CNRDY_R = crate::BitReader;
#[doc = "Field `AFSDET` reader - Anticipated frame synchronization detection"]
pub type AFSDET_R = crate::BitReader;
#[doc = "Field `LFSDET` reader - Late frame synchronization detection"]
pub type LFSDET_R = crate::BitReader;
#[doc = "Field `FLVL` reader - FIFO level threshold"]
pub type FLVL_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Overrun / underrun"]
    #[inline(always)]
    pub fn ovrudr(&self) -> OVRUDR_R {
        OVRUDR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mute detection"]
    #[inline(always)]
    pub fn mutedet(&self) -> MUTEDET_R {
        MUTEDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wrong clock configuration flag. This bit is read only"]
    #[inline(always)]
    pub fn wckcfg(&self) -> WCKCFG_R {
        WCKCFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO request"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Codec not ready"]
    #[inline(always)]
    pub fn cnrdy(&self) -> CNRDY_R {
        CNRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Anticipated frame synchronization detection"]
    #[inline(always)]
    pub fn afsdet(&self) -> AFSDET_R {
        AFSDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Late frame synchronization detection"]
    #[inline(always)]
    pub fn lfsdet(&self) -> LFSDET_R {
        LFSDET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:18 - FIFO level threshold"]
    #[inline(always)]
    pub fn flvl(&self) -> FLVL_R {
        FLVL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
#[doc = "A Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ASRrs;
impl crate::RegisterSpec for ASRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`asr::R`](R) reader structure"]
impl crate::Readable for ASRrs {}
#[doc = "`reset()` method sets ASR to value 0x08"]
impl crate::Resettable for ASRrs {
    const RESET_VALUE: u32 = 0x08;
}
