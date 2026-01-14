///Register `SDRTR` reader
pub type R = crate::R<SDRTRrs>;
///Register `SDRTR` writer
pub type W = crate::W<SDRTRrs>;
/**Clear Refresh error flag This bit is used to clear the Refresh Error Flag (RE) in the Status Register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRE {
    ///1: Refresh Error Flag is cleared
    Clear = 1,
}
impl From<CRE> for bool {
    #[inline(always)]
    fn from(variant: CRE) -> Self {
        variant as u8 != 0
    }
}
///Field `CRE` writer - Clear Refresh error flag This bit is used to clear the Refresh Error Flag (RE) in the Status Register.
pub type CRE_W<'a, REG> = crate::BitWriter<'a, REG, CRE>;
impl<'a, REG> CRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Refresh Error Flag is cleared
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CRE::Clear)
    }
}
///Field `COUNT` reader - Refresh Timer Count This 13-bit field defines the refresh rate of the SDRAM device. It is expressed in number of memory clock cycles. It must be set at least to 41 SDRAM clock cycles (0x29). Refresh rate = (COUNT + 1) x SDRAM frequency clock COUNT = (SDRAM refresh period / Number of rows) - 20
pub type COUNT_R = crate::FieldReader<u16>;
///Field `COUNT` writer - Refresh Timer Count This 13-bit field defines the refresh rate of the SDRAM device. It is expressed in number of memory clock cycles. It must be set at least to 41 SDRAM clock cycles (0x29). Refresh rate = (COUNT + 1) x SDRAM frequency clock COUNT = (SDRAM refresh period / Number of rows) - 20
pub type COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16, crate::Safe>;
/**RES Interrupt Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REIE {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is generated if RE = 1
    Enabled = 1,
}
impl From<REIE> for bool {
    #[inline(always)]
    fn from(variant: REIE) -> Self {
        variant as u8 != 0
    }
}
///Field `REIE` reader - RES Interrupt Enable
pub type REIE_R = crate::BitReader<REIE>;
impl REIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REIE {
        match self.bits {
            false => REIE::Disabled,
            true => REIE::Enabled,
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REIE::Disabled
    }
    ///Interrupt is generated if RE = 1
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REIE::Enabled
    }
}
///Field `REIE` writer - RES Interrupt Enable
pub type REIE_W<'a, REG> = crate::BitWriter<'a, REG, REIE>;
impl<'a, REG> REIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(REIE::Disabled)
    }
    ///Interrupt is generated if RE = 1
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(REIE::Enabled)
    }
}
impl R {
    ///Bits 1:13 - Refresh Timer Count This 13-bit field defines the refresh rate of the SDRAM device. It is expressed in number of memory clock cycles. It must be set at least to 41 SDRAM clock cycles (0x29). Refresh rate = (COUNT + 1) x SDRAM frequency clock COUNT = (SDRAM refresh period / Number of rows) - 20
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 1) & 0x1fff) as u16)
    }
    ///Bit 14 - RES Interrupt Enable
    #[inline(always)]
    pub fn reie(&self) -> REIE_R {
        REIE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDRTR")
            .field("count", &self.count())
            .field("reie", &self.reie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clear Refresh error flag This bit is used to clear the Refresh Error Flag (RE) in the Status Register.
    #[inline(always)]
    pub fn cre(&mut self) -> CRE_W<'_, SDRTRrs> {
        CRE_W::new(self, 0)
    }
    ///Bits 1:13 - Refresh Timer Count This 13-bit field defines the refresh rate of the SDRAM device. It is expressed in number of memory clock cycles. It must be set at least to 41 SDRAM clock cycles (0x29). Refresh rate = (COUNT + 1) x SDRAM frequency clock COUNT = (SDRAM refresh period / Number of rows) - 20
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W<'_, SDRTRrs> {
        COUNT_W::new(self, 1)
    }
    ///Bit 14 - RES Interrupt Enable
    #[inline(always)]
    pub fn reie(&mut self) -> REIE_W<'_, SDRTRrs> {
        REIE_W::new(self, 14)
    }
}
/**This register sets the refresh rate in number of SDCLK clock cycles between the refresh cycles by configuring the Refresh Timer Count value.Examplewhere 64 ms is the SDRAM refresh period.The refresh rate must be increased by 20 SDRAM clock cycles (as in the above example) to obtain a safe margin if an internal refresh request occurs when a read request has been accepted. It corresponds to a COUNT value of 0000111000000 (448). This 13-bit field is loaded into a timer which is decremented using the SDRAM clock. This timer generates a refresh pulse when zero is reached. The COUNT value must be set at least to 41 SDRAM clock cycles.As soon as the FMC_SDRTR register is programmed, the timer starts counting. If the value programmed in the register is 0, no refresh is carried out. This register must not be reprogrammed after the initialization procedure to avoid modifying the refresh rate.Each time a refresh pulse is generated, this 13-bit COUNT field is reloaded into the counter.If a memory access is in progress, the Auto-refresh request is delayed. However, if the memory access and Auto-refresh requests are generated simultaneously, the Auto-refresh takes precedence. If the memory access occurs during a refresh operation, the request is buffered to be processed when the refresh is complete.This register is common to SDRAM bank 1 and bank 2.

You can [`read`](crate::Reg::read) this register and get [`sdrtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdrtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#FMC:SDRTR)*/
pub struct SDRTRrs;
impl crate::RegisterSpec for SDRTRrs {
    type Ux = u32;
}
///`read()` method returns [`sdrtr::R`](R) reader structure
impl crate::Readable for SDRTRrs {}
///`write(|w| ..)` method takes [`sdrtr::W`](W) writer structure
impl crate::Writable for SDRTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDRTR to value 0
impl crate::Resettable for SDRTRrs {}
