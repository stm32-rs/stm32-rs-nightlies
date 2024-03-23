#[doc = "Register `SYSCFG_ITLINE12` reader"]
pub type R = crate::R<SYSCFG_ITLINE12rs>;
#[doc = "Field `ADC` reader - ADC interrupt request pending"]
pub type ADC_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ADC interrupt request pending"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 12 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline12::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG_ITLINE12rs;
impl crate::RegisterSpec for SYSCFG_ITLINE12rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline12::R`](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE12rs {}
#[doc = "`reset()` method sets SYSCFG_ITLINE12 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE12rs {
    const RESET_VALUE: u32 = 0;
}
