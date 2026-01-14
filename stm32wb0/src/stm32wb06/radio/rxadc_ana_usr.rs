///Register `RXADC_ANA_USR` reader
pub type R = crate::R<RXADC_ANA_USRrs>;
///Register `RXADC_ANA_USR` writer
pub type W = crate::W<RXADC_ANA_USRrs>;
///Field `RFD_RXADC_DELAYTRIM_I` reader - ADC loop delay control bits for I channel to apply when SW overload is enabled
pub type RFD_RXADC_DELAYTRIM_I_R = crate::FieldReader;
///Field `RFD_RXADC_DELAYTRIM_I` writer - ADC loop delay control bits for I channel to apply when SW overload is enabled
pub type RFD_RXADC_DELAYTRIM_I_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RFD_RXADC_DELAYTRIM_Q` reader - ADC loop delay control bits for Q channel to apply when SW overload is enabled
pub type RFD_RXADC_DELAYTRIM_Q_R = crate::FieldReader;
///Field `RFD_RXADC_DELAYTRIM_Q` writer - ADC loop delay control bits for Q channel to apply when SW overload is enabled
pub type RFD_RXADC_DELAYTRIM_Q_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RXADC_DELAYTRIM_I_TST_SEL` reader - Enable the SW overload on RXADX delay trimming
pub type RXADC_DELAYTRIM_I_TST_SEL_R = crate::BitReader;
///Field `RXADC_DELAYTRIM_I_TST_SEL` writer - Enable the SW overload on RXADX delay trimming
pub type RXADC_DELAYTRIM_I_TST_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXADC_DELAYTRIM_Q_TST_SEL` reader - Enable the SW overload on RXADX delay trimming
pub type RXADC_DELAYTRIM_Q_TST_SEL_R = crate::BitReader;
///Field `RXADC_DELAYTRIM_Q_TST_SEL` writer - Enable the SW overload on RXADX delay trimming
pub type RXADC_DELAYTRIM_Q_TST_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - ADC loop delay control bits for I channel to apply when SW overload is enabled
    #[inline(always)]
    pub fn rfd_rxadc_delaytrim_i(&self) -> RFD_RXADC_DELAYTRIM_I_R {
        RFD_RXADC_DELAYTRIM_I_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - ADC loop delay control bits for Q channel to apply when SW overload is enabled
    #[inline(always)]
    pub fn rfd_rxadc_delaytrim_q(&self) -> RFD_RXADC_DELAYTRIM_Q_R {
        RFD_RXADC_DELAYTRIM_Q_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bit 6 - Enable the SW overload on RXADX delay trimming
    #[inline(always)]
    pub fn rxadc_delaytrim_i_tst_sel(&self) -> RXADC_DELAYTRIM_I_TST_SEL_R {
        RXADC_DELAYTRIM_I_TST_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Enable the SW overload on RXADX delay trimming
    #[inline(always)]
    pub fn rxadc_delaytrim_q_tst_sel(&self) -> RXADC_DELAYTRIM_Q_TST_SEL_R {
        RXADC_DELAYTRIM_Q_TST_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXADC_ANA_USR")
            .field("rfd_rxadc_delaytrim_i", &self.rfd_rxadc_delaytrim_i())
            .field("rfd_rxadc_delaytrim_q", &self.rfd_rxadc_delaytrim_q())
            .field(
                "rxadc_delaytrim_i_tst_sel",
                &self.rxadc_delaytrim_i_tst_sel(),
            )
            .field(
                "rxadc_delaytrim_q_tst_sel",
                &self.rxadc_delaytrim_q_tst_sel(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:2 - ADC loop delay control bits for I channel to apply when SW overload is enabled
    #[inline(always)]
    pub fn rfd_rxadc_delaytrim_i(&mut self) -> RFD_RXADC_DELAYTRIM_I_W<'_, RXADC_ANA_USRrs> {
        RFD_RXADC_DELAYTRIM_I_W::new(self, 0)
    }
    ///Bits 3:5 - ADC loop delay control bits for Q channel to apply when SW overload is enabled
    #[inline(always)]
    pub fn rfd_rxadc_delaytrim_q(&mut self) -> RFD_RXADC_DELAYTRIM_Q_W<'_, RXADC_ANA_USRrs> {
        RFD_RXADC_DELAYTRIM_Q_W::new(self, 3)
    }
    ///Bit 6 - Enable the SW overload on RXADX delay trimming
    #[inline(always)]
    pub fn rxadc_delaytrim_i_tst_sel(
        &mut self,
    ) -> RXADC_DELAYTRIM_I_TST_SEL_W<'_, RXADC_ANA_USRrs> {
        RXADC_DELAYTRIM_I_TST_SEL_W::new(self, 6)
    }
    ///Bit 7 - Enable the SW overload on RXADX delay trimming
    #[inline(always)]
    pub fn rxadc_delaytrim_q_tst_sel(
        &mut self,
    ) -> RXADC_DELAYTRIM_Q_TST_SEL_W<'_, RXADC_ANA_USRrs> {
        RXADC_DELAYTRIM_Q_TST_SEL_W::new(self, 7)
    }
}
/**RXADC_ANA_USR register

You can [`read`](crate::Reg::read) this register and get [`rxadc_ana_usr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxadc_ana_usr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#RADIO:RXADC_ANA_USR)*/
pub struct RXADC_ANA_USRrs;
impl crate::RegisterSpec for RXADC_ANA_USRrs {
    type Ux = u32;
}
///`read()` method returns [`rxadc_ana_usr::R`](R) reader structure
impl crate::Readable for RXADC_ANA_USRrs {}
///`write(|w| ..)` method takes [`rxadc_ana_usr::W`](W) writer structure
impl crate::Writable for RXADC_ANA_USRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RXADC_ANA_USR to value 0x1b
impl crate::Resettable for RXADC_ANA_USRrs {
    const RESET_VALUE: u32 = 0x1b;
}
