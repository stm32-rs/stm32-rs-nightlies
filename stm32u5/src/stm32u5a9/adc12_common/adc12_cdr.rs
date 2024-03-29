#[doc = "Register `ADC12_CDR` reader"]
pub type R = crate::R<ADC12_CDRrs>;
#[doc = "Field `RDATA_MST` reader - Regular data of the master ADC. In dual mode, these bits contain the regular data of the master ADC. Refer to . The data alignment is applied as described in offset (ADC_DR, ADC_JDRy, OFFSETy, OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT)) In DAMDF\\[1:0\\]
= 11 mode, bits 15:8 contains SLV_ADC_DR\\[7:0\\], bits 7:0 contains MST_ADC_DR\\[7:0\\]."]
pub type RDATA_MST_R = crate::FieldReader<u16>;
#[doc = "Field `RDATA_SLV` reader - Regular data of the slave ADC In dual mode, these bits contain the regular data of the slave ADC. Refer to Dual ADC modes. The data alignment is applied as described in offset (ADC_DR, ADC_JDRy, OFFSETy, OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT))"]
pub type RDATA_SLV_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Regular data of the master ADC. In dual mode, these bits contain the regular data of the master ADC. Refer to . The data alignment is applied as described in offset (ADC_DR, ADC_JDRy, OFFSETy, OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT)) In DAMDF\\[1:0\\]
= 11 mode, bits 15:8 contains SLV_ADC_DR\\[7:0\\], bits 7:0 contains MST_ADC_DR\\[7:0\\]."]
    #[inline(always)]
    pub fn rdata_mst(&self) -> RDATA_MST_R {
        RDATA_MST_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Regular data of the slave ADC In dual mode, these bits contain the regular data of the slave ADC. Refer to Dual ADC modes. The data alignment is applied as described in offset (ADC_DR, ADC_JDRy, OFFSETy, OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT))"]
    #[inline(always)]
    pub fn rdata_slv(&self) -> RDATA_SLV_R {
        RDATA_SLV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "ADC common regular data register for dual mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12_cdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC12_CDRrs;
impl crate::RegisterSpec for ADC12_CDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc12_cdr::R`](R) reader structure"]
impl crate::Readable for ADC12_CDRrs {}
#[doc = "`reset()` method sets ADC12_CDR to value 0"]
impl crate::Resettable for ADC12_CDRrs {
    const RESET_VALUE: u32 = 0;
}
