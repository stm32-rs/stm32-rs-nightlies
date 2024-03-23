#[doc = "Register `DAC_DOR1` reader"]
pub type R = crate::R<DAC_DOR1rs>;
#[doc = "Field `DACC1DOR` reader - DACC1DOR"]
pub type DACC1DOR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - DACC1DOR"]
    #[inline(always)]
    pub fn dacc1dor(&self) -> DACC1DOR_R {
        DACC1DOR_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DAC channel1 data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dor1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC_DOR1rs;
impl crate::RegisterSpec for DAC_DOR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_dor1::R`](R) reader structure"]
impl crate::Readable for DAC_DOR1rs {}
#[doc = "`reset()` method sets DAC_DOR1 to value 0"]
impl crate::Resettable for DAC_DOR1rs {
    const RESET_VALUE: u32 = 0;
}
