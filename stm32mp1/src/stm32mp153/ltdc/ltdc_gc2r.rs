#[doc = "Register `LTDC_GC2R` reader"]
pub type R = crate::R<LTDC_GC2Rrs>;
#[doc = "Field `EDCEN` reader - EDCEN"]
pub type EDCEN_R = crate::BitReader;
#[doc = "Field `STSAEN` reader - STSAEN"]
pub type STSAEN_R = crate::BitReader;
#[doc = "Field `DVAEN` reader - DVAEN"]
pub type DVAEN_R = crate::BitReader;
#[doc = "Field `DPAEN` reader - DPAEN"]
pub type DPAEN_R = crate::BitReader;
#[doc = "Field `BW` reader - BW"]
pub type BW_R = crate::FieldReader;
#[doc = "Field `EDCA` reader - EDCA"]
pub type EDCA_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - EDCEN"]
    #[inline(always)]
    pub fn edcen(&self) -> EDCEN_R {
        EDCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STSAEN"]
    #[inline(always)]
    pub fn stsaen(&self) -> STSAEN_R {
        STSAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DVAEN"]
    #[inline(always)]
    pub fn dvaen(&self) -> DVAEN_R {
        DVAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DPAEN"]
    #[inline(always)]
    pub fn dpaen(&self) -> DPAEN_R {
        DPAEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - BW"]
    #[inline(always)]
    pub fn bw(&self) -> BW_R {
        BW_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - EDCA"]
    #[inline(always)]
    pub fn edca(&self) -> EDCA_R {
        EDCA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "LTDC global configuration 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_gc2r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_GC2Rrs;
impl crate::RegisterSpec for LTDC_GC2Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_gc2r::R`](R) reader structure"]
impl crate::Readable for LTDC_GC2Rrs {}
#[doc = "`reset()` method sets LTDC_GC2R to value 0x30"]
impl crate::Resettable for LTDC_GC2Rrs {
    const RESET_VALUE: u32 = 0x30;
}
