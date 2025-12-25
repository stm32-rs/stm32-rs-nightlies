///Register `BCLRFR` writer
pub type W = crate::W<BCLRFRrs>;
///Field `COVRUDR` writer - Clear overrun / underrun. This bit is write only. Programming this bit to 1 clears the OVRUDR flag in the SAI_xSR register. Reading this bit always returns the value 0.
pub type COVRUDR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMUTEDET` writer - Mute detection flag. This bit is write only. Programming this bit to 1 clears the MUTEDET flag in the SAI_xSR register. Reading this bit always returns the value 0.
pub type CMUTEDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWCKCFG` writer - Clear wrong clock configuration flag. This bit is write only. Programming this bit to 1 clears the WCKCFG flag in the SAI_xSR register. This bit is used only when the audio block is set as master (MODE\[1\] = 0) and NODIV = 0 in the SAI_xCR1 register. Reading this bit always returns the value 0.
pub type CWCKCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCNRDY` writer - Clear Codec not ready flag. This bit is write only. Programming this bit to 1 clears the CNRDY flag in the SAI_xSR register. This bit is used only when the AC97 audio protocol is selected in the SAI_xCR1 register. Reading this bit always returns the value 0.
pub type CCNRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAFSDET` writer - Clear anticipated frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the AFSDET flag in the SAI_xSR register. It is not used in AC97or SPDIF mode. Reading this bit always returns the value 0.
pub type CAFSDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLFSDET` writer - Clear late frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the LFSDET flag in the SAI_xSR register. This bit is not used in AC97or SPDIF mode Reading this bit always returns the value 0.
pub type CLFSDET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<BCLRFRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear overrun / underrun. This bit is write only. Programming this bit to 1 clears the OVRUDR flag in the SAI_xSR register. Reading this bit always returns the value 0.
    #[inline(always)]
    pub fn covrudr(&mut self) -> COVRUDR_W<'_, BCLRFRrs> {
        COVRUDR_W::new(self, 0)
    }
    ///Bit 1 - Mute detection flag. This bit is write only. Programming this bit to 1 clears the MUTEDET flag in the SAI_xSR register. Reading this bit always returns the value 0.
    #[inline(always)]
    pub fn cmutedet(&mut self) -> CMUTEDET_W<'_, BCLRFRrs> {
        CMUTEDET_W::new(self, 1)
    }
    ///Bit 2 - Clear wrong clock configuration flag. This bit is write only. Programming this bit to 1 clears the WCKCFG flag in the SAI_xSR register. This bit is used only when the audio block is set as master (MODE\[1\] = 0) and NODIV = 0 in the SAI_xCR1 register. Reading this bit always returns the value 0.
    #[inline(always)]
    pub fn cwckcfg(&mut self) -> CWCKCFG_W<'_, BCLRFRrs> {
        CWCKCFG_W::new(self, 2)
    }
    ///Bit 4 - Clear Codec not ready flag. This bit is write only. Programming this bit to 1 clears the CNRDY flag in the SAI_xSR register. This bit is used only when the AC97 audio protocol is selected in the SAI_xCR1 register. Reading this bit always returns the value 0.
    #[inline(always)]
    pub fn ccnrdy(&mut self) -> CCNRDY_W<'_, BCLRFRrs> {
        CCNRDY_W::new(self, 4)
    }
    ///Bit 5 - Clear anticipated frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the AFSDET flag in the SAI_xSR register. It is not used in AC97or SPDIF mode. Reading this bit always returns the value 0.
    #[inline(always)]
    pub fn cafsdet(&mut self) -> CAFSDET_W<'_, BCLRFRrs> {
        CAFSDET_W::new(self, 5)
    }
    ///Bit 6 - Clear late frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the LFSDET flag in the SAI_xSR register. This bit is not used in AC97or SPDIF mode Reading this bit always returns the value 0.
    #[inline(always)]
    pub fn clfsdet(&mut self) -> CLFSDET_W<'_, BCLRFRrs> {
        CLFSDET_W::new(self, 6)
    }
}
/**SAI clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bclrfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#SAI1:BCLRFR)*/
pub struct BCLRFRrs;
impl crate::RegisterSpec for BCLRFRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`bclrfr::W`](W) writer structure
impl crate::Writable for BCLRFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCLRFR to value 0
impl crate::Resettable for BCLRFRrs {}
