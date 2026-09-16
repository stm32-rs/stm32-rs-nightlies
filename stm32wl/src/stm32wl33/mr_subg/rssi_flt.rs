///Register `RSSI_FLT` reader
pub type R = crate::R<RSSI_FLTrs>;
///Register `RSSI_FLT` writer
pub type W = crate::W<RSSI_FLTrs>;
///Field `OOK_PEAK_DECAY` reader - Peak decay control for OOK: 3 slow decay; 0 fast decay
pub type OOK_PEAK_DECAY_R = crate::FieldReader;
///Field `OOK_PEAK_DECAY` writer - Peak decay control for OOK: 3 slow decay; 0 fast decay
pub type OOK_PEAK_DECAY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RSSI_FLT` reader - Gain of the RSSI filter
pub type RSSI_FLT_R = crate::FieldReader;
///Field `RSSI_FLT` writer - Gain of the RSSI filter
pub type RSSI_FLT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Peak decay control for OOK: 3 slow decay; 0 fast decay
    #[inline(always)]
    pub fn ook_peak_decay(&self) -> OOK_PEAK_DECAY_R {
        OOK_PEAK_DECAY_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Gain of the RSSI filter
    #[inline(always)]
    pub fn rssi_flt(&self) -> RSSI_FLT_R {
        RSSI_FLT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSSI_FLT")
            .field("ook_peak_decay", &self.ook_peak_decay())
            .field("rssi_flt", &self.rssi_flt())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Peak decay control for OOK: 3 slow decay; 0 fast decay
    #[inline(always)]
    pub fn ook_peak_decay(&mut self) -> OOK_PEAK_DECAY_W<'_, RSSI_FLTrs> {
        OOK_PEAK_DECAY_W::new(self, 0)
    }
    ///Bits 4:7 - Gain of the RSSI filter
    #[inline(always)]
    pub fn rssi_flt(&mut self) -> RSSI_FLT_W<'_, RSSI_FLTrs> {
        RSSI_FLT_W::new(self, 4)
    }
}
/**RSSI_FLT register

You can [`read`](crate::Reg::read) this register and get [`rssi_flt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rssi_flt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:RSSI_FLT)*/
pub struct RSSI_FLTrs;
impl crate::RegisterSpec for RSSI_FLTrs {
    type Ux = u32;
}
///`read()` method returns [`rssi_flt::R`](R) reader structure
impl crate::Readable for RSSI_FLTrs {}
///`write(|w| ..)` method takes [`rssi_flt::W`](W) writer structure
impl crate::Writable for RSSI_FLTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RSSI_FLT to value 0xe0
impl crate::Resettable for RSSI_FLTrs {
    const RESET_VALUE: u32 = 0xe0;
}
