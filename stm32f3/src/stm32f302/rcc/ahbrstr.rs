///Register `AHBRSTR` reader
pub type R = crate::R<AHBRSTRrs>;
///Register `AHBRSTR` writer
pub type W = crate::W<AHBRSTRrs>;
/**FMC reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMCRST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<FMCRST> for bool {
    #[inline(always)]
    fn from(variant: FMCRST) -> Self {
        variant as u8 != 0
    }
}
///Field `FMCRST` reader - FMC reset
pub type FMCRST_R = crate::BitReader<FMCRST>;
impl FMCRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<FMCRST> {
        match self.bits {
            true => Some(FMCRST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == FMCRST::Reset
    }
}
///Field `FMCRST` writer - FMC reset
pub type FMCRST_W<'a, REG> = crate::BitWriter<'a, REG, FMCRST>;
impl<'a, REG> FMCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(FMCRST::Reset)
    }
}
///Field `IOPHRST` reader - IO port H reset
pub use FMCRST_R as IOPHRST_R;
///Field `IOPARST` reader - I/O port A reset
pub use FMCRST_R as IOPARST_R;
///Field `IOPBRST` reader - I/O port B reset
pub use FMCRST_R as IOPBRST_R;
///Field `IOPCRST` reader - I/O port C reset
pub use FMCRST_R as IOPCRST_R;
///Field `IOPDRST` reader - I/O port D reset
pub use FMCRST_R as IOPDRST_R;
///Field `IOPERST` reader - I/O port E reset
pub use FMCRST_R as IOPERST_R;
///Field `IOPFRST` reader - I/O port F reset
pub use FMCRST_R as IOPFRST_R;
///Field `IOPGRST` reader - IO port G reset
pub use FMCRST_R as IOPGRST_R;
///Field `TSCRST` reader - Touch sensing controller reset
pub use FMCRST_R as TSCRST_R;
///Field `ADC12RST` reader - ADC1 and ADC2 reset
pub use FMCRST_R as ADC12RST_R;
///Field `IOPHRST` writer - IO port H reset
pub use FMCRST_W as IOPHRST_W;
///Field `IOPARST` writer - I/O port A reset
pub use FMCRST_W as IOPARST_W;
///Field `IOPBRST` writer - I/O port B reset
pub use FMCRST_W as IOPBRST_W;
///Field `IOPCRST` writer - I/O port C reset
pub use FMCRST_W as IOPCRST_W;
///Field `IOPDRST` writer - I/O port D reset
pub use FMCRST_W as IOPDRST_W;
///Field `IOPERST` writer - I/O port E reset
pub use FMCRST_W as IOPERST_W;
///Field `IOPFRST` writer - I/O port F reset
pub use FMCRST_W as IOPFRST_W;
///Field `IOPGRST` writer - IO port G reset
pub use FMCRST_W as IOPGRST_W;
///Field `TSCRST` writer - Touch sensing controller reset
pub use FMCRST_W as TSCRST_W;
///Field `ADC12RST` writer - ADC1 and ADC2 reset
pub use FMCRST_W as ADC12RST_W;
impl R {
    ///Bit 5 - FMC reset
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 16 - IO port H reset
    #[inline(always)]
    pub fn iophrst(&self) -> IOPHRST_R {
        IOPHRST_R::new(((self.bits >> 16) & 1) != 0)
    }
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
    ///Bit 23 - IO port G reset
    #[inline(always)]
    pub fn iopgrst(&self) -> IOPGRST_R {
        IOPGRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Touch sensing controller reset
    #[inline(always)]
    pub fn tscrst(&self) -> TSCRST_R {
        TSCRST_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - ADC1 and ADC2 reset
    #[inline(always)]
    pub fn adc12rst(&self) -> ADC12RST_R {
        ADC12RST_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBRSTR")
            .field("fmcrst", &self.fmcrst())
            .field("ioparst", &self.ioparst())
            .field("iopbrst", &self.iopbrst())
            .field("iopcrst", &self.iopcrst())
            .field("iopdrst", &self.iopdrst())
            .field("ioperst", &self.ioperst())
            .field("iopfrst", &self.iopfrst())
            .field("tscrst", &self.tscrst())
            .field("adc12rst", &self.adc12rst())
            .field("iophrst", &self.iophrst())
            .field("iopgrst", &self.iopgrst())
            .finish()
    }
}
impl W {
    ///Bit 5 - FMC reset
    #[inline(always)]
    pub fn fmcrst(&mut self) -> FMCRST_W<'_, AHBRSTRrs> {
        FMCRST_W::new(self, 5)
    }
    ///Bit 16 - IO port H reset
    #[inline(always)]
    pub fn iophrst(&mut self) -> IOPHRST_W<'_, AHBRSTRrs> {
        IOPHRST_W::new(self, 16)
    }
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
    ///Bit 23 - IO port G reset
    #[inline(always)]
    pub fn iopgrst(&mut self) -> IOPGRST_W<'_, AHBRSTRrs> {
        IOPGRST_W::new(self, 23)
    }
    ///Bit 24 - Touch sensing controller reset
    #[inline(always)]
    pub fn tscrst(&mut self) -> TSCRST_W<'_, AHBRSTRrs> {
        TSCRST_W::new(self, 24)
    }
    ///Bit 28 - ADC1 and ADC2 reset
    #[inline(always)]
    pub fn adc12rst(&mut self) -> ADC12RST_W<'_, AHBRSTRrs> {
        ADC12RST_W::new(self, 28)
    }
}
/**AHB peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F302.html#RCC:AHBRSTR)*/
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
