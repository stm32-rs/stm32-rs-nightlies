///Register `BCLRFR` writer
pub type W = crate::W<BCLRFRrs>;
/**Clear overrun / underrun. This bit is write only. Programming this bit to 1 clears the OVRUDR flag in the SAI_xSR register. Reading this bit always returns the value 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COVRUDRW {
    ///1: Clears the OVRUDR flag
    Clear = 1,
}
impl From<COVRUDRW> for bool {
    #[inline(always)]
    fn from(variant: COVRUDRW) -> Self {
        variant as u8 != 0
    }
}
///Field `COVRUDR` writer - Clear overrun / underrun. This bit is write only. Programming this bit to 1 clears the OVRUDR flag in the SAI_xSR register. Reading this bit always returns the value 0.
pub type COVRUDR_W<'a, REG> = crate::BitWriter<'a, REG, COVRUDRW>;
impl<'a, REG> COVRUDR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the OVRUDR flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(COVRUDRW::Clear)
    }
}
/**Mute detection flag. This bit is write only. Programming this bit to 1 clears the MUTEDET flag in the SAI_xSR register. Reading this bit always returns the value 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMUTEDETW {
    ///1: Clears the MUTEDET flag
    Clear = 1,
}
impl From<CMUTEDETW> for bool {
    #[inline(always)]
    fn from(variant: CMUTEDETW) -> Self {
        variant as u8 != 0
    }
}
///Field `CMUTEDET` writer - Mute detection flag. This bit is write only. Programming this bit to 1 clears the MUTEDET flag in the SAI_xSR register. Reading this bit always returns the value 0.
pub type CMUTEDET_W<'a, REG> = crate::BitWriter<'a, REG, CMUTEDETW>;
impl<'a, REG> CMUTEDET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the MUTEDET flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CMUTEDETW::Clear)
    }
}
/**Clear wrong clock configuration flag. This bit is write only. Programming this bit to 1 clears the WCKCFG flag in the SAI_xSR register. This bit is used only when the audio block is set as master (MODE\[1\] = 0) and NODIV = 0 in the SAI_xCR1 register. Reading this bit always returns the value 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWCKCFGW {
    ///1: Clears the WCKCFG flag
    Clear = 1,
}
impl From<CWCKCFGW> for bool {
    #[inline(always)]
    fn from(variant: CWCKCFGW) -> Self {
        variant as u8 != 0
    }
}
///Field `CWCKCFG` writer - Clear wrong clock configuration flag. This bit is write only. Programming this bit to 1 clears the WCKCFG flag in the SAI_xSR register. This bit is used only when the audio block is set as master (MODE\[1\] = 0) and NODIV = 0 in the SAI_xCR1 register. Reading this bit always returns the value 0.
pub type CWCKCFG_W<'a, REG> = crate::BitWriter<'a, REG, CWCKCFGW>;
impl<'a, REG> CWCKCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the WCKCFG flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CWCKCFGW::Clear)
    }
}
/**Clear Codec not ready flag. This bit is write only. Programming this bit to 1 clears the CNRDY flag in the SAI_xSR register. This bit is used only when the AC'97 audio protocol is selected in the SAI_xCR1 register. Reading this bit always returns the value 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCNRDYW {
    ///1: Clears the CNRDY flag
    Clear = 1,
}
impl From<CCNRDYW> for bool {
    #[inline(always)]
    fn from(variant: CCNRDYW) -> Self {
        variant as u8 != 0
    }
}
///Field `CCNRDY` writer - Clear Codec not ready flag. This bit is write only. Programming this bit to 1 clears the CNRDY flag in the SAI_xSR register. This bit is used only when the AC'97 audio protocol is selected in the SAI_xCR1 register. Reading this bit always returns the value 0.
pub type CCNRDY_W<'a, REG> = crate::BitWriter<'a, REG, CCNRDYW>;
impl<'a, REG> CCNRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the CNRDY flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CCNRDYW::Clear)
    }
}
/**Clear anticipated frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the AFSDET flag in the SAI_xSR register. It is not used in AC'97or SPDIF mode. Reading this bit always returns the value 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAFSDETW {
    ///1: Clears the AFSDET flag
    Clear = 1,
}
impl From<CAFSDETW> for bool {
    #[inline(always)]
    fn from(variant: CAFSDETW) -> Self {
        variant as u8 != 0
    }
}
///Field `CAFSDET` writer - Clear anticipated frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the AFSDET flag in the SAI_xSR register. It is not used in AC'97or SPDIF mode. Reading this bit always returns the value 0.
pub type CAFSDET_W<'a, REG> = crate::BitWriter<'a, REG, CAFSDETW>;
impl<'a, REG> CAFSDET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the AFSDET flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CAFSDETW::Clear)
    }
}
/**Clear late frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the LFSDET flag in the SAI_xSR register. This bit is not used in AC'97or SPDIF mode Reading this bit always returns the value 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLFSDETW {
    ///1: Clears the LFSDET flag
    Clear = 1,
}
impl From<CLFSDETW> for bool {
    #[inline(always)]
    fn from(variant: CLFSDETW) -> Self {
        variant as u8 != 0
    }
}
///Field `CLFSDET` writer - Clear late frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the LFSDET flag in the SAI_xSR register. This bit is not used in AC'97or SPDIF mode Reading this bit always returns the value 0.
pub type CLFSDET_W<'a, REG> = crate::BitWriter<'a, REG, CLFSDETW>;
impl<'a, REG> CLFSDET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the LFSDET flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CLFSDETW::Clear)
    }
}
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
    ///Bit 4 - Clear Codec not ready flag. This bit is write only. Programming this bit to 1 clears the CNRDY flag in the SAI_xSR register. This bit is used only when the AC'97 audio protocol is selected in the SAI_xCR1 register. Reading this bit always returns the value 0.
    #[inline(always)]
    pub fn ccnrdy(&mut self) -> CCNRDY_W<'_, BCLRFRrs> {
        CCNRDY_W::new(self, 4)
    }
    ///Bit 5 - Clear anticipated frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the AFSDET flag in the SAI_xSR register. It is not used in AC'97or SPDIF mode. Reading this bit always returns the value 0.
    #[inline(always)]
    pub fn cafsdet(&mut self) -> CAFSDET_W<'_, BCLRFRrs> {
        CAFSDET_W::new(self, 5)
    }
    ///Bit 6 - Clear late frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the LFSDET flag in the SAI_xSR register. This bit is not used in AC'97or SPDIF mode Reading this bit always returns the value 0.
    #[inline(always)]
    pub fn clfsdet(&mut self) -> CLFSDET_W<'_, BCLRFRrs> {
        CLFSDET_W::new(self, 6)
    }
}
/**SAI clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bclrfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#SAI1:BCLRFR)*/
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
