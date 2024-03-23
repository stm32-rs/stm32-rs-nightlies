#[doc = "Register `GICH_VTR` reader"]
pub type R = crate::R<GICH_VTRrs>;
#[doc = "Field `LISTREGS` reader - LISTREGS"]
pub type LISTREGS_R = crate::FieldReader;
#[doc = "Field `PREBITS` reader - PREBITS"]
pub type PREBITS_R = crate::FieldReader;
#[doc = "Field `PRIBITS` reader - PRIBITS"]
pub type PRIBITS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - LISTREGS"]
    #[inline(always)]
    pub fn listregs(&self) -> LISTREGS_R {
        LISTREGS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 26:28 - PREBITS"]
    #[inline(always)]
    pub fn prebits(&self) -> PREBITS_R {
        PREBITS_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - PRIBITS"]
    #[inline(always)]
    pub fn pribits(&self) -> PRIBITS_R {
        PRIBITS_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[doc = "GICH VGIC type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gich_vtr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICH_VTRrs;
impl crate::RegisterSpec for GICH_VTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gich_vtr::R`](R) reader structure"]
impl crate::Readable for GICH_VTRrs {}
#[doc = "`reset()` method sets GICH_VTR to value 0x9000_0003"]
impl crate::Resettable for GICH_VTRrs {
    const RESET_VALUE: u32 = 0x9000_0003;
}
