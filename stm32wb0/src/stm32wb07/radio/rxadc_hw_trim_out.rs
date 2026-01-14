///Register `RXADC_HW_TRIM_OUT` reader
pub type R = crate::R<RXADC_HW_TRIM_OUTrs>;
///Field `HW_RXADC_DELAYTRIM_I` reader - control bits of the RX ADC loop delay for I channel (provided by the HW trimming, automatically loaded on POR).
pub type HW_RXADC_DELAYTRIM_I_R = crate::FieldReader;
///Field `HW_RXADC_DELAYTRIM_Q` reader - control bits of the RX ADC loop delay for Q channel (provided by the HW trimming, automatically loaded on POR).
pub type HW_RXADC_DELAYTRIM_Q_R = crate::FieldReader;
impl R {
    ///Bits 0:2 - control bits of the RX ADC loop delay for I channel (provided by the HW trimming, automatically loaded on POR).
    #[inline(always)]
    pub fn hw_rxadc_delaytrim_i(&self) -> HW_RXADC_DELAYTRIM_I_R {
        HW_RXADC_DELAYTRIM_I_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - control bits of the RX ADC loop delay for Q channel (provided by the HW trimming, automatically loaded on POR).
    #[inline(always)]
    pub fn hw_rxadc_delaytrim_q(&self) -> HW_RXADC_DELAYTRIM_Q_R {
        HW_RXADC_DELAYTRIM_Q_R::new(((self.bits >> 3) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXADC_HW_TRIM_OUT")
            .field("hw_rxadc_delaytrim_i", &self.hw_rxadc_delaytrim_i())
            .field("hw_rxadc_delaytrim_q", &self.hw_rxadc_delaytrim_q())
            .finish()
    }
}
/**RXADC_HW_TRIM_OUT register

You can [`read`](crate::Reg::read) this register and get [`rxadc_hw_trim_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RADIO:RXADC_HW_TRIM_OUT)*/
pub struct RXADC_HW_TRIM_OUTrs;
impl crate::RegisterSpec for RXADC_HW_TRIM_OUTrs {
    type Ux = u32;
}
///`read()` method returns [`rxadc_hw_trim_out::R`](R) reader structure
impl crate::Readable for RXADC_HW_TRIM_OUTrs {}
///`reset()` method sets RXADC_HW_TRIM_OUT to value 0x1b
impl crate::Resettable for RXADC_HW_TRIM_OUTrs {
    const RESET_VALUE: u32 = 0x1b;
}
