///Register `DTB_CONF` reader
pub type R = crate::R<DTB_CONFrs>;
///Register `DTB_CONF` writer
pub type W = crate::W<DTB_CONFrs>;
///Field `ADC_DBG_CONF` reader - ADC_DBG_CONF\[3:0\]: use for debug purpose.
pub type ADC_DBG_CONF_R = crate::FieldReader;
///Field `ADC_DBG_CONF` writer - ADC_DBG_CONF\[3:0\]: use for debug purpose.
pub type ADC_DBG_CONF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ADC_DTB_CONF` reader - ADC_DTB_CONF\[1:0\]: configure the DTB output. 00: DTB bus is all 0 01: output the ADC_BUSY, ADC_EOC, offset compensation data\[11:0\] on the ADC_DTB 10: output the DS information on the ADC_DTB 11: select states of the FSM and enable ADC serial output Note: detailed DTB configurations are available in the Table 38 in IUM
pub type ADC_DTB_CONF_R = crate::FieldReader;
///Field `ADC_DTB_CONF` writer - ADC_DTB_CONF\[1:0\]: configure the DTB output. 00: DTB bus is all 0 01: output the ADC_BUSY, ADC_EOC, offset compensation data\[11:0\] on the ADC_DTB 10: output the DS information on the ADC_DTB 11: select states of the FSM and enable ADC serial output Note: detailed DTB configurations are available in the Table 38 in IUM
pub type ADC_DTB_CONF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DTB_SER_SEL` reader - DTB_SER_SEL: DTB serial output selection when ADC_DB_CONF\[1:0\]=3d 0: pre down-sampler with offset compensation data 1: post down-sampler data
pub type DTB_SER_SEL_R = crate::BitReader;
///Field `DTB_SER_SEL` writer - DTB_SER_SEL: DTB serial output selection when ADC_DB_CONF\[1:0\]=3d 0: pre down-sampler with offset compensation data 1: post down-sampler data
pub type DTB_SER_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSM_STATE` reader - FSM_STATE\[7:0\]: show the state of the state machine. Bit 0: IDLE Bit 1: Reserved Bit 2: ADC setup phase Bit 3: Reserved Bit 4: ADC_START_CONV resynchronization Bit 5: Reserved Bit 6: ADC mode Bit 7: sequence mode
pub type FSM_STATE_R = crate::FieldReader;
///Field `FSM_CUR_STATE` reader - FSM_CUR_STATE\[2:0\]: show the last executed state by the state machine. 000: IDLE mode 001: Reserved 010: ADC setup phase 011: Reserved 100: ADC_START_CONV resynchronization 101: Reserved 110: ADC mode 111: sequence mode
pub type FSM_CUR_STATE_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - ADC_DBG_CONF\[3:0\]: use for debug purpose.
    #[inline(always)]
    pub fn adc_dbg_conf(&self) -> ADC_DBG_CONF_R {
        ADC_DBG_CONF_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:9 - ADC_DTB_CONF\[1:0\]: configure the DTB output. 00: DTB bus is all 0 01: output the ADC_BUSY, ADC_EOC, offset compensation data\[11:0\] on the ADC_DTB 10: output the DS information on the ADC_DTB 11: select states of the FSM and enable ADC serial output Note: detailed DTB configurations are available in the Table 38 in IUM
    #[inline(always)]
    pub fn adc_dtb_conf(&self) -> ADC_DTB_CONF_R {
        ADC_DTB_CONF_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - DTB_SER_SEL: DTB serial output selection when ADC_DB_CONF\[1:0\]=3d 0: pre down-sampler with offset compensation data 1: post down-sampler data
    #[inline(always)]
    pub fn dtb_ser_sel(&self) -> DTB_SER_SEL_R {
        DTB_SER_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 16:23 - FSM_STATE\[7:0\]: show the state of the state machine. Bit 0: IDLE Bit 1: Reserved Bit 2: ADC setup phase Bit 3: Reserved Bit 4: ADC_START_CONV resynchronization Bit 5: Reserved Bit 6: ADC mode Bit 7: sequence mode
    #[inline(always)]
    pub fn fsm_state(&self) -> FSM_STATE_R {
        FSM_STATE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:26 - FSM_CUR_STATE\[2:0\]: show the last executed state by the state machine. 000: IDLE mode 001: Reserved 010: ADC setup phase 011: Reserved 100: ADC_START_CONV resynchronization 101: Reserved 110: ADC mode 111: sequence mode
    #[inline(always)]
    pub fn fsm_cur_state(&self) -> FSM_CUR_STATE_R {
        FSM_CUR_STATE_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTB_CONF")
            .field("adc_dbg_conf", &self.adc_dbg_conf())
            .field("adc_dtb_conf", &self.adc_dtb_conf())
            .field("dtb_ser_sel", &self.dtb_ser_sel())
            .field("fsm_state", &self.fsm_state())
            .field("fsm_cur_state", &self.fsm_cur_state())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - ADC_DBG_CONF\[3:0\]: use for debug purpose.
    #[inline(always)]
    pub fn adc_dbg_conf(&mut self) -> ADC_DBG_CONF_W<'_, DTB_CONFrs> {
        ADC_DBG_CONF_W::new(self, 0)
    }
    ///Bits 8:9 - ADC_DTB_CONF\[1:0\]: configure the DTB output. 00: DTB bus is all 0 01: output the ADC_BUSY, ADC_EOC, offset compensation data\[11:0\] on the ADC_DTB 10: output the DS information on the ADC_DTB 11: select states of the FSM and enable ADC serial output Note: detailed DTB configurations are available in the Table 38 in IUM
    #[inline(always)]
    pub fn adc_dtb_conf(&mut self) -> ADC_DTB_CONF_W<'_, DTB_CONFrs> {
        ADC_DTB_CONF_W::new(self, 8)
    }
    ///Bit 10 - DTB_SER_SEL: DTB serial output selection when ADC_DB_CONF\[1:0\]=3d 0: pre down-sampler with offset compensation data 1: post down-sampler data
    #[inline(always)]
    pub fn dtb_ser_sel(&mut self) -> DTB_SER_SEL_W<'_, DTB_CONFrs> {
        DTB_SER_SEL_W::new(self, 10)
    }
}
/**DTB_CONF register

You can [`read`](crate::Reg::read) this register and get [`dtb_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtb_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#ADC:DTB_CONF)*/
pub struct DTB_CONFrs;
impl crate::RegisterSpec for DTB_CONFrs {
    type Ux = u32;
}
///`read()` method returns [`dtb_conf::R`](R) reader structure
impl crate::Readable for DTB_CONFrs {}
///`write(|w| ..)` method takes [`dtb_conf::W`](W) writer structure
impl crate::Writable for DTB_CONFrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DTB_CONF to value 0
impl crate::Resettable for DTB_CONFrs {}
