///Register `APB1HRSTR` reader
pub type R = crate::R<APB1HRSTRrs>;
///Register `APB1HRSTR` writer
pub type W = crate::W<APB1HRSTRrs>;
/**Clock Recovery System reset

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
///Field `CRSRST` reader - Clock Recovery System reset
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
///Field `CRSRST` writer - Clock Recovery System reset
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
///Field `SWPRST` reader - SWPMI block reset
pub use CRSRST_R as SWPRST_R;
///Field `OPAMPRST` reader - OPAMP block reset
pub use CRSRST_R as OPAMPRST_R;
///Field `MDIOSRST` reader - MDIOS block reset
pub use CRSRST_R as MDIOSRST_R;
///Field `FDCANRST` reader - FDCAN block reset
pub use CRSRST_R as FDCANRST_R;
///Field `SWPRST` writer - SWPMI block reset
pub use CRSRST_W as SWPRST_W;
///Field `OPAMPRST` writer - OPAMP block reset
pub use CRSRST_W as OPAMPRST_W;
///Field `MDIOSRST` writer - MDIOS block reset
pub use CRSRST_W as MDIOSRST_W;
///Field `FDCANRST` writer - FDCAN block reset
pub use CRSRST_W as FDCANRST_W;
impl R {
    ///Bit 1 - Clock Recovery System reset
    #[inline(always)]
    pub fn crsrst(&self) -> CRSRST_R {
        CRSRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SWPMI block reset
    #[inline(always)]
    pub fn swprst(&self) -> SWPRST_R {
        SWPRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - OPAMP block reset
    #[inline(always)]
    pub fn opamprst(&self) -> OPAMPRST_R {
        OPAMPRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MDIOS block reset
    #[inline(always)]
    pub fn mdiosrst(&self) -> MDIOSRST_R {
        MDIOSRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - FDCAN block reset
    #[inline(always)]
    pub fn fdcanrst(&self) -> FDCANRST_R {
        FDCANRST_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1HRSTR")
            .field("crsrst", &self.crsrst())
            .field("swprst", &self.swprst())
            .field("opamprst", &self.opamprst())
            .field("mdiosrst", &self.mdiosrst())
            .field("fdcanrst", &self.fdcanrst())
            .finish()
    }
}
impl W {
    ///Bit 1 - Clock Recovery System reset
    #[inline(always)]
    pub fn crsrst(&mut self) -> CRSRST_W<'_, APB1HRSTRrs> {
        CRSRST_W::new(self, 1)
    }
    ///Bit 2 - SWPMI block reset
    #[inline(always)]
    pub fn swprst(&mut self) -> SWPRST_W<'_, APB1HRSTRrs> {
        SWPRST_W::new(self, 2)
    }
    ///Bit 4 - OPAMP block reset
    #[inline(always)]
    pub fn opamprst(&mut self) -> OPAMPRST_W<'_, APB1HRSTRrs> {
        OPAMPRST_W::new(self, 4)
    }
    ///Bit 5 - MDIOS block reset
    #[inline(always)]
    pub fn mdiosrst(&mut self) -> MDIOSRST_W<'_, APB1HRSTRrs> {
        MDIOSRST_W::new(self, 5)
    }
    ///Bit 8 - FDCAN block reset
    #[inline(always)]
    pub fn fdcanrst(&mut self) -> FDCANRST_W<'_, APB1HRSTRrs> {
        FDCANRST_W::new(self, 8)
    }
}
/**RCC APB1 Peripheral Reset Register

You can [`read`](crate::Reg::read) this register and get [`apb1hrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#RCC:APB1HRSTR)*/
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
