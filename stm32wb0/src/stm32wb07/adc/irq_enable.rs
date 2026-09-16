///Register `IRQ_ENABLE` reader
pub type R = crate::R<IRQ_ENABLErs>;
///Register `IRQ_ENABLE` writer
pub type W = crate::W<IRQ_ENABLErs>;
///Field `EOC_IRQ_ENA` reader - (Used in test mode only): End of ADC conversion interrupt enable
pub type EOC_IRQ_ENA_R = crate::BitReader;
///Field `EOC_IRQ_ENA` writer - (Used in test mode only): End of ADC conversion interrupt enable
pub type EOC_IRQ_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EODS_IRQ_ENA` reader - End of conversion interrupt enable for the Down Sampler output
pub type EODS_IRQ_ENA_R = crate::BitReader;
///Field `EODS_IRQ_ENA` writer - End of conversion interrupt enable for the Down Sampler output
pub type EODS_IRQ_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EODF_IRQ_ENA` reader - End of conversion interrupt enable for the decimation filter output
pub type EODF_IRQ_ENA_R = crate::BitReader;
///Field `EODF_IRQ_ENA` writer - End of conversion interrupt enable for the decimation filter output
pub type EODF_IRQ_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOS_IRQ_ENA` reader - End of regular sequence interrupt enable
pub type EOS_IRQ_ENA_R = crate::BitReader;
///Field `EOS_IRQ_ENA` writer - End of regular sequence interrupt enable
pub type EOS_IRQ_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWD_IRQ_ENA` reader - analog watchdog interrupt enable
pub type AWD_IRQ_ENA_R = crate::BitReader;
///Field `AWD_IRQ_ENA` writer - analog watchdog interrupt enable
pub type AWD_IRQ_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVR_DS_IRQ_ENA` reader - Down Sampler overrun interrupt enable
pub type OVR_DS_IRQ_ENA_R = crate::BitReader;
///Field `OVR_DS_IRQ_ENA` writer - Down Sampler overrun interrupt enable
pub type OVR_DS_IRQ_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVR_DF_IRQ_ENA` reader - decimation filter overrun interrupt enable
pub type OVR_DF_IRQ_ENA_R = crate::BitReader;
///Field `OVR_DF_IRQ_ENA` writer - decimation filter overrun interrupt enable
pub type OVR_DF_IRQ_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DF_OVRFL_IRQ_ENA` reader - decimation filter saturation interrupt enable
pub type DF_OVRFL_IRQ_ENA_R = crate::BitReader;
///Field `DF_OVRFL_IRQ_ENA` writer - decimation filter saturation interrupt enable
pub type DF_OVRFL_IRQ_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - (Used in test mode only): End of ADC conversion interrupt enable
    #[inline(always)]
    pub fn eoc_irq_ena(&self) -> EOC_IRQ_ENA_R {
        EOC_IRQ_ENA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of conversion interrupt enable for the Down Sampler output
    #[inline(always)]
    pub fn eods_irq_ena(&self) -> EODS_IRQ_ENA_R {
        EODS_IRQ_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - End of conversion interrupt enable for the decimation filter output
    #[inline(always)]
    pub fn eodf_irq_ena(&self) -> EODF_IRQ_ENA_R {
        EODF_IRQ_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - End of regular sequence interrupt enable
    #[inline(always)]
    pub fn eos_irq_ena(&self) -> EOS_IRQ_ENA_R {
        EOS_IRQ_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - analog watchdog interrupt enable
    #[inline(always)]
    pub fn awd_irq_ena(&self) -> AWD_IRQ_ENA_R {
        AWD_IRQ_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Down Sampler overrun interrupt enable
    #[inline(always)]
    pub fn ovr_ds_irq_ena(&self) -> OVR_DS_IRQ_ENA_R {
        OVR_DS_IRQ_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - decimation filter overrun interrupt enable
    #[inline(always)]
    pub fn ovr_df_irq_ena(&self) -> OVR_DF_IRQ_ENA_R {
        OVR_DF_IRQ_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - decimation filter saturation interrupt enable
    #[inline(always)]
    pub fn df_ovrfl_irq_ena(&self) -> DF_OVRFL_IRQ_ENA_R {
        DF_OVRFL_IRQ_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQ_ENABLE")
            .field("df_ovrfl_irq_ena", &self.df_ovrfl_irq_ena())
            .field("ovr_df_irq_ena", &self.ovr_df_irq_ena())
            .field("ovr_ds_irq_ena", &self.ovr_ds_irq_ena())
            .field("awd_irq_ena", &self.awd_irq_ena())
            .field("eos_irq_ena", &self.eos_irq_ena())
            .field("eodf_irq_ena", &self.eodf_irq_ena())
            .field("eods_irq_ena", &self.eods_irq_ena())
            .field("eoc_irq_ena", &self.eoc_irq_ena())
            .finish()
    }
}
impl W {
    ///Bit 0 - (Used in test mode only): End of ADC conversion interrupt enable
    #[inline(always)]
    pub fn eoc_irq_ena(&mut self) -> EOC_IRQ_ENA_W<'_, IRQ_ENABLErs> {
        EOC_IRQ_ENA_W::new(self, 0)
    }
    ///Bit 1 - End of conversion interrupt enable for the Down Sampler output
    #[inline(always)]
    pub fn eods_irq_ena(&mut self) -> EODS_IRQ_ENA_W<'_, IRQ_ENABLErs> {
        EODS_IRQ_ENA_W::new(self, 1)
    }
    ///Bit 2 - End of conversion interrupt enable for the decimation filter output
    #[inline(always)]
    pub fn eodf_irq_ena(&mut self) -> EODF_IRQ_ENA_W<'_, IRQ_ENABLErs> {
        EODF_IRQ_ENA_W::new(self, 2)
    }
    ///Bit 3 - End of regular sequence interrupt enable
    #[inline(always)]
    pub fn eos_irq_ena(&mut self) -> EOS_IRQ_ENA_W<'_, IRQ_ENABLErs> {
        EOS_IRQ_ENA_W::new(self, 3)
    }
    ///Bit 4 - analog watchdog interrupt enable
    #[inline(always)]
    pub fn awd_irq_ena(&mut self) -> AWD_IRQ_ENA_W<'_, IRQ_ENABLErs> {
        AWD_IRQ_ENA_W::new(self, 4)
    }
    ///Bit 5 - Down Sampler overrun interrupt enable
    #[inline(always)]
    pub fn ovr_ds_irq_ena(&mut self) -> OVR_DS_IRQ_ENA_W<'_, IRQ_ENABLErs> {
        OVR_DS_IRQ_ENA_W::new(self, 5)
    }
    ///Bit 6 - decimation filter overrun interrupt enable
    #[inline(always)]
    pub fn ovr_df_irq_ena(&mut self) -> OVR_DF_IRQ_ENA_W<'_, IRQ_ENABLErs> {
        OVR_DF_IRQ_ENA_W::new(self, 6)
    }
    ///Bit 7 - decimation filter saturation interrupt enable
    #[inline(always)]
    pub fn df_ovrfl_irq_ena(&mut self) -> DF_OVRFL_IRQ_ENA_W<'_, IRQ_ENABLErs> {
        DF_OVRFL_IRQ_ENA_W::new(self, 7)
    }
}
/**Enable/disable Interrupts

You can [`read`](crate::Reg::read) this register and get [`irq_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#ADC:IRQ_ENABLE)*/
pub struct IRQ_ENABLErs;
impl crate::RegisterSpec for IRQ_ENABLErs {
    type Ux = u32;
}
///`read()` method returns [`irq_enable::R`](R) reader structure
impl crate::Readable for IRQ_ENABLErs {}
///`write(|w| ..)` method takes [`irq_enable::W`](W) writer structure
impl crate::Writable for IRQ_ENABLErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IRQ_ENABLE to value 0
impl crate::Resettable for IRQ_ENABLErs {}
