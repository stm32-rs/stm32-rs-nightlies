///Register `APB3RSTR` reader
pub type R = crate::R<APB3RSTRrs>;
///Register `APB3RSTR` writer
pub type W = crate::W<APB3RSTRrs>;
/**SBS block reset Set and reset by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBSRST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<SBSRST> for bool {
    #[inline(always)]
    fn from(variant: SBSRST) -> Self {
        variant as u8 != 0
    }
}
///Field `SBSRST` reader - SBS block reset Set and reset by software.
pub type SBSRST_R = crate::BitReader<SBSRST>;
impl SBSRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SBSRST> {
        match self.bits {
            true => Some(SBSRST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SBSRST::Reset
    }
}
///Field `SBSRST` writer - SBS block reset Set and reset by software.
pub type SBSRST_W<'a, REG> = crate::BitWriter<'a, REG, SBSRST>;
impl<'a, REG> SBSRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SBSRST::Reset)
    }
}
///Field `LPUART1RST` reader - LPUART1 block reset Set and reset by software.
pub use SBSRST_R as LPUART1RST_R;
///Field `I3C2RST` reader - I3C2RST block reset Set and reset by software.
pub use SBSRST_R as I3C2RST_R;
///Field `LPTIM1RST` reader - LPTIM1 block reset Set and reset by software.
pub use SBSRST_R as LPTIM1RST_R;
///Field `VREFRST` reader - VREF block reset Set and reset by software.
pub use SBSRST_R as VREFRST_R;
///Field `LPUART1RST` writer - LPUART1 block reset Set and reset by software.
pub use SBSRST_W as LPUART1RST_W;
///Field `I3C2RST` writer - I3C2RST block reset Set and reset by software.
pub use SBSRST_W as I3C2RST_W;
///Field `LPTIM1RST` writer - LPTIM1 block reset Set and reset by software.
pub use SBSRST_W as LPTIM1RST_W;
///Field `VREFRST` writer - VREF block reset Set and reset by software.
pub use SBSRST_W as VREFRST_W;
impl R {
    ///Bit 1 - SBS block reset Set and reset by software.
    #[inline(always)]
    pub fn sbsrst(&self) -> SBSRST_R {
        SBSRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 6 - LPUART1 block reset Set and reset by software.
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - I3C2RST block reset Set and reset by software.
    #[inline(always)]
    pub fn i3c2rst(&self) -> I3C2RST_R {
        I3C2RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - LPTIM1 block reset Set and reset by software.
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 20 - VREF block reset Set and reset by software.
    #[inline(always)]
    pub fn vrefrst(&self) -> VREFRST_R {
        VREFRST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB3RSTR")
            .field("sbsrst", &self.sbsrst())
            .field("lpuart1rst", &self.lpuart1rst())
            .field("i3c2rst", &self.i3c2rst())
            .field("lptim1rst", &self.lptim1rst())
            .field("vrefrst", &self.vrefrst())
            .finish()
    }
}
impl W {
    ///Bit 1 - SBS block reset Set and reset by software.
    #[inline(always)]
    pub fn sbsrst(&mut self) -> SBSRST_W<'_, APB3RSTRrs> {
        SBSRST_W::new(self, 1)
    }
    ///Bit 6 - LPUART1 block reset Set and reset by software.
    #[inline(always)]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<'_, APB3RSTRrs> {
        LPUART1RST_W::new(self, 6)
    }
    ///Bit 9 - I3C2RST block reset Set and reset by software.
    #[inline(always)]
    pub fn i3c2rst(&mut self) -> I3C2RST_W<'_, APB3RSTRrs> {
        I3C2RST_W::new(self, 9)
    }
    ///Bit 11 - LPTIM1 block reset Set and reset by software.
    #[inline(always)]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<'_, APB3RSTRrs> {
        LPTIM1RST_W::new(self, 11)
    }
    ///Bit 20 - VREF block reset Set and reset by software.
    #[inline(always)]
    pub fn vrefrst(&mut self) -> VREFRST_W<'_, APB3RSTRrs> {
        VREFRST_W::new(self, 20)
    }
}
/**RCC APB3 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb3rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RCC:APB3RSTR)*/
pub struct APB3RSTRrs;
impl crate::RegisterSpec for APB3RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`apb3rstr::R`](R) reader structure
impl crate::Readable for APB3RSTRrs {}
///`write(|w| ..)` method takes [`apb3rstr::W`](W) writer structure
impl crate::Writable for APB3RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB3RSTR to value 0
impl crate::Resettable for APB3RSTRrs {}
