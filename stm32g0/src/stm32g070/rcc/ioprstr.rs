///Register `IOPRSTR` reader
pub type R = crate::R<IOPRSTRrs>;
///Register `IOPRSTR` writer
pub type W = crate::W<IOPRSTRrs>;
/**GPIOARST

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOARST {
    ///1: Reset peripheral
    Reset = 1,
}
impl From<GPIOARST> for bool {
    #[inline(always)]
    fn from(variant: GPIOARST) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOARST` reader - GPIOARST
pub type GPIOARST_R = crate::BitReader<GPIOARST>;
impl GPIOARST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<GPIOARST> {
        match self.bits {
            true => Some(GPIOARST::Reset),
            _ => None,
        }
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == GPIOARST::Reset
    }
}
///Field `GPIOARST` writer - GPIOARST
pub type GPIOARST_W<'a, REG> = crate::BitWriter<'a, REG, GPIOARST>;
impl<'a, REG> GPIOARST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOARST::Reset)
    }
}
///Field `GPIOBRST` reader - GPIOBRST
pub use GPIOARST_R as GPIOBRST_R;
///Field `GPIOCRST` reader - GPIOCRST
pub use GPIOARST_R as GPIOCRST_R;
///Field `GPIODRST` reader - GPIODRST
pub use GPIOARST_R as GPIODRST_R;
///Field `GPIOERST` reader - GPIOERST
pub use GPIOARST_R as GPIOERST_R;
///Field `GPIOFRST` reader - GPIOFRST
pub use GPIOARST_R as GPIOFRST_R;
///Field `GPIOBRST` writer - GPIOBRST
pub use GPIOARST_W as GPIOBRST_W;
///Field `GPIOCRST` writer - GPIOCRST
pub use GPIOARST_W as GPIOCRST_W;
///Field `GPIODRST` writer - GPIODRST
pub use GPIOARST_W as GPIODRST_W;
///Field `GPIOERST` writer - GPIOERST
pub use GPIOARST_W as GPIOERST_W;
///Field `GPIOFRST` writer - GPIOFRST
pub use GPIOARST_W as GPIOFRST_W;
impl R {
    ///Bit 0 - GPIOARST
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPIOBRST
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GPIOCRST
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPIODRST
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GPIOERST
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GPIOFRST
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOPRSTR")
            .field("gpioarst", &self.gpioarst())
            .field("gpiobrst", &self.gpiobrst())
            .field("gpiocrst", &self.gpiocrst())
            .field("gpiodrst", &self.gpiodrst())
            .field("gpioerst", &self.gpioerst())
            .field("gpiofrst", &self.gpiofrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPIOARST
    #[inline(always)]
    pub fn gpioarst(&mut self) -> GPIOARST_W<'_, IOPRSTRrs> {
        GPIOARST_W::new(self, 0)
    }
    ///Bit 1 - GPIOBRST
    #[inline(always)]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<'_, IOPRSTRrs> {
        GPIOBRST_W::new(self, 1)
    }
    ///Bit 2 - GPIOCRST
    #[inline(always)]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<'_, IOPRSTRrs> {
        GPIOCRST_W::new(self, 2)
    }
    ///Bit 3 - GPIODRST
    #[inline(always)]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<'_, IOPRSTRrs> {
        GPIODRST_W::new(self, 3)
    }
    ///Bit 4 - GPIOERST
    #[inline(always)]
    pub fn gpioerst(&mut self) -> GPIOERST_W<'_, IOPRSTRrs> {
        GPIOERST_W::new(self, 4)
    }
    ///Bit 5 - GPIOFRST
    #[inline(always)]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<'_, IOPRSTRrs> {
        GPIOFRST_W::new(self, 5)
    }
}
/**I/O port reset register

You can [`read`](crate::Reg::read) this register and get [`ioprstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioprstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#RCC:IOPRSTR)*/
pub struct IOPRSTRrs;
impl crate::RegisterSpec for IOPRSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ioprstr::R`](R) reader structure
impl crate::Readable for IOPRSTRrs {}
///`write(|w| ..)` method takes [`ioprstr::W`](W) writer structure
impl crate::Writable for IOPRSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IOPRSTR to value 0
impl crate::Resettable for IOPRSTRrs {}
