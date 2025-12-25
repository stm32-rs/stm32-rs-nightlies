///Register `AHBRSTR` reader
pub type R = crate::R<AHBRSTRrs>;
///Register `AHBRSTR` writer
pub type W = crate::W<AHBRSTRrs>;
/**I/O port A reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOPARST {
    ///1: Reset the selected module
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
    ///Reset the selected module
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
    ///Reset the selected module
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
///Field `IOPERST` reader - I/O port E reset
pub use IOPARST_R as IOPERST_R;
///Field `IOPFRST` reader - I/O port F reset
pub use IOPARST_R as IOPFRST_R;
///Field `TSCRST` reader - Touch sensing controller reset
pub use IOPARST_R as TSCRST_R;
///Field `IOPBRST` writer - I/O port B reset
pub use IOPARST_W as IOPBRST_W;
///Field `IOPCRST` writer - I/O port C reset
pub use IOPARST_W as IOPCRST_W;
///Field `IOPDRST` writer - I/O port D reset
pub use IOPARST_W as IOPDRST_W;
///Field `IOPERST` writer - I/O port E reset
pub use IOPARST_W as IOPERST_W;
///Field `IOPFRST` writer - I/O port F reset
pub use IOPARST_W as IOPFRST_W;
///Field `TSCRST` writer - Touch sensing controller reset
pub use IOPARST_W as TSCRST_W;
impl R {
    ///Bit 17 - I/O port A reset
    #[inline(always)]
    pub fn ioparst(&self) -> IOPARST_R {
        IOPARST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - I/O port B reset
    #[inline(always)]
    pub fn iopbrst(&self) -> IOPBRST_R {
        IOPBRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - I/O port C reset
    #[inline(always)]
    pub fn iopcrst(&self) -> IOPCRST_R {
        IOPCRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - I/O port D reset
    #[inline(always)]
    pub fn iopdrst(&self) -> IOPDRST_R {
        IOPDRST_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I/O port E reset
    #[inline(always)]
    pub fn ioperst(&self) -> IOPERST_R {
        IOPERST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I/O port F reset
    #[inline(always)]
    pub fn iopfrst(&self) -> IOPFRST_R {
        IOPFRST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - Touch sensing controller reset
    #[inline(always)]
    pub fn tscrst(&self) -> TSCRST_R {
        TSCRST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBRSTR")
            .field("ioparst", &self.ioparst())
            .field("iopbrst", &self.iopbrst())
            .field("iopcrst", &self.iopcrst())
            .field("iopdrst", &self.iopdrst())
            .field("iopfrst", &self.iopfrst())
            .field("tscrst", &self.tscrst())
            .field("ioperst", &self.ioperst())
            .finish()
    }
}
impl W {
    ///Bit 17 - I/O port A reset
    #[inline(always)]
    pub fn ioparst(&mut self) -> IOPARST_W<'_, AHBRSTRrs> {
        IOPARST_W::new(self, 17)
    }
    ///Bit 18 - I/O port B reset
    #[inline(always)]
    pub fn iopbrst(&mut self) -> IOPBRST_W<'_, AHBRSTRrs> {
        IOPBRST_W::new(self, 18)
    }
    ///Bit 19 - I/O port C reset
    #[inline(always)]
    pub fn iopcrst(&mut self) -> IOPCRST_W<'_, AHBRSTRrs> {
        IOPCRST_W::new(self, 19)
    }
    ///Bit 20 - I/O port D reset
    #[inline(always)]
    pub fn iopdrst(&mut self) -> IOPDRST_W<'_, AHBRSTRrs> {
        IOPDRST_W::new(self, 20)
    }
    ///Bit 21 - I/O port E reset
    #[inline(always)]
    pub fn ioperst(&mut self) -> IOPERST_W<'_, AHBRSTRrs> {
        IOPERST_W::new(self, 21)
    }
    ///Bit 22 - I/O port F reset
    #[inline(always)]
    pub fn iopfrst(&mut self) -> IOPFRST_W<'_, AHBRSTRrs> {
        IOPFRST_W::new(self, 22)
    }
    ///Bit 24 - Touch sensing controller reset
    #[inline(always)]
    pub fn tscrst(&mut self) -> TSCRST_W<'_, AHBRSTRrs> {
        TSCRST_W::new(self, 24)
    }
}
/**AHB peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x8.html#RCC:AHBRSTR)*/
pub struct AHBRSTRrs;
impl crate::RegisterSpec for AHBRSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahbrstr::R`](R) reader structure
impl crate::Readable for AHBRSTRrs {}
///`write(|w| ..)` method takes [`ahbrstr::W`](W) writer structure
impl crate::Writable for AHBRSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHBRSTR to value 0
impl crate::Resettable for AHBRSTRrs {}
