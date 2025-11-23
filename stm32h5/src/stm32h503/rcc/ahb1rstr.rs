///Register `AHB1RSTR` reader
pub type R = crate::R<AHB1RSTRrs>;
///Register `AHB1RSTR` writer
pub type W = crate::W<AHB1RSTRrs>;
/**GPDMA1 block reset Set and reset by software.

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
///Field `GPDMA1RST` reader - GPDMA1 block reset Set and reset by software.
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
///Field `GPDMA1RST` writer - GPDMA1 block reset Set and reset by software.
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
///Field `GPDMA2RST` reader - GPDMA2 block reset Set and reset by software.
pub use GPDMA1RST_R as GPDMA2RST_R;
///Field `CRCRST` reader - CRC block reset Set and reset by software.
pub use GPDMA1RST_R as CRCRST_R;
///Field `RAMCFGRST` reader - RAMCFG block reset Set and reset by software.
pub use GPDMA1RST_R as RAMCFGRST_R;
///Field `GPDMA2RST` writer - GPDMA2 block reset Set and reset by software.
pub use GPDMA1RST_W as GPDMA2RST_W;
///Field `CRCRST` writer - CRC block reset Set and reset by software.
pub use GPDMA1RST_W as CRCRST_W;
///Field `RAMCFGRST` writer - RAMCFG block reset Set and reset by software.
pub use GPDMA1RST_W as RAMCFGRST_W;
impl R {
    ///Bit 0 - GPDMA1 block reset Set and reset by software.
    #[inline(always)]
    pub fn gpdma1rst(&self) -> GPDMA1RST_R {
        GPDMA1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPDMA2 block reset Set and reset by software.
    #[inline(always)]
    pub fn gpdma2rst(&self) -> GPDMA2RST_R {
        GPDMA2RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 12 - CRC block reset Set and reset by software.
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 17 - RAMCFG block reset Set and reset by software.
    #[inline(always)]
    pub fn ramcfgrst(&self) -> RAMCFGRST_R {
        RAMCFGRST_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1RSTR")
            .field("gpdma1rst", &self.gpdma1rst())
            .field("gpdma2rst", &self.gpdma2rst())
            .field("crcrst", &self.crcrst())
            .field("ramcfgrst", &self.ramcfgrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPDMA1 block reset Set and reset by software.
    #[inline(always)]
    pub fn gpdma1rst(&mut self) -> GPDMA1RST_W<'_, AHB1RSTRrs> {
        GPDMA1RST_W::new(self, 0)
    }
    ///Bit 1 - GPDMA2 block reset Set and reset by software.
    #[inline(always)]
    pub fn gpdma2rst(&mut self) -> GPDMA2RST_W<'_, AHB1RSTRrs> {
        GPDMA2RST_W::new(self, 1)
    }
    ///Bit 12 - CRC block reset Set and reset by software.
    #[inline(always)]
    pub fn crcrst(&mut self) -> CRCRST_W<'_, AHB1RSTRrs> {
        CRCRST_W::new(self, 12)
    }
    ///Bit 17 - RAMCFG block reset Set and reset by software.
    #[inline(always)]
    pub fn ramcfgrst(&mut self) -> RAMCFGRST_W<'_, AHB1RSTRrs> {
        RAMCFGRST_W::new(self, 17)
    }
}
/**RCC AHB1 reset register

You can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RCC:AHB1RSTR)*/
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
