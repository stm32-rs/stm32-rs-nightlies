///Register `AHB1LPENR` reader
pub type R = crate::R<AHB1LPENRrs>;
///Register `AHB1LPENR` writer
pub type W = crate::W<AHB1LPENRrs>;
/**GPDMA1 clock enable in low-power mode

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPDMA1LPEN {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<GPDMA1LPEN> for bool {
    #[inline(always)]
    fn from(variant: GPDMA1LPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPDMA1LPEN` reader - GPDMA1 clock enable in low-power mode
pub type GPDMA1LPEN_R = crate::BitReader<GPDMA1LPEN>;
impl GPDMA1LPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPDMA1LPEN {
        match self.bits {
            false => GPDMA1LPEN::Disabled,
            true => GPDMA1LPEN::Enabled,
        }
    }
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPDMA1LPEN::Disabled
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPDMA1LPEN::Enabled
    }
}
///Field `GPDMA1LPEN` writer - GPDMA1 clock enable in low-power mode
pub type GPDMA1LPEN_W<'a, REG> = crate::BitWriter<'a, REG, GPDMA1LPEN>;
impl<'a, REG> GPDMA1LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA1LPEN::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA1LPEN::Enabled)
    }
}
///Field `ADC12LPEN` reader - ADC1 and 2 peripheral clocks enable in low-power mode
pub use GPDMA1LPEN_R as ADC12LPEN_R;
///Field `ETH1MACLPEN` reader - ETH1 MAC peripheral clock enable in low-power mode
pub use GPDMA1LPEN_R as ETH1MACLPEN_R;
///Field `ETH1TXLPEN` reader - ETH1 transmission peripheral clock enable in low-power mode
pub use GPDMA1LPEN_R as ETH1TXLPEN_R;
///Field `ETH1RXLPEN` reader - ETH1 reception peripheral clock enable in low-power mode
pub use GPDMA1LPEN_R as ETH1RXLPEN_R;
///Field `ADC12LPEN` writer - ADC1 and 2 peripheral clocks enable in low-power mode
pub use GPDMA1LPEN_W as ADC12LPEN_W;
///Field `ETH1MACLPEN` writer - ETH1 MAC peripheral clock enable in low-power mode
pub use GPDMA1LPEN_W as ETH1MACLPEN_W;
///Field `ETH1TXLPEN` writer - ETH1 transmission peripheral clock enable in low-power mode
pub use GPDMA1LPEN_W as ETH1TXLPEN_W;
///Field `ETH1RXLPEN` writer - ETH1 reception peripheral clock enable in low-power mode
pub use GPDMA1LPEN_W as ETH1RXLPEN_W;
///Field `UCPDCTRL` reader - USBPHYC common block power-down control
pub type UCPDCTRL_R = crate::BitReader;
///Field `UCPDCTRL` writer - USBPHYC common block power-down control
pub type UCPDCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGHSLPEN` reader - OTGHS peripheral clock enable in low-power mode
pub use GPDMA1LPEN_R as OTGHSLPEN_R;
///Field `USBPHYCLPEN` reader - USBPHYC peripheral clock enable in low-power mode
pub use GPDMA1LPEN_R as USBPHYCLPEN_R;
///Field `OTGFSLPEN` reader - OTGFS clock enable in low-power mode
pub use GPDMA1LPEN_R as OTGFSLPEN_R;
///Field `ADF1LPEN` reader - ADF clock enable in low-power mode
pub use GPDMA1LPEN_R as ADF1LPEN_R;
///Field `OTGHSLPEN` writer - OTGHS peripheral clock enable in low-power mode
pub use GPDMA1LPEN_W as OTGHSLPEN_W;
///Field `USBPHYCLPEN` writer - USBPHYC peripheral clock enable in low-power mode
pub use GPDMA1LPEN_W as USBPHYCLPEN_W;
///Field `OTGFSLPEN` writer - OTGFS clock enable in low-power mode
pub use GPDMA1LPEN_W as OTGFSLPEN_W;
///Field `ADF1LPEN` writer - ADF clock enable in low-power mode
pub use GPDMA1LPEN_W as ADF1LPEN_W;
impl R {
    ///Bit 4 - GPDMA1 clock enable in low-power mode
    #[inline(always)]
    pub fn gpdma1lpen(&self) -> GPDMA1LPEN_R {
        GPDMA1LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ADC1 and 2 peripheral clocks enable in low-power mode
    #[inline(always)]
    pub fn adc12lpen(&self) -> ADC12LPEN_R {
        ADC12LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 15 - ETH1 MAC peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn eth1maclpen(&self) -> ETH1MACLPEN_R {
        ETH1MACLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - ETH1 transmission peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn eth1txlpen(&self) -> ETH1TXLPEN_R {
        ETH1TXLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ETH1 reception peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn eth1rxlpen(&self) -> ETH1RXLPEN_R {
        ETH1RXLPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 24 - USBPHYC common block power-down control
    #[inline(always)]
    pub fn ucpdctrl(&self) -> UCPDCTRL_R {
        UCPDCTRL_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - OTGHS peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn otghslpen(&self) -> OTGHSLPEN_R {
        OTGHSLPEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - USBPHYC peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn usbphyclpen(&self) -> USBPHYCLPEN_R {
        USBPHYCLPEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - OTGFS clock enable in low-power mode
    #[inline(always)]
    pub fn otgfslpen(&self) -> OTGFSLPEN_R {
        OTGFSLPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 31 - ADF clock enable in low-power mode
    #[inline(always)]
    pub fn adf1lpen(&self) -> ADF1LPEN_R {
        ADF1LPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1LPENR")
            .field("gpdma1lpen", &self.gpdma1lpen())
            .field("adc12lpen", &self.adc12lpen())
            .field("eth1maclpen", &self.eth1maclpen())
            .field("eth1txlpen", &self.eth1txlpen())
            .field("eth1rxlpen", &self.eth1rxlpen())
            .field("ucpdctrl", &self.ucpdctrl())
            .field("otghslpen", &self.otghslpen())
            .field("usbphyclpen", &self.usbphyclpen())
            .field("otgfslpen", &self.otgfslpen())
            .field("adf1lpen", &self.adf1lpen())
            .finish()
    }
}
impl W {
    ///Bit 4 - GPDMA1 clock enable in low-power mode
    #[inline(always)]
    pub fn gpdma1lpen(&mut self) -> GPDMA1LPEN_W<'_, AHB1LPENRrs> {
        GPDMA1LPEN_W::new(self, 4)
    }
    ///Bit 5 - ADC1 and 2 peripheral clocks enable in low-power mode
    #[inline(always)]
    pub fn adc12lpen(&mut self) -> ADC12LPEN_W<'_, AHB1LPENRrs> {
        ADC12LPEN_W::new(self, 5)
    }
    ///Bit 15 - ETH1 MAC peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn eth1maclpen(&mut self) -> ETH1MACLPEN_W<'_, AHB1LPENRrs> {
        ETH1MACLPEN_W::new(self, 15)
    }
    ///Bit 16 - ETH1 transmission peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn eth1txlpen(&mut self) -> ETH1TXLPEN_W<'_, AHB1LPENRrs> {
        ETH1TXLPEN_W::new(self, 16)
    }
    ///Bit 17 - ETH1 reception peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn eth1rxlpen(&mut self) -> ETH1RXLPEN_W<'_, AHB1LPENRrs> {
        ETH1RXLPEN_W::new(self, 17)
    }
    ///Bit 24 - USBPHYC common block power-down control
    #[inline(always)]
    pub fn ucpdctrl(&mut self) -> UCPDCTRL_W<'_, AHB1LPENRrs> {
        UCPDCTRL_W::new(self, 24)
    }
    ///Bit 25 - OTGHS peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn otghslpen(&mut self) -> OTGHSLPEN_W<'_, AHB1LPENRrs> {
        OTGHSLPEN_W::new(self, 25)
    }
    ///Bit 26 - USBPHYC peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn usbphyclpen(&mut self) -> USBPHYCLPEN_W<'_, AHB1LPENRrs> {
        USBPHYCLPEN_W::new(self, 26)
    }
    ///Bit 27 - OTGFS clock enable in low-power mode
    #[inline(always)]
    pub fn otgfslpen(&mut self) -> OTGFSLPEN_W<'_, AHB1LPENRrs> {
        OTGFSLPEN_W::new(self, 27)
    }
    ///Bit 31 - ADF clock enable in low-power mode
    #[inline(always)]
    pub fn adf1lpen(&mut self) -> ADF1LPEN_W<'_, AHB1LPENRrs> {
        ADF1LPEN_W::new(self, 31)
    }
}
/**RCC AHB1 low-power clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb1lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RCC:AHB1LPENR)*/
pub struct AHB1LPENRrs;
impl crate::RegisterSpec for AHB1LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb1lpenr::R`](R) reader structure
impl crate::Readable for AHB1LPENRrs {}
///`write(|w| ..)` method takes [`ahb1lpenr::W`](W) writer structure
impl crate::Writable for AHB1LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1LPENR to value 0x8e03_8030
impl crate::Resettable for AHB1LPENRrs {
    const RESET_VALUE: u32 = 0x8e03_8030;
}
