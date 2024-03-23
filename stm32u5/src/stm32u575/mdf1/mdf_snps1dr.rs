#[doc = "Register `MDF_SNPS1DR` reader"]
pub type R = crate::R<MDF_SNPS1DRrs>;
#[doc = "Field `MCICDC` reader - Contains the MCIC decimation counter value at the moment of the last trigger event occurs (MCIC_CNT)"]
pub type MCICDC_R = crate::FieldReader<u16>;
#[doc = "Field `EXTSDR` reader - Extended data size If SNPSFMT = 0 , EXTSDR\\[6:0\\]
contains the bit 7 to 1 of the last valid data processed by the digital filter, If SNPSFMT = 1 , this field contains the INT accumulator counter value at the moment of the last trigger event occurs (INT_CNT)."]
pub type EXTSDR_R = crate::FieldReader;
#[doc = "Field `SDR` reader - Contains the 16 MSB of the last valid data processed by the digital filter."]
pub type SDR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - Contains the MCIC decimation counter value at the moment of the last trigger event occurs (MCIC_CNT)"]
    #[inline(always)]
    pub fn mcicdc(&self) -> MCICDC_R {
        MCICDC_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - Extended data size If SNPSFMT = 0 , EXTSDR\\[6:0\\]
contains the bit 7 to 1 of the last valid data processed by the digital filter, If SNPSFMT = 1 , this field contains the INT accumulator counter value at the moment of the last trigger event occurs (INT_CNT)."]
    #[inline(always)]
    pub fn extsdr(&self) -> EXTSDR_R {
        EXTSDR_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:31 - Contains the 16 MSB of the last valid data processed by the digital filter."]
    #[inline(always)]
    pub fn sdr(&self) -> SDR_R {
        SDR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "This register is used to read the data processed by each digital filter in snapshot mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdf_snps1dr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDF_SNPS1DRrs;
impl crate::RegisterSpec for MDF_SNPS1DRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdf_snps1dr::R`](R) reader structure"]
impl crate::Readable for MDF_SNPS1DRrs {}
#[doc = "`reset()` method sets MDF_SNPS1DR to value 0"]
impl crate::Resettable for MDF_SNPS1DRrs {
    const RESET_VALUE: u32 = 0;
}
