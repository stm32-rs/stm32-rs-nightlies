#[doc = "Register `MDMA_C18ESR` reader"]
pub type R = crate::R<MDMA_C18ESRrs>;
#[doc = "Field `TEA` reader - TEA"]
pub type TEA_R = crate::FieldReader;
#[doc = "Field `TED` reader - TED"]
pub type TED_R = crate::BitReader;
#[doc = "Field `TELD` reader - TELD"]
pub type TELD_R = crate::BitReader;
#[doc = "Field `TEMD` reader - TEMD"]
pub type TEMD_R = crate::BitReader;
#[doc = "Field `ASE` reader - ASE"]
pub type ASE_R = crate::BitReader;
#[doc = "Field `BSE` reader - BSE"]
pub type BSE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - TEA"]
    #[inline(always)]
    pub fn tea(&self) -> TEA_R {
        TEA_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - TED"]
    #[inline(always)]
    pub fn ted(&self) -> TED_R {
        TED_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TELD"]
    #[inline(always)]
    pub fn teld(&self) -> TELD_R {
        TELD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TEMD"]
    #[inline(always)]
    pub fn temd(&self) -> TEMD_R {
        TEMD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ASE"]
    #[inline(always)]
    pub fn ase(&self) -> ASE_R {
        ASE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BSE"]
    #[inline(always)]
    pub fn bse(&self) -> BSE_R {
        BSE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "MDMA channel 18 error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c18esr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C18ESRrs;
impl crate::RegisterSpec for MDMA_C18ESRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c18esr::R`](R) reader structure"]
impl crate::Readable for MDMA_C18ESRrs {}
#[doc = "`reset()` method sets MDMA_C18ESR to value 0"]
impl crate::Resettable for MDMA_C18ESRrs {
    const RESET_VALUE: u32 = 0;
}
