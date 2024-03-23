#[doc = "Register `CDR` reader"]
pub type R = crate::R<CDRrs>;
#[doc = "Field `DATA1` reader - 1st data item of a pair of regular conversions"]
pub type DATA1_R = crate::FieldReader<u16>;
#[doc = "Field `DATA2` reader - 2nd data item of a pair of regular conversions"]
pub type DATA2_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 1st data item of a pair of regular conversions"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 2nd data item of a pair of regular conversions"]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "ADC common regular data register for dual and triple modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDRrs;
impl crate::RegisterSpec for CDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdr::R`](R) reader structure"]
impl crate::Readable for CDRrs {}
#[doc = "`reset()` method sets CDR to value 0"]
impl crate::Resettable for CDRrs {
    const RESET_VALUE: u32 = 0;
}
