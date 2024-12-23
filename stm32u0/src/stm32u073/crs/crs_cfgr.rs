///Register `CRS_CFGR` reader
pub type R = crate::R<CRS_CFGRrs>;
///Register `CRS_CFGR` writer
pub type W = crate::W<CRS_CFGRrs>;
///Field `RELOAD` reader - Counter reload value RELOAD is the value to be loaded in the frequency error counter with each SYNC event. Refer to Section15.4.3 for more details about counter behavior.
pub type RELOAD_R = crate::FieldReader<u16>;
///Field `RELOAD` writer - Counter reload value RELOAD is the value to be loaded in the frequency error counter with each SYNC event. Refer to Section15.4.3 for more details about counter behavior.
pub type RELOAD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
/**Field `FELIM` reader - Frequency error limit FELIM contains the value to be used to evaluate the captured frequency error value latched in the FECAP\[15:0\]
bits of the CRS_ISR register. Refer to Section15.4.4 for more details about FECAP evaluation.*/
pub type FELIM_R = crate::FieldReader;
/**Field `FELIM` writer - Frequency error limit FELIM contains the value to be used to evaluate the captured frequency error value latched in the FECAP\[15:0\]
bits of the CRS_ISR register. Refer to Section15.4.4 for more details about FECAP evaluation.*/
pub type FELIM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SYNCDIV` reader - SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal.
pub type SYNCDIV_R = crate::FieldReader;
///Field `SYNCDIV` writer - SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal.
pub type SYNCDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SYNCSRC` reader - SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source (see Table122): Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF is not generated by the host. No SYNC signal is therefore provided to the CRS to calibrate the HSI48 oscillator on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs must be used as SYNC signal.
pub type SYNCSRC_R = crate::FieldReader;
///Field `SYNCSRC` writer - SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source (see Table122): Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF is not generated by the host. No SYNC signal is therefore provided to the CRS to calibrate the HSI48 oscillator on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs must be used as SYNC signal.
pub type SYNCSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SYNCPOL` reader - SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source.
pub type SYNCPOL_R = crate::BitReader;
///Field `SYNCPOL` writer - SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source.
pub type SYNCPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - Counter reload value RELOAD is the value to be loaded in the frequency error counter with each SYNC event. Refer to Section15.4.3 for more details about counter behavior.
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new((self.bits & 0xffff) as u16)
    }
    /**Bits 16:23 - Frequency error limit FELIM contains the value to be used to evaluate the captured frequency error value latched in the FECAP\[15:0\]
    bits of the CRS_ISR register. Refer to Section15.4.4 for more details about FECAP evaluation.*/
    #[inline(always)]
    pub fn felim(&self) -> FELIM_R {
        FELIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:26 - SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal.
    #[inline(always)]
    pub fn syncdiv(&self) -> SYNCDIV_R {
        SYNCDIV_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 28:29 - SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source (see Table122): Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF is not generated by the host. No SYNC signal is therefore provided to the CRS to calibrate the HSI48 oscillator on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs must be used as SYNC signal.
    #[inline(always)]
    pub fn syncsrc(&self) -> SYNCSRC_R {
        SYNCSRC_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 31 - SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source.
    #[inline(always)]
    pub fn syncpol(&self) -> SYNCPOL_R {
        SYNCPOL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRS_CFGR")
            .field("reload", &self.reload())
            .field("felim", &self.felim())
            .field("syncdiv", &self.syncdiv())
            .field("syncsrc", &self.syncsrc())
            .field("syncpol", &self.syncpol())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Counter reload value RELOAD is the value to be loaded in the frequency error counter with each SYNC event. Refer to Section15.4.3 for more details about counter behavior.
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W<CRS_CFGRrs> {
        RELOAD_W::new(self, 0)
    }
    /**Bits 16:23 - Frequency error limit FELIM contains the value to be used to evaluate the captured frequency error value latched in the FECAP\[15:0\]
    bits of the CRS_ISR register. Refer to Section15.4.4 for more details about FECAP evaluation.*/
    #[inline(always)]
    pub fn felim(&mut self) -> FELIM_W<CRS_CFGRrs> {
        FELIM_W::new(self, 16)
    }
    ///Bits 24:26 - SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal.
    #[inline(always)]
    pub fn syncdiv(&mut self) -> SYNCDIV_W<CRS_CFGRrs> {
        SYNCDIV_W::new(self, 24)
    }
    ///Bits 28:29 - SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source (see Table122): Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF is not generated by the host. No SYNC signal is therefore provided to the CRS to calibrate the HSI48 oscillator on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs must be used as SYNC signal.
    #[inline(always)]
    pub fn syncsrc(&mut self) -> SYNCSRC_W<CRS_CFGRrs> {
        SYNCSRC_W::new(self, 28)
    }
    ///Bit 31 - SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source.
    #[inline(always)]
    pub fn syncpol(&mut self) -> SYNCPOL_W<CRS_CFGRrs> {
        SYNCPOL_W::new(self, 31)
    }
}
/**CRS configuration register

You can [`read`](crate::Reg::read) this register and get [`crs_cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crs_cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#CRS:CRS_CFGR)*/
pub struct CRS_CFGRrs;
impl crate::RegisterSpec for CRS_CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`crs_cfgr::R`](R) reader structure
impl crate::Readable for CRS_CFGRrs {}
///`write(|w| ..)` method takes [`crs_cfgr::W`](W) writer structure
impl crate::Writable for CRS_CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CRS_CFGR to value 0x2022_bb7f
impl crate::Resettable for CRS_CFGRrs {
    const RESET_VALUE: u32 = 0x2022_bb7f;
}
