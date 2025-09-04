///Register `APB1HRSTR` reader
pub type R = crate::R<APB1HRSTRrs>;
///Register `APB1HRSTR` writer
pub type W = crate::W<APB1HRSTRrs>;
/**clock recovery system reset Set and reset by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSRST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<CRSRST> for bool {
    #[inline(always)]
    fn from(variant: CRSRST) -> Self {
        variant as u8 != 0
    }
}
///Field `CRSRST` reader - clock recovery system reset Set and reset by software.
pub type CRSRST_R = crate::BitReader<CRSRST>;
impl CRSRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CRSRST> {
        match self.bits {
            true => Some(CRSRST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CRSRST::Reset
    }
}
///Field `CRSRST` writer - clock recovery system reset Set and reset by software.
pub type CRSRST_W<'a, REG> = crate::BitWriter<'a, REG, CRSRST>;
impl<'a, REG> CRSRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(CRSRST::Reset)
    }
}
///Field `MDIOSRST` reader - MDIOS block reset Set and reset by software.
pub use CRSRST_R as MDIOSRST_R;
///Field `FDCANRST` reader - FDCAN block reset Set and reset by software.
pub use CRSRST_R as FDCANRST_R;
///Field `UCPDRST` reader - UCPD block reset Set and reset by software.
pub use CRSRST_R as UCPDRST_R;
///Field `MDIOSRST` writer - MDIOS block reset Set and reset by software.
pub use CRSRST_W as MDIOSRST_W;
///Field `FDCANRST` writer - FDCAN block reset Set and reset by software.
pub use CRSRST_W as FDCANRST_W;
///Field `UCPDRST` writer - UCPD block reset Set and reset by software.
pub use CRSRST_W as UCPDRST_W;
impl R {
    ///Bit 1 - clock recovery system reset Set and reset by software.
    #[inline(always)]
    pub fn crsrst(&self) -> CRSRST_R {
        CRSRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - MDIOS block reset Set and reset by software.
    #[inline(always)]
    pub fn mdiosrst(&self) -> MDIOSRST_R {
        MDIOSRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - FDCAN block reset Set and reset by software.
    #[inline(always)]
    pub fn fdcanrst(&self) -> FDCANRST_R {
        FDCANRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 27 - UCPD block reset Set and reset by software.
    #[inline(always)]
    pub fn ucpdrst(&self) -> UCPDRST_R {
        UCPDRST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1HRSTR")
            .field("crsrst", &self.crsrst())
            .field("mdiosrst", &self.mdiosrst())
            .field("fdcanrst", &self.fdcanrst())
            .field("ucpdrst", &self.ucpdrst())
            .finish()
    }
}
impl W {
    ///Bit 1 - clock recovery system reset Set and reset by software.
    #[inline(always)]
    pub fn crsrst(&mut self) -> CRSRST_W<APB1HRSTRrs> {
        CRSRST_W::new(self, 1)
    }
    ///Bit 5 - MDIOS block reset Set and reset by software.
    #[inline(always)]
    pub fn mdiosrst(&mut self) -> MDIOSRST_W<APB1HRSTRrs> {
        MDIOSRST_W::new(self, 5)
    }
    ///Bit 8 - FDCAN block reset Set and reset by software.
    #[inline(always)]
    pub fn fdcanrst(&mut self) -> FDCANRST_W<APB1HRSTRrs> {
        FDCANRST_W::new(self, 8)
    }
    ///Bit 27 - UCPD block reset Set and reset by software.
    #[inline(always)]
    pub fn ucpdrst(&mut self) -> UCPDRST_W<APB1HRSTRrs> {
        UCPDRST_W::new(self, 27)
    }
}
/**RCC APB1 peripheral reset register 2

You can [`read`](crate::Reg::read) this register and get [`apb1hrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RCC:APB1HRSTR)*/
pub struct APB1HRSTRrs;
impl crate::RegisterSpec for APB1HRSTRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1hrstr::R`](R) reader structure
impl crate::Readable for APB1HRSTRrs {}
///`write(|w| ..)` method takes [`apb1hrstr::W`](W) writer structure
impl crate::Writable for APB1HRSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1HRSTR to value 0
impl crate::Resettable for APB1HRSTRrs {}
