///Register `IRQ_STATUS` reader
pub type R = crate::R<IRQ_STATUSrs>;
///Register `IRQ_STATUS` writer
pub type W = crate::W<IRQ_STATUSrs>;
///Field `EOC_IRQ` reader - (Used in test mode only): set when the ADC conversion is completed.
pub type EOC_IRQ_R = crate::BitReader;
///Field `EOC_IRQ` writer - (Used in test mode only): set when the ADC conversion is completed.
pub type EOC_IRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EODS_IRQ` reader - set when the Down Sampler conversion is completed.
pub type EODS_IRQ_R = crate::BitReader;
///Field `EODS_IRQ` writer - set when the Down Sampler conversion is completed.
pub type EODS_IRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EODF_IRQ` reader - set when the decimation filter conversion is completed
pub type EODF_IRQ_R = crate::BitReader;
///Field `EODF_IRQ` writer - set when the decimation filter conversion is completed
pub type EODF_IRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOS_IRQ` reader - set when a sequence of conversion is completed
pub type EOS_IRQ_R = crate::BitReader;
///Field `EOS_IRQ` writer - set when a sequence of conversion is completed
pub type EOS_IRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD_IRQ` reader - set when an analog watchdog event occurs
pub type AWD_IRQ_R = crate::BitReader;
///Field `AWD_IRQ` writer - set when an analog watchdog event occurs
pub type AWD_IRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVR_DS_IRQ` reader - set to indicate a Down Sampler overrun (at least one data is lost)
pub type OVR_DS_IRQ_R = crate::BitReader;
///Field `OVR_DS_IRQ` writer - set to indicate a Down Sampler overrun (at least one data is lost)
pub type OVR_DS_IRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVR_DF_IRQ` reader - set to indicate a decimation filter overrun (a data is lost)
pub type OVR_DF_IRQ_R = crate::BitReader;
///Field `OVR_DF_IRQ` writer - set to indicate a decimation filter overrun (a data is lost)
pub type OVR_DF_IRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DF_OVRFL_IRQ` reader - set to indicate the decimation filter is saturated.
pub type DF_OVRFL_IRQ_R = crate::BitReader;
///Field `DF_OVRFL_IRQ` writer - set to indicate the decimation filter is saturated.
pub type DF_OVRFL_IRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - (Used in test mode only): set when the ADC conversion is completed.
    #[inline(always)]
    pub fn eoc_irq(&self) -> EOC_IRQ_R {
        EOC_IRQ_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - set when the Down Sampler conversion is completed.
    #[inline(always)]
    pub fn eods_irq(&self) -> EODS_IRQ_R {
        EODS_IRQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - set when the decimation filter conversion is completed
    #[inline(always)]
    pub fn eodf_irq(&self) -> EODF_IRQ_R {
        EODF_IRQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - set when a sequence of conversion is completed
    #[inline(always)]
    pub fn eos_irq(&self) -> EOS_IRQ_R {
        EOS_IRQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - set when an analog watchdog event occurs
    #[inline(always)]
    pub fn awd_irq(&self) -> AWD_IRQ_R {
        AWD_IRQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - set to indicate a Down Sampler overrun (at least one data is lost)
    #[inline(always)]
    pub fn ovr_ds_irq(&self) -> OVR_DS_IRQ_R {
        OVR_DS_IRQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - set to indicate a decimation filter overrun (a data is lost)
    #[inline(always)]
    pub fn ovr_df_irq(&self) -> OVR_DF_IRQ_R {
        OVR_DF_IRQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - set to indicate the decimation filter is saturated.
    #[inline(always)]
    pub fn df_ovrfl_irq(&self) -> DF_OVRFL_IRQ_R {
        DF_OVRFL_IRQ_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQ_STATUS")
            .field("df_ovrfl_irq", &self.df_ovrfl_irq())
            .field("ovr_df_irq", &self.ovr_df_irq())
            .field("ovr_ds_irq", &self.ovr_ds_irq())
            .field("awd_irq", &self.awd_irq())
            .field("eos_irq", &self.eos_irq())
            .field("eodf_irq", &self.eodf_irq())
            .field("eods_irq", &self.eods_irq())
            .field("eoc_irq", &self.eoc_irq())
            .finish()
    }
}
impl W {
    ///Bit 0 - (Used in test mode only): set when the ADC conversion is completed.
    #[inline(always)]
    pub fn eoc_irq(&mut self) -> EOC_IRQ_W<'_, IRQ_STATUSrs> {
        EOC_IRQ_W::new(self, 0)
    }
    ///Bit 1 - set when the Down Sampler conversion is completed.
    #[inline(always)]
    pub fn eods_irq(&mut self) -> EODS_IRQ_W<'_, IRQ_STATUSrs> {
        EODS_IRQ_W::new(self, 1)
    }
    ///Bit 2 - set when the decimation filter conversion is completed
    #[inline(always)]
    pub fn eodf_irq(&mut self) -> EODF_IRQ_W<'_, IRQ_STATUSrs> {
        EODF_IRQ_W::new(self, 2)
    }
    ///Bit 3 - set when a sequence of conversion is completed
    #[inline(always)]
    pub fn eos_irq(&mut self) -> EOS_IRQ_W<'_, IRQ_STATUSrs> {
        EOS_IRQ_W::new(self, 3)
    }
    ///Bit 4 - set when an analog watchdog event occurs
    #[inline(always)]
    pub fn awd_irq(&mut self) -> AWD_IRQ_W<'_, IRQ_STATUSrs> {
        AWD_IRQ_W::new(self, 4)
    }
    ///Bit 5 - set to indicate a Down Sampler overrun (at least one data is lost)
    #[inline(always)]
    pub fn ovr_ds_irq(&mut self) -> OVR_DS_IRQ_W<'_, IRQ_STATUSrs> {
        OVR_DS_IRQ_W::new(self, 5)
    }
    ///Bit 6 - set to indicate a decimation filter overrun (a data is lost)
    #[inline(always)]
    pub fn ovr_df_irq(&mut self) -> OVR_DF_IRQ_W<'_, IRQ_STATUSrs> {
        OVR_DF_IRQ_W::new(self, 6)
    }
    ///Bit 7 - set to indicate the decimation filter is saturated.
    #[inline(always)]
    pub fn df_ovrfl_irq(&mut self) -> DF_OVRFL_IRQ_W<'_, IRQ_STATUSrs> {
        DF_OVRFL_IRQ_W::new(self, 7)
    }
}
/**Interrupt Status register

You can [`read`](crate::Reg::read) this register and get [`irq_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#ADC:IRQ_STATUS)*/
pub struct IRQ_STATUSrs;
impl crate::RegisterSpec for IRQ_STATUSrs {
    type Ux = u32;
}
///`read()` method returns [`irq_status::R`](R) reader structure
impl crate::Readable for IRQ_STATUSrs {}
///`write(|w| ..)` method takes [`irq_status::W`](W) writer structure
impl crate::Writable for IRQ_STATUSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IRQ_STATUS to value 0
impl crate::Resettable for IRQ_STATUSrs {}
