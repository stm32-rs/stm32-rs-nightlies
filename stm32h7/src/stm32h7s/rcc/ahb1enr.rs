///Register `AHB1ENR` reader
pub type R = crate::R<AHB1ENRrs>;
///Register `AHB1ENR` writer
pub type W = crate::W<AHB1ENRrs>;
/**GPDMA1 clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPDMA1EN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<GPDMA1EN> for bool {
    #[inline(always)]
    fn from(variant: GPDMA1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPDMA1EN` reader - GPDMA1 clock enable
pub type GPDMA1EN_R = crate::BitReader<GPDMA1EN>;
impl GPDMA1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPDMA1EN {
        match self.bits {
            false => GPDMA1EN::Disabled,
            true => GPDMA1EN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPDMA1EN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPDMA1EN::Enabled
    }
}
///Field `GPDMA1EN` writer - GPDMA1 clock enable
pub type GPDMA1EN_W<'a, REG> = crate::BitWriter<'a, REG, GPDMA1EN>;
impl<'a, REG> GPDMA1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA1EN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA1EN::Enabled)
    }
}
///Field `ADC12EN` reader - ADC1 and 2 peripheral clocks enable
pub use GPDMA1EN_R as ADC12EN_R;
///Field `ETH1MACEN` reader - ETH1 MAC peripheral clock enable
pub use GPDMA1EN_R as ETH1MACEN_R;
///Field `ETH1TXEN` reader - ETH1 transmission clock enable
pub use GPDMA1EN_R as ETH1TXEN_R;
///Field `ETH1RXEN` reader - ETH1 reception clock enable
pub use GPDMA1EN_R as ETH1RXEN_R;
///Field `OTGHSEN` reader - OTGHS clocks enable
pub use GPDMA1EN_R as OTGHSEN_R;
///Field `USBPHYCEN` reader - USBPHYC clocks enable
pub use GPDMA1EN_R as USBPHYCEN_R;
///Field `OTGFSEN` reader - OTGFS peripheral clocks enable
pub use GPDMA1EN_R as OTGFSEN_R;
///Field `ADF1EN` reader - ADF clocks enable
pub use GPDMA1EN_R as ADF1EN_R;
///Field `ADC12EN` writer - ADC1 and 2 peripheral clocks enable
pub use GPDMA1EN_W as ADC12EN_W;
///Field `ETH1MACEN` writer - ETH1 MAC peripheral clock enable
pub use GPDMA1EN_W as ETH1MACEN_W;
///Field `ETH1TXEN` writer - ETH1 transmission clock enable
pub use GPDMA1EN_W as ETH1TXEN_W;
///Field `ETH1RXEN` writer - ETH1 reception clock enable
pub use GPDMA1EN_W as ETH1RXEN_W;
///Field `OTGHSEN` writer - OTGHS clocks enable
pub use GPDMA1EN_W as OTGHSEN_W;
///Field `USBPHYCEN` writer - USBPHYC clocks enable
pub use GPDMA1EN_W as USBPHYCEN_W;
///Field `OTGFSEN` writer - OTGFS peripheral clocks enable
pub use GPDMA1EN_W as OTGFSEN_W;
///Field `ADF1EN` writer - ADF clocks enable
pub use GPDMA1EN_W as ADF1EN_W;
impl R {
    ///Bit 4 - GPDMA1 clock enable
    #[inline(always)]
    pub fn gpdma1en(&self) -> GPDMA1EN_R {
        GPDMA1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ADC1 and 2 peripheral clocks enable
    #[inline(always)]
    pub fn adc12en(&self) -> ADC12EN_R {
        ADC12EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 15 - ETH1 MAC peripheral clock enable
    #[inline(always)]
    pub fn eth1macen(&self) -> ETH1MACEN_R {
        ETH1MACEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - ETH1 transmission clock enable
    #[inline(always)]
    pub fn eth1txen(&self) -> ETH1TXEN_R {
        ETH1TXEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ETH1 reception clock enable
    #[inline(always)]
    pub fn eth1rxen(&self) -> ETH1RXEN_R {
        ETH1RXEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 25 - OTGHS clocks enable
    #[inline(always)]
    pub fn otghsen(&self) -> OTGHSEN_R {
        OTGHSEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - USBPHYC clocks enable
    #[inline(always)]
    pub fn usbphycen(&self) -> USBPHYCEN_R {
        USBPHYCEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - OTGFS peripheral clocks enable
    #[inline(always)]
    pub fn otgfsen(&self) -> OTGFSEN_R {
        OTGFSEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 31 - ADF clocks enable
    #[inline(always)]
    pub fn adf1en(&self) -> ADF1EN_R {
        ADF1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1ENR")
            .field("gpdma1en", &self.gpdma1en())
            .field("adc12en", &self.adc12en())
            .field("eth1macen", &self.eth1macen())
            .field("eth1txen", &self.eth1txen())
            .field("eth1rxen", &self.eth1rxen())
            .field("otghsen", &self.otghsen())
            .field("usbphycen", &self.usbphycen())
            .field("otgfsen", &self.otgfsen())
            .field("adf1en", &self.adf1en())
            .finish()
    }
}
impl W {
    ///Bit 4 - GPDMA1 clock enable
    #[inline(always)]
    pub fn gpdma1en(&mut self) -> GPDMA1EN_W<'_, AHB1ENRrs> {
        GPDMA1EN_W::new(self, 4)
    }
    ///Bit 5 - ADC1 and 2 peripheral clocks enable
    #[inline(always)]
    pub fn adc12en(&mut self) -> ADC12EN_W<'_, AHB1ENRrs> {
        ADC12EN_W::new(self, 5)
    }
    ///Bit 15 - ETH1 MAC peripheral clock enable
    #[inline(always)]
    pub fn eth1macen(&mut self) -> ETH1MACEN_W<'_, AHB1ENRrs> {
        ETH1MACEN_W::new(self, 15)
    }
    ///Bit 16 - ETH1 transmission clock enable
    #[inline(always)]
    pub fn eth1txen(&mut self) -> ETH1TXEN_W<'_, AHB1ENRrs> {
        ETH1TXEN_W::new(self, 16)
    }
    ///Bit 17 - ETH1 reception clock enable
    #[inline(always)]
    pub fn eth1rxen(&mut self) -> ETH1RXEN_W<'_, AHB1ENRrs> {
        ETH1RXEN_W::new(self, 17)
    }
    ///Bit 25 - OTGHS clocks enable
    #[inline(always)]
    pub fn otghsen(&mut self) -> OTGHSEN_W<'_, AHB1ENRrs> {
        OTGHSEN_W::new(self, 25)
    }
    ///Bit 26 - USBPHYC clocks enable
    #[inline(always)]
    pub fn usbphycen(&mut self) -> USBPHYCEN_W<'_, AHB1ENRrs> {
        USBPHYCEN_W::new(self, 26)
    }
    ///Bit 27 - OTGFS peripheral clocks enable
    #[inline(always)]
    pub fn otgfsen(&mut self) -> OTGFSEN_W<'_, AHB1ENRrs> {
        OTGFSEN_W::new(self, 27)
    }
    ///Bit 31 - ADF clocks enable
    #[inline(always)]
    pub fn adf1en(&mut self) -> ADF1EN_W<'_, AHB1ENRrs> {
        ADF1EN_W::new(self, 31)
    }
}
/**RCC AHB1 clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb1enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RCC:AHB1ENR)*/
pub struct AHB1ENRrs;
impl crate::RegisterSpec for AHB1ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb1enr::R`](R) reader structure
impl crate::Readable for AHB1ENRrs {}
///`write(|w| ..)` method takes [`ahb1enr::W`](W) writer structure
impl crate::Writable for AHB1ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1ENR to value 0
impl crate::Resettable for AHB1ENRrs {}
