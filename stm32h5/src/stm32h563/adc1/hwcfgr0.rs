#[doc = "Register `HWCFGR0` reader"]
pub type R = crate::R<HWCFGR0rs>;
#[doc = "Field `ADCNUM` reader - Number of ADCs implemented"]
pub type ADCNUM_R = crate::FieldReader;
#[doc = "Field `MULPIPE` reader - Number of pipeline stages"]
pub type MULPIPE_R = crate::FieldReader;
#[doc = "Field `OPBITS` reader - Number of option bits"]
pub type OPBITS_R = crate::FieldReader;
#[doc = "Field `IDLEVALUE` reader - Idle value for non-selected channels"]
pub type IDLEVALUE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Number of ADCs implemented"]
    #[inline(always)]
    pub fn adcnum(&self) -> ADCNUM_R {
        ADCNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Number of pipeline stages"]
    #[inline(always)]
    pub fn mulpipe(&self) -> MULPIPE_R {
        MULPIPE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Number of option bits"]
    #[inline(always)]
    pub fn opbits(&self) -> OPBITS_R {
        OPBITS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Idle value for non-selected channels"]
    #[inline(always)]
    pub fn idlevalue(&self) -> IDLEVALUE_R {
        IDLEVALUE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "ADC hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGR0rs;
impl crate::RegisterSpec for HWCFGR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr0::R`](R) reader structure"]
impl crate::Readable for HWCFGR0rs {}
#[doc = "`reset()` method sets HWCFGR0 to value 0x1112"]
impl crate::Resettable for HWCFGR0rs {
    const RESET_VALUE: u32 = 0x1112;
}
