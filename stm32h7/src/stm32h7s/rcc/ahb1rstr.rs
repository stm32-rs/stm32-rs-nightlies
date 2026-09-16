///Register `AHB1RSTR` reader
pub type R = crate::R<AHB1RSTRrs>;
///Register `AHB1RSTR` writer
pub type W = crate::W<AHB1RSTRrs>;
/**GPDMA1 blocks reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPDMA1RST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<GPDMA1RST> for bool {
    #[inline(always)]
    fn from(variant: GPDMA1RST) -> Self {
        variant as u8 != 0
    }
}
///Field `GPDMA1RST` reader - GPDMA1 blocks reset
pub type GPDMA1RST_R = crate::BitReader<GPDMA1RST>;
impl GPDMA1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<GPDMA1RST> {
        match self.bits {
            true => Some(GPDMA1RST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == GPDMA1RST::Reset
    }
}
///Field `GPDMA1RST` writer - GPDMA1 blocks reset
pub type GPDMA1RST_W<'a, REG> = crate::BitWriter<'a, REG, GPDMA1RST>;
impl<'a, REG> GPDMA1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA1RST::Reset)
    }
}
///Field `ADC12RST` reader - ADC1 and 2 blocks reset
pub use GPDMA1RST_R as ADC12RST_R;
///Field `ETH1MACRST` reader - ETH1 block reset
pub use GPDMA1RST_R as ETH1MACRST_R;
///Field `OTGHSRST` reader - OTGHS block reset
pub use GPDMA1RST_R as OTGHSRST_R;
///Field `USBPHYCRST` reader - USBPHYC block reset
pub use GPDMA1RST_R as USBPHYCRST_R;
///Field `OTGFSRST` reader - OTGFS block reset
pub use GPDMA1RST_R as OTGFSRST_R;
///Field `ADF1RST` reader - ADF block reset
pub use GPDMA1RST_R as ADF1RST_R;
///Field `ADC12RST` writer - ADC1 and 2 blocks reset
pub use GPDMA1RST_W as ADC12RST_W;
///Field `ETH1MACRST` writer - ETH1 block reset
pub use GPDMA1RST_W as ETH1MACRST_W;
///Field `OTGHSRST` writer - OTGHS block reset
pub use GPDMA1RST_W as OTGHSRST_W;
///Field `USBPHYCRST` writer - USBPHYC block reset
pub use GPDMA1RST_W as USBPHYCRST_W;
///Field `OTGFSRST` writer - OTGFS block reset
pub use GPDMA1RST_W as OTGFSRST_W;
///Field `ADF1RST` writer - ADF block reset
pub use GPDMA1RST_W as ADF1RST_W;
impl R {
    ///Bit 4 - GPDMA1 blocks reset
    #[inline(always)]
    pub fn gpdma1rst(&self) -> GPDMA1RST_R {
        GPDMA1RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ADC1 and 2 blocks reset
    #[inline(always)]
    pub fn adc12rst(&self) -> ADC12RST_R {
        ADC12RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 15 - ETH1 block reset
    #[inline(always)]
    pub fn eth1macrst(&self) -> ETH1MACRST_R {
        ETH1MACRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 25 - OTGHS block reset
    #[inline(always)]
    pub fn otghsrst(&self) -> OTGHSRST_R {
        OTGHSRST_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - USBPHYC block reset
    #[inline(always)]
    pub fn usbphycrst(&self) -> USBPHYCRST_R {
        USBPHYCRST_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - OTGFS block reset
    #[inline(always)]
    pub fn otgfsrst(&self) -> OTGFSRST_R {
        OTGFSRST_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 31 - ADF block reset
    #[inline(always)]
    pub fn adf1rst(&self) -> ADF1RST_R {
        ADF1RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1RSTR")
            .field("gpdma1rst", &self.gpdma1rst())
            .field("adc12rst", &self.adc12rst())
            .field("eth1macrst", &self.eth1macrst())
            .field("otghsrst", &self.otghsrst())
            .field("usbphycrst", &self.usbphycrst())
            .field("otgfsrst", &self.otgfsrst())
            .field("adf1rst", &self.adf1rst())
            .finish()
    }
}
impl W {
    ///Bit 4 - GPDMA1 blocks reset
    #[inline(always)]
    pub fn gpdma1rst(&mut self) -> GPDMA1RST_W<'_, AHB1RSTRrs> {
        GPDMA1RST_W::new(self, 4)
    }
    ///Bit 5 - ADC1 and 2 blocks reset
    #[inline(always)]
    pub fn adc12rst(&mut self) -> ADC12RST_W<'_, AHB1RSTRrs> {
        ADC12RST_W::new(self, 5)
    }
    ///Bit 15 - ETH1 block reset
    #[inline(always)]
    pub fn eth1macrst(&mut self) -> ETH1MACRST_W<'_, AHB1RSTRrs> {
        ETH1MACRST_W::new(self, 15)
    }
    ///Bit 25 - OTGHS block reset
    #[inline(always)]
    pub fn otghsrst(&mut self) -> OTGHSRST_W<'_, AHB1RSTRrs> {
        OTGHSRST_W::new(self, 25)
    }
    ///Bit 26 - USBPHYC block reset
    #[inline(always)]
    pub fn usbphycrst(&mut self) -> USBPHYCRST_W<'_, AHB1RSTRrs> {
        USBPHYCRST_W::new(self, 26)
    }
    ///Bit 27 - OTGFS block reset
    #[inline(always)]
    pub fn otgfsrst(&mut self) -> OTGFSRST_W<'_, AHB1RSTRrs> {
        OTGFSRST_W::new(self, 27)
    }
    ///Bit 31 - ADF block reset
    #[inline(always)]
    pub fn adf1rst(&mut self) -> ADF1RST_W<'_, AHB1RSTRrs> {
        ADF1RST_W::new(self, 31)
    }
}
/**RCC AHB1 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RCC:AHB1RSTR)*/
pub struct AHB1RSTRrs;
impl crate::RegisterSpec for AHB1RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb1rstr::R`](R) reader structure
impl crate::Readable for AHB1RSTRrs {}
///`write(|w| ..)` method takes [`ahb1rstr::W`](W) writer structure
impl crate::Writable for AHB1RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1RSTR to value 0
impl crate::Resettable for AHB1RSTRrs {}
