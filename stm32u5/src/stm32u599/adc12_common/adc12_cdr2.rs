#[doc = "Register `ADC12_CDR2` reader"]
pub type R = crate::R<ADC12_CDR2rs>;
#[doc = "Field `RDATA_ALT` reader - Regular data of the master/slave alternated ADCs In dual mode, these bits alternatively contains the regular 32-bit data of the master and the slave ADC. Refer to . The data alignment is applied as described in (ADC_DR, ADC_JDRy, OFFSETy, OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT)."]
pub type RDATA_ALT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Regular data of the master/slave alternated ADCs In dual mode, these bits alternatively contains the regular 32-bit data of the master and the slave ADC. Refer to . The data alignment is applied as described in (ADC_DR, ADC_JDRy, OFFSETy, OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT)."]
    #[inline(always)]
    pub fn rdata_alt(&self) -> RDATA_ALT_R {
        RDATA_ALT_R::new(self.bits)
    }
}
#[doc = "ADC common regular data register for 32-bit dual mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12_cdr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC12_CDR2rs;
impl crate::RegisterSpec for ADC12_CDR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc12_cdr2::R`](R) reader structure"]
impl crate::Readable for ADC12_CDR2rs {}
#[doc = "`reset()` method sets ADC12_CDR2 to value 0"]
impl crate::Resettable for ADC12_CDR2rs {
    const RESET_VALUE: u32 = 0;
}
