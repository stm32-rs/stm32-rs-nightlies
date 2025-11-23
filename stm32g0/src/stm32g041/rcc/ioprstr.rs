///Register `IOPRSTR` reader
pub type R = crate::R<IOPRSTRrs>;
///Register `IOPRSTR` writer
pub type W = crate::W<IOPRSTRrs>;
/**I/O port A reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOPARST {
    ///1: Reset peripheral
    Reset = 1,
}
impl From<IOPARST> for bool {
    #[inline(always)]
    fn from(variant: IOPARST) -> Self {
        variant as u8 != 0
    }
}
///Field `IOPARST` reader - I/O port A reset
pub type IOPARST_R = crate::BitReader<IOPARST>;
impl IOPARST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<IOPARST> {
        match self.bits {
            true => Some(IOPARST::Reset),
            _ => None,
        }
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == IOPARST::Reset
    }
}
///Field `IOPARST` writer - I/O port A reset
pub type IOPARST_W<'a, REG> = crate::BitWriter<'a, REG, IOPARST>;
impl<'a, REG> IOPARST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(IOPARST::Reset)
    }
}
///Field `IOPBRST` reader - I/O port B reset
pub use IOPARST_R as IOPBRST_R;
///Field `IOPCRST` reader - I/O port C reset
pub use IOPARST_R as IOPCRST_R;
///Field `IOPDRST` reader - I/O port D reset
pub use IOPARST_R as IOPDRST_R;
///Field `IOPFRST` reader - I/O port F reset
pub use IOPARST_R as IOPFRST_R;
///Field `IOPBRST` writer - I/O port B reset
pub use IOPARST_W as IOPBRST_W;
///Field `IOPCRST` writer - I/O port C reset
pub use IOPARST_W as IOPCRST_W;
///Field `IOPDRST` writer - I/O port D reset
pub use IOPARST_W as IOPDRST_W;
///Field `IOPFRST` writer - I/O port F reset
pub use IOPARST_W as IOPFRST_W;
impl R {
    ///Bit 0 - I/O port A reset
    #[inline(always)]
    pub fn ioparst(&self) -> IOPARST_R {
        IOPARST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I/O port B reset
    #[inline(always)]
    pub fn iopbrst(&self) -> IOPBRST_R {
        IOPBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - I/O port C reset
    #[inline(always)]
    pub fn iopcrst(&self) -> IOPCRST_R {
        IOPCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I/O port D reset
    #[inline(always)]
    pub fn iopdrst(&self) -> IOPDRST_R {
        IOPDRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - I/O port F reset
    #[inline(always)]
    pub fn iopfrst(&self) -> IOPFRST_R {
        IOPFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOPRSTR")
            .field("ioparst", &self.ioparst())
            .field("iopbrst", &self.iopbrst())
            .field("iopcrst", &self.iopcrst())
            .field("iopdrst", &self.iopdrst())
            .field("iopfrst", &self.iopfrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - I/O port A reset
    #[inline(always)]
    pub fn ioparst(&mut self) -> IOPARST_W<'_, IOPRSTRrs> {
        IOPARST_W::new(self, 0)
    }
    ///Bit 1 - I/O port B reset
    #[inline(always)]
    pub fn iopbrst(&mut self) -> IOPBRST_W<'_, IOPRSTRrs> {
        IOPBRST_W::new(self, 1)
    }
    ///Bit 2 - I/O port C reset
    #[inline(always)]
    pub fn iopcrst(&mut self) -> IOPCRST_W<'_, IOPRSTRrs> {
        IOPCRST_W::new(self, 2)
    }
    ///Bit 3 - I/O port D reset
    #[inline(always)]
    pub fn iopdrst(&mut self) -> IOPDRST_W<'_, IOPRSTRrs> {
        IOPDRST_W::new(self, 3)
    }
    ///Bit 5 - I/O port F reset
    #[inline(always)]
    pub fn iopfrst(&mut self) -> IOPFRST_W<'_, IOPRSTRrs> {
        IOPFRST_W::new(self, 5)
    }
}
/**GPIO reset register

You can [`read`](crate::Reg::read) this register and get [`ioprstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioprstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G041.html#RCC:IOPRSTR)*/
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
