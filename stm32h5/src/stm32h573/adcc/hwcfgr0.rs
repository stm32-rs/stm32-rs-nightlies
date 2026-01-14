///Register `HWCFGR0` reader
pub type R = crate::R<HWCFGR0rs>;
///Field `ADCNUM` reader - Number of ADCs implemented
pub type ADCNUM_R = crate::FieldReader;
///Field `MULPIPE` reader - Number of pipeline stages
pub type MULPIPE_R = crate::FieldReader;
///Field `OPBITS` reader - Number of option bits
pub type OPBITS_R = crate::FieldReader;
///Field `IDLEVALUE` reader - Idle value for non-selected channels
pub type IDLEVALUE_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - Number of ADCs implemented
    #[inline(always)]
    pub fn adcnum(&self) -> ADCNUM_R {
        ADCNUM_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Number of pipeline stages
    #[inline(always)]
    pub fn mulpipe(&self) -> MULPIPE_R {
        MULPIPE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Number of option bits
    #[inline(always)]
    pub fn opbits(&self) -> OPBITS_R {
        OPBITS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Idle value for non-selected channels
    #[inline(always)]
    pub fn idlevalue(&self) -> IDLEVALUE_R {
        IDLEVALUE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR0")
            .field("adcnum", &self.adcnum())
            .field("mulpipe", &self.mulpipe())
            .field("opbits", &self.opbits())
            .field("idlevalue", &self.idlevalue())
            .finish()
    }
}
/**ADC hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#ADCC:HWCFGR0)*/
pub struct HWCFGR0rs;
impl crate::RegisterSpec for HWCFGR0rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr0::R`](R) reader structure
impl crate::Readable for HWCFGR0rs {}
///`reset()` method sets HWCFGR0 to value 0x1212
impl crate::Resettable for HWCFGR0rs {
    const RESET_VALUE: u32 = 0x1212;
}
