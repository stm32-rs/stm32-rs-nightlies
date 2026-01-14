///Register `AHB1RSTR` reader
pub type R = crate::R<AHB1RSTRrs>;
///Register `AHB1RSTR` writer
pub type W = crate::W<AHB1RSTRrs>;
/**DMA1 and DMAMUX1 blocks reset Set and reset by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1RST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<DMA1RST> for bool {
    #[inline(always)]
    fn from(variant: DMA1RST) -> Self {
        variant as u8 != 0
    }
}
///Field `DMA1RST` reader - DMA1 and DMAMUX1 blocks reset Set and reset by software.
pub type DMA1RST_R = crate::BitReader<DMA1RST>;
impl DMA1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DMA1RST> {
        match self.bits {
            true => Some(DMA1RST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DMA1RST::Reset
    }
}
///Field `DMA1RST` writer - DMA1 and DMAMUX1 blocks reset Set and reset by software.
pub type DMA1RST_W<'a, REG> = crate::BitWriter<'a, REG, DMA1RST>;
impl<'a, REG> DMA1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1RST::Reset)
    }
}
///Field `DMA2RST` reader - DMA2 and DMAMUX2 blocks reset Set and reset by software.
pub use DMA1RST_R as DMA2RST_R;
///Field `ADC12RST` reader - ADC1 and 2 blocks reset Set and reset by software.
pub use DMA1RST_R as ADC12RST_R;
///Field `CRCRST` reader - CRC block reset Set and reset by software.
pub use DMA1RST_R as CRCRST_R;
///Field `USB1OTGRST` reader - USB1OTG block reset Set and reset by software.
pub use DMA1RST_R as USB1OTGRST_R;
///Field `DMA2RST` writer - DMA2 and DMAMUX2 blocks reset Set and reset by software.
pub use DMA1RST_W as DMA2RST_W;
///Field `ADC12RST` writer - ADC1 and 2 blocks reset Set and reset by software.
pub use DMA1RST_W as ADC12RST_W;
///Field `CRCRST` writer - CRC block reset Set and reset by software.
pub use DMA1RST_W as CRCRST_W;
///Field `USB1OTGRST` writer - USB1OTG block reset Set and reset by software.
pub use DMA1RST_W as USB1OTGRST_W;
impl R {
    ///Bit 0 - DMA1 and DMAMUX1 blocks reset Set and reset by software.
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2 and DMAMUX2 blocks reset Set and reset by software.
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - ADC1 and 2 blocks reset Set and reset by software.
    #[inline(always)]
    pub fn adc12rst(&self) -> ADC12RST_R {
        ADC12RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 9 - CRC block reset Set and reset by software.
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 25 - USB1OTG block reset Set and reset by software.
    #[inline(always)]
    pub fn usb1otgrst(&self) -> USB1OTGRST_R {
        USB1OTGRST_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1RSTR")
            .field("dma1rst", &self.dma1rst())
            .field("dma2rst", &self.dma2rst())
            .field("adc12rst", &self.adc12rst())
            .field("crcrst", &self.crcrst())
            .field("usb1otgrst", &self.usb1otgrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1 and DMAMUX1 blocks reset Set and reset by software.
    #[inline(always)]
    pub fn dma1rst(&mut self) -> DMA1RST_W<'_, AHB1RSTRrs> {
        DMA1RST_W::new(self, 0)
    }
    ///Bit 1 - DMA2 and DMAMUX2 blocks reset Set and reset by software.
    #[inline(always)]
    pub fn dma2rst(&mut self) -> DMA2RST_W<'_, AHB1RSTRrs> {
        DMA2RST_W::new(self, 1)
    }
    ///Bit 5 - ADC1 and 2 blocks reset Set and reset by software.
    #[inline(always)]
    pub fn adc12rst(&mut self) -> ADC12RST_W<'_, AHB1RSTRrs> {
        ADC12RST_W::new(self, 5)
    }
    ///Bit 9 - CRC block reset Set and reset by software.
    #[inline(always)]
    pub fn crcrst(&mut self) -> CRCRST_W<'_, AHB1RSTRrs> {
        CRCRST_W::new(self, 9)
    }
    ///Bit 25 - USB1OTG block reset Set and reset by software.
    #[inline(always)]
    pub fn usb1otgrst(&mut self) -> USB1OTGRST_W<'_, AHB1RSTRrs> {
        USB1OTGRST_W::new(self, 25)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#RCC:AHB1RSTR)*/
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
