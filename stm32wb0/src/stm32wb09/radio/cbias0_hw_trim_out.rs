///Register `CBIAS0_HW_TRIM_OUT` reader
pub type R = crate::R<CBIAS0_HW_TRIM_OUTrs>;
///Field `HW_CBIAS_IBIAS_TRIM` reader - CBIAS current (provided by the HW trimming, automatically loaded on POR).
pub type HW_CBIAS_IBIAS_TRIM_R = crate::FieldReader;
///Field `HW_CBIAS_IPTAT_TRIM` reader - CBIAS current (provided by the HW trimming, automatically loaded on POR).
pub type HW_CBIAS_IPTAT_TRIM_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - CBIAS current (provided by the HW trimming, automatically loaded on POR).
    #[inline(always)]
    pub fn hw_cbias_ibias_trim(&self) -> HW_CBIAS_IBIAS_TRIM_R {
        HW_CBIAS_IBIAS_TRIM_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - CBIAS current (provided by the HW trimming, automatically loaded on POR).
    #[inline(always)]
    pub fn hw_cbias_iptat_trim(&self) -> HW_CBIAS_IPTAT_TRIM_R {
        HW_CBIAS_IPTAT_TRIM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CBIAS0_HW_TRIM_OUT")
            .field("hw_cbias_ibias_trim", &self.hw_cbias_ibias_trim())
            .field("hw_cbias_iptat_trim", &self.hw_cbias_iptat_trim())
            .finish()
    }
}
/**CBIAS0_HW_TRIM_OUT register

You can [`read`](crate::Reg::read) this register and get [`cbias0_hw_trim_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:CBIAS0_HW_TRIM_OUT)*/
pub struct CBIAS0_HW_TRIM_OUTrs;
impl crate::RegisterSpec for CBIAS0_HW_TRIM_OUTrs {
    type Ux = u32;
}
///`read()` method returns [`cbias0_hw_trim_out::R`](R) reader structure
impl crate::Readable for CBIAS0_HW_TRIM_OUTrs {}
///`reset()` method sets CBIAS0_HW_TRIM_OUT to value 0x88
impl crate::Resettable for CBIAS0_HW_TRIM_OUTrs {
    const RESET_VALUE: u32 = 0x88;
}
