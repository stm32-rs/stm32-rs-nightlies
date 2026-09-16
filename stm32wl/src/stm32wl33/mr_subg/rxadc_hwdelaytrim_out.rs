///Register `RXADC_HWDELAYTRIM_OUT` reader
pub type R = crate::R<RXADC_HWDELAYTRIM_OUTrs>;
///Field `RXADC_HW_DELAYTRIM_I` reader - Control bits of the RX ADC loop delay for I channel (from SoC Flash).
pub type RXADC_HW_DELAYTRIM_I_R = crate::FieldReader;
///Field `RXADC_HW_DELAYTRIM_Q` reader - Control bits of the RX ADC loop delay for Q channel (from SoC Flash).
pub type RXADC_HW_DELAYTRIM_Q_R = crate::FieldReader;
impl R {
    ///Bits 0:2 - Control bits of the RX ADC loop delay for I channel (from SoC Flash).
    #[inline(always)]
    pub fn rxadc_hw_delaytrim_i(&self) -> RXADC_HW_DELAYTRIM_I_R {
        RXADC_HW_DELAYTRIM_I_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - Control bits of the RX ADC loop delay for Q channel (from SoC Flash).
    #[inline(always)]
    pub fn rxadc_hw_delaytrim_q(&self) -> RXADC_HW_DELAYTRIM_Q_R {
        RXADC_HW_DELAYTRIM_Q_R::new(((self.bits >> 3) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXADC_HWDELAYTRIM_OUT")
            .field("rxadc_hw_delaytrim_i", &self.rxadc_hw_delaytrim_i())
            .field("rxadc_hw_delaytrim_q", &self.rxadc_hw_delaytrim_q())
            .finish()
    }
}
/**RXADC_HWDELAYTRIM_OUT register

You can [`read`](crate::Reg::read) this register and get [`rxadc_hwdelaytrim_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:RXADC_HWDELAYTRIM_OUT)*/
pub struct RXADC_HWDELAYTRIM_OUTrs;
impl crate::RegisterSpec for RXADC_HWDELAYTRIM_OUTrs {
    type Ux = u32;
}
///`read()` method returns [`rxadc_hwdelaytrim_out::R`](R) reader structure
impl crate::Readable for RXADC_HWDELAYTRIM_OUTrs {}
///`reset()` method sets RXADC_HWDELAYTRIM_OUT to value 0x1b
impl crate::Resettable for RXADC_HWDELAYTRIM_OUTrs {
    const RESET_VALUE: u32 = 0x1b;
}
