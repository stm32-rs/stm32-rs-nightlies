#[doc = "Register `PIDR4` reader"]
pub type R = crate::R<PIDR4rs>;
#[doc = "Field `JEP106CON` reader - JEP106 continuation code"]
pub type JEP106CON_R = crate::FieldReader;
#[doc = "Field `SIZE` reader - register file size"]
pub type SIZE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - JEP106 continuation code"]
    #[inline(always)]
    pub fn jep106con(&self) -> JEP106CON_R {
        JEP106CON_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - register file size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "DBGMCU CoreSight peripheral identity register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIDR4rs;
impl crate::RegisterSpec for PIDR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr4::R`](R) reader structure"]
impl crate::Readable for PIDR4rs {}
#[doc = "`reset()` method sets PIDR4 to value 0"]
impl crate::Resettable for PIDR4rs {
    const RESET_VALUE: u32 = 0;
}
