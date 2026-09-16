///Register `D3AMR` reader
pub type R = crate::R<D3AMRrs>;
///Register `D3AMR` writer
pub type W = crate::W<D3AMRrs>;
/**BDMA and DMAMUX Autonomous mode enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BDMAAMEN {
    ///0: Clock disabled in autonomous mode
    Disabled = 0,
    ///1: Clock enabled in autonomous mode
    Enabled = 1,
}
impl From<BDMAAMEN> for bool {
    #[inline(always)]
    fn from(variant: BDMAAMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `BDMAAMEN` reader - BDMA and DMAMUX Autonomous mode enable
pub type BDMAAMEN_R = crate::BitReader<BDMAAMEN>;
impl BDMAAMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BDMAAMEN {
        match self.bits {
            false => BDMAAMEN::Disabled,
            true => BDMAAMEN::Enabled,
        }
    }
    ///Clock disabled in autonomous mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BDMAAMEN::Disabled
    }
    ///Clock enabled in autonomous mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BDMAAMEN::Enabled
    }
}
///Field `BDMAAMEN` writer - BDMA and DMAMUX Autonomous mode enable
pub type BDMAAMEN_W<'a, REG> = crate::BitWriter<'a, REG, BDMAAMEN>;
impl<'a, REG> BDMAAMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled in autonomous mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BDMAAMEN::Disabled)
    }
    ///Clock enabled in autonomous mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BDMAAMEN::Enabled)
    }
}
///Field `LPUART1AMEN` reader - LPUART1 Autonomous mode enable
pub use BDMAAMEN_R as LPUART1AMEN_R;
///Field `SPI6AMEN` reader - SPI6 Autonomous mode enable
pub use BDMAAMEN_R as SPI6AMEN_R;
///Field `I2C4AMEN` reader - I2C4 Autonomous mode enable
pub use BDMAAMEN_R as I2C4AMEN_R;
///Field `LPTIM2AMEN` reader - LPTIM2 Autonomous mode enable
pub use BDMAAMEN_R as LPTIM2AMEN_R;
///Field `LPTIM3AMEN` reader - LPTIM3 Autonomous mode enable
pub use BDMAAMEN_R as LPTIM3AMEN_R;
///Field `LPTIM4AMEN` reader - LPTIM4 Autonomous mode enable
pub use BDMAAMEN_R as LPTIM4AMEN_R;
///Field `LPTIM5AMEN` reader - LPTIM5 Autonomous mode enable
pub use BDMAAMEN_R as LPTIM5AMEN_R;
///Field `COMP12AMEN` reader - COMP12 Autonomous mode enable
pub use BDMAAMEN_R as COMP12AMEN_R;
///Field `VREFAMEN` reader - VREF Autonomous mode enable
pub use BDMAAMEN_R as VREFAMEN_R;
///Field `RTCAMEN` reader - RTC Autonomous mode enable
pub use BDMAAMEN_R as RTCAMEN_R;
///Field `CRCAMEN` reader - CRC Autonomous mode enable
pub use BDMAAMEN_R as CRCAMEN_R;
///Field `SAI4AMEN` reader - SAI4 Autonomous mode enable
pub use BDMAAMEN_R as SAI4AMEN_R;
///Field `ADC3AMEN` reader - ADC3 Autonomous mode enable
pub use BDMAAMEN_R as ADC3AMEN_R;
///Field `DTSAMEN` reader - Digital temperature sensor Autonomous mode enable Set and reset by software. Refer to for additional information.
pub use BDMAAMEN_R as DTSAMEN_R;
///Field `BKPSRAMAMEN` reader - Backup RAM Autonomous mode enable
pub use BDMAAMEN_R as BKPSRAMAMEN_R;
///Field `SRAM4AMEN` reader - SRAM4 Autonomous mode enable
pub use BDMAAMEN_R as SRAM4AMEN_R;
///Field `LPUART1AMEN` writer - LPUART1 Autonomous mode enable
pub use BDMAAMEN_W as LPUART1AMEN_W;
///Field `SPI6AMEN` writer - SPI6 Autonomous mode enable
pub use BDMAAMEN_W as SPI6AMEN_W;
///Field `I2C4AMEN` writer - I2C4 Autonomous mode enable
pub use BDMAAMEN_W as I2C4AMEN_W;
///Field `LPTIM2AMEN` writer - LPTIM2 Autonomous mode enable
pub use BDMAAMEN_W as LPTIM2AMEN_W;
///Field `LPTIM3AMEN` writer - LPTIM3 Autonomous mode enable
pub use BDMAAMEN_W as LPTIM3AMEN_W;
///Field `LPTIM4AMEN` writer - LPTIM4 Autonomous mode enable
pub use BDMAAMEN_W as LPTIM4AMEN_W;
///Field `LPTIM5AMEN` writer - LPTIM5 Autonomous mode enable
pub use BDMAAMEN_W as LPTIM5AMEN_W;
///Field `COMP12AMEN` writer - COMP12 Autonomous mode enable
pub use BDMAAMEN_W as COMP12AMEN_W;
///Field `VREFAMEN` writer - VREF Autonomous mode enable
pub use BDMAAMEN_W as VREFAMEN_W;
///Field `RTCAMEN` writer - RTC Autonomous mode enable
pub use BDMAAMEN_W as RTCAMEN_W;
///Field `CRCAMEN` writer - CRC Autonomous mode enable
pub use BDMAAMEN_W as CRCAMEN_W;
///Field `SAI4AMEN` writer - SAI4 Autonomous mode enable
pub use BDMAAMEN_W as SAI4AMEN_W;
///Field `ADC3AMEN` writer - ADC3 Autonomous mode enable
pub use BDMAAMEN_W as ADC3AMEN_W;
///Field `DTSAMEN` writer - Digital temperature sensor Autonomous mode enable Set and reset by software. Refer to for additional information.
pub use BDMAAMEN_W as DTSAMEN_W;
///Field `BKPSRAMAMEN` writer - Backup RAM Autonomous mode enable
pub use BDMAAMEN_W as BKPSRAMAMEN_W;
///Field `SRAM4AMEN` writer - SRAM4 Autonomous mode enable
pub use BDMAAMEN_W as SRAM4AMEN_W;
impl R {
    ///Bit 0 - BDMA and DMAMUX Autonomous mode enable
    #[inline(always)]
    pub fn bdmaamen(&self) -> BDMAAMEN_R {
        BDMAAMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - LPUART1 Autonomous mode enable
    #[inline(always)]
    pub fn lpuart1amen(&self) -> LPUART1AMEN_R {
        LPUART1AMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - SPI6 Autonomous mode enable
    #[inline(always)]
    pub fn spi6amen(&self) -> SPI6AMEN_R {
        SPI6AMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - I2C4 Autonomous mode enable
    #[inline(always)]
    pub fn i2c4amen(&self) -> I2C4AMEN_R {
        I2C4AMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - LPTIM2 Autonomous mode enable
    #[inline(always)]
    pub fn lptim2amen(&self) -> LPTIM2AMEN_R {
        LPTIM2AMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - LPTIM3 Autonomous mode enable
    #[inline(always)]
    pub fn lptim3amen(&self) -> LPTIM3AMEN_R {
        LPTIM3AMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - LPTIM4 Autonomous mode enable
    #[inline(always)]
    pub fn lptim4amen(&self) -> LPTIM4AMEN_R {
        LPTIM4AMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPTIM5 Autonomous mode enable
    #[inline(always)]
    pub fn lptim5amen(&self) -> LPTIM5AMEN_R {
        LPTIM5AMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - COMP12 Autonomous mode enable
    #[inline(always)]
    pub fn comp12amen(&self) -> COMP12AMEN_R {
        COMP12AMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - VREF Autonomous mode enable
    #[inline(always)]
    pub fn vrefamen(&self) -> VREFAMEN_R {
        VREFAMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - RTC Autonomous mode enable
    #[inline(always)]
    pub fn rtcamen(&self) -> RTCAMEN_R {
        RTCAMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 19 - CRC Autonomous mode enable
    #[inline(always)]
    pub fn crcamen(&self) -> CRCAMEN_R {
        CRCAMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - SAI4 Autonomous mode enable
    #[inline(always)]
    pub fn sai4amen(&self) -> SAI4AMEN_R {
        SAI4AMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 24 - ADC3 Autonomous mode enable
    #[inline(always)]
    pub fn adc3amen(&self) -> ADC3AMEN_R {
        ADC3AMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - Digital temperature sensor Autonomous mode enable Set and reset by software. Refer to for additional information.
    #[inline(always)]
    pub fn dtsamen(&self) -> DTSAMEN_R {
        DTSAMEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - Backup RAM Autonomous mode enable
    #[inline(always)]
    pub fn bkpsramamen(&self) -> BKPSRAMAMEN_R {
        BKPSRAMAMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SRAM4 Autonomous mode enable
    #[inline(always)]
    pub fn sram4amen(&self) -> SRAM4AMEN_R {
        SRAM4AMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D3AMR")
            .field("bdmaamen", &self.bdmaamen())
            .field("lpuart1amen", &self.lpuart1amen())
            .field("spi6amen", &self.spi6amen())
            .field("i2c4amen", &self.i2c4amen())
            .field("lptim2amen", &self.lptim2amen())
            .field("lptim3amen", &self.lptim3amen())
            .field("lptim4amen", &self.lptim4amen())
            .field("lptim5amen", &self.lptim5amen())
            .field("comp12amen", &self.comp12amen())
            .field("vrefamen", &self.vrefamen())
            .field("rtcamen", &self.rtcamen())
            .field("crcamen", &self.crcamen())
            .field("sai4amen", &self.sai4amen())
            .field("adc3amen", &self.adc3amen())
            .field("dtsamen", &self.dtsamen())
            .field("bkpsramamen", &self.bkpsramamen())
            .field("sram4amen", &self.sram4amen())
            .finish()
    }
}
impl W {
    ///Bit 0 - BDMA and DMAMUX Autonomous mode enable
    #[inline(always)]
    pub fn bdmaamen(&mut self) -> BDMAAMEN_W<'_, D3AMRrs> {
        BDMAAMEN_W::new(self, 0)
    }
    ///Bit 3 - LPUART1 Autonomous mode enable
    #[inline(always)]
    pub fn lpuart1amen(&mut self) -> LPUART1AMEN_W<'_, D3AMRrs> {
        LPUART1AMEN_W::new(self, 3)
    }
    ///Bit 5 - SPI6 Autonomous mode enable
    #[inline(always)]
    pub fn spi6amen(&mut self) -> SPI6AMEN_W<'_, D3AMRrs> {
        SPI6AMEN_W::new(self, 5)
    }
    ///Bit 7 - I2C4 Autonomous mode enable
    #[inline(always)]
    pub fn i2c4amen(&mut self) -> I2C4AMEN_W<'_, D3AMRrs> {
        I2C4AMEN_W::new(self, 7)
    }
    ///Bit 9 - LPTIM2 Autonomous mode enable
    #[inline(always)]
    pub fn lptim2amen(&mut self) -> LPTIM2AMEN_W<'_, D3AMRrs> {
        LPTIM2AMEN_W::new(self, 9)
    }
    ///Bit 10 - LPTIM3 Autonomous mode enable
    #[inline(always)]
    pub fn lptim3amen(&mut self) -> LPTIM3AMEN_W<'_, D3AMRrs> {
        LPTIM3AMEN_W::new(self, 10)
    }
    ///Bit 11 - LPTIM4 Autonomous mode enable
    #[inline(always)]
    pub fn lptim4amen(&mut self) -> LPTIM4AMEN_W<'_, D3AMRrs> {
        LPTIM4AMEN_W::new(self, 11)
    }
    ///Bit 12 - LPTIM5 Autonomous mode enable
    #[inline(always)]
    pub fn lptim5amen(&mut self) -> LPTIM5AMEN_W<'_, D3AMRrs> {
        LPTIM5AMEN_W::new(self, 12)
    }
    ///Bit 14 - COMP12 Autonomous mode enable
    #[inline(always)]
    pub fn comp12amen(&mut self) -> COMP12AMEN_W<'_, D3AMRrs> {
        COMP12AMEN_W::new(self, 14)
    }
    ///Bit 15 - VREF Autonomous mode enable
    #[inline(always)]
    pub fn vrefamen(&mut self) -> VREFAMEN_W<'_, D3AMRrs> {
        VREFAMEN_W::new(self, 15)
    }
    ///Bit 16 - RTC Autonomous mode enable
    #[inline(always)]
    pub fn rtcamen(&mut self) -> RTCAMEN_W<'_, D3AMRrs> {
        RTCAMEN_W::new(self, 16)
    }
    ///Bit 19 - CRC Autonomous mode enable
    #[inline(always)]
    pub fn crcamen(&mut self) -> CRCAMEN_W<'_, D3AMRrs> {
        CRCAMEN_W::new(self, 19)
    }
    ///Bit 21 - SAI4 Autonomous mode enable
    #[inline(always)]
    pub fn sai4amen(&mut self) -> SAI4AMEN_W<'_, D3AMRrs> {
        SAI4AMEN_W::new(self, 21)
    }
    ///Bit 24 - ADC3 Autonomous mode enable
    #[inline(always)]
    pub fn adc3amen(&mut self) -> ADC3AMEN_W<'_, D3AMRrs> {
        ADC3AMEN_W::new(self, 24)
    }
    ///Bit 26 - Digital temperature sensor Autonomous mode enable Set and reset by software. Refer to for additional information.
    #[inline(always)]
    pub fn dtsamen(&mut self) -> DTSAMEN_W<'_, D3AMRrs> {
        DTSAMEN_W::new(self, 26)
    }
    ///Bit 28 - Backup RAM Autonomous mode enable
    #[inline(always)]
    pub fn bkpsramamen(&mut self) -> BKPSRAMAMEN_W<'_, D3AMRrs> {
        BKPSRAMAMEN_W::new(self, 28)
    }
    ///Bit 29 - SRAM4 Autonomous mode enable
    #[inline(always)]
    pub fn sram4amen(&mut self) -> SRAM4AMEN_W<'_, D3AMRrs> {
        SRAM4AMEN_W::new(self, 29)
    }
}
/**RCC D3 Autonomous mode Register

You can [`read`](crate::Reg::read) this register and get [`d3amr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3amr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#RCC:D3AMR)*/
pub struct D3AMRrs;
impl crate::RegisterSpec for D3AMRrs {
    type Ux = u32;
}
///`read()` method returns [`d3amr::R`](R) reader structure
impl crate::Readable for D3AMRrs {}
///`write(|w| ..)` method takes [`d3amr::W`](W) writer structure
impl crate::Writable for D3AMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D3AMR to value 0
impl crate::Resettable for D3AMRrs {}
