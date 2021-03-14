#[doc = "Reader of register DDRPHYC_ZQ0SR1"]
pub type R = crate::R<u8, super::DDRPHYC_ZQ0SR1>;
#[doc = "Reader of field `ZPD`"]
pub type ZPD_R = crate::R<u8, u8>;
#[doc = "Reader of field `ZPU`"]
pub type ZPU_R = crate::R<u8, u8>;
#[doc = "Reader of field `OPD`"]
pub type OPD_R = crate::R<u8, u8>;
#[doc = "Reader of field `OPU`"]
pub type OPU_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - ZPD"]
    #[inline(always)]
    pub fn zpd(&self) -> ZPD_R {
        ZPD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - ZPU"]
    #[inline(always)]
    pub fn zpu(&self) -> ZPU_R {
        ZPU_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - OPD"]
    #[inline(always)]
    pub fn opd(&self) -> OPD_R {
        OPD_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - OPU"]
    #[inline(always)]
    pub fn opu(&self) -> OPU_R {
        OPU_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
