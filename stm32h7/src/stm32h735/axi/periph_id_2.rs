#[doc = "Register `PERIPH_ID_2` reader"]
pub type R = crate::R<PERIPH_ID_2rs>;
#[doc = "Field `JEP106ID` reader - JEP106 Identity bits 4 to 6"]
pub type JEP106ID_R = crate::FieldReader;
#[doc = "Field `JEDEC` reader - JEP106 code flag"]
pub type JEDEC_R = crate::BitReader;
#[doc = "Field `REVISION` reader - Peripheral revision number"]
pub type REVISION_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - JEP106 Identity bits 4 to 6"]
    #[inline(always)]
    pub fn jep106id(&self) -> JEP106ID_R {
        JEP106ID_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - JEP106 code flag"]
    #[inline(always)]
    pub fn jedec(&self) -> JEDEC_R {
        JEDEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Peripheral revision number"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "AXI interconnect - peripheral ID2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`periph_id_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERIPH_ID_2rs;
impl crate::RegisterSpec for PERIPH_ID_2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periph_id_2::R`](R) reader structure"]
impl crate::Readable for PERIPH_ID_2rs {}
#[doc = "`reset()` method sets PERIPH_ID_2 to value 0x04"]
impl crate::Resettable for PERIPH_ID_2rs {
    const RESET_VALUE: u32 = 0x04;
}
