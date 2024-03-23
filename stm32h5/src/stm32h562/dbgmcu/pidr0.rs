#[doc = "Register `PIDR0` reader"]
pub type R = crate::R<PIDR0rs>;
#[doc = "Field `PARTNUM` reader - part number bits \\[7:0\\]"]
pub type PARTNUM_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - part number bits \\[7:0\\]"]
    #[inline(always)]
    pub fn partnum(&self) -> PARTNUM_R {
        PARTNUM_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DBGMCU CoreSight peripheral identity register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIDR0rs;
impl crate::RegisterSpec for PIDR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr0::R`](R) reader structure"]
impl crate::Readable for PIDR0rs {}
#[doc = "`reset()` method sets PIDR0 to value 0"]
impl crate::Resettable for PIDR0rs {
    const RESET_VALUE: u32 = 0;
}
