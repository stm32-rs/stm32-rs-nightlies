#[doc = "Register `PIDR1` reader"]
pub type R = crate::R<PIDR1rs>;
#[doc = "Field `PARTNUM` reader - part number bits \\[11:8\\]"]
pub type PARTNUM_R = crate::FieldReader;
#[doc = "Field `JEP106ID` reader - JEP106 identity code bits \\[3:0\\]"]
pub type JEP106ID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - part number bits \\[11:8\\]"]
    #[inline(always)]
    pub fn partnum(&self) -> PARTNUM_R {
        PARTNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - JEP106 identity code bits \\[3:0\\]"]
    #[inline(always)]
    pub fn jep106id(&self) -> JEP106ID_R {
        JEP106ID_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "DBGMCU CoreSight peripheral identity register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIDR1rs;
impl crate::RegisterSpec for PIDR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr1::R`](R) reader structure"]
impl crate::Readable for PIDR1rs {}
#[doc = "`reset()` method sets PIDR1 to value 0"]
impl crate::Resettable for PIDR1rs {
    const RESET_VALUE: u32 = 0;
}
