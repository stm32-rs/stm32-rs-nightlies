#[doc = "Register `CCVALR` reader"]
pub type R = crate::R<CCVALRrs>;
#[doc = "Field `ANSRC1` reader - compensation value for the NMOS transistor This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range."]
pub type ANSRC1_R = crate::FieldReader;
#[doc = "Field `APSRC1` reader - compensation value for the PMOS transistor This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range."]
pub type APSRC1_R = crate::FieldReader;
#[doc = "Field `ANSRC2` reader - Compensation value for the NMOS transistor This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range."]
pub type ANSRC2_R = crate::FieldReader;
#[doc = "Field `APSRC2` reader - compensation value for the PMOS transistor This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range."]
pub type APSRC2_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - compensation value for the NMOS transistor This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range."]
    #[inline(always)]
    pub fn ansrc1(&self) -> ANSRC1_R {
        ANSRC1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - compensation value for the PMOS transistor This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range."]
    #[inline(always)]
    pub fn apsrc1(&self) -> APSRC1_R {
        APSRC1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Compensation value for the NMOS transistor This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range."]
    #[inline(always)]
    pub fn ansrc2(&self) -> ANSRC2_R {
        ANSRC2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - compensation value for the PMOS transistor This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range."]
    #[inline(always)]
    pub fn apsrc2(&self) -> APSRC2_R {
        APSRC2_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "SBS compensation cell for I/Os value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccvalr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCVALRrs;
impl crate::RegisterSpec for CCVALRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccvalr::R`](R) reader structure"]
impl crate::Readable for CCVALRrs {}
#[doc = "`reset()` method sets CCVALR to value 0x88"]
impl crate::Resettable for CCVALRrs {
    const RESET_VALUE: u32 = 0x88;
}
