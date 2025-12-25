///Register `AHB2RSTR` reader
pub type R = crate::R<AHB2RSTRrs>;
///Register `AHB2RSTR` writer
pub type W = crate::W<AHB2RSTRrs>;
/**CRYP module reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRYPRST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<CRYPRST> for bool {
    #[inline(always)]
    fn from(variant: CRYPRST) -> Self {
        variant as u8 != 0
    }
}
///Field `CRYPRST` reader - CRYP module reset
pub type CRYPRST_R = crate::BitReader<CRYPRST>;
impl CRYPRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CRYPRST> {
        match self.bits {
            true => Some(CRYPRST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CRYPRST::Reset
    }
}
///Field `CRYPRST` writer - CRYP module reset
pub type CRYPRST_W<'a, REG> = crate::BitWriter<'a, REG, CRYPRST>;
impl<'a, REG> CRYPRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(CRYPRST::Reset)
    }
}
///Field `RNGRST` reader - RNGRST
pub use CRYPRST_R as RNGRST_R;
///Field `OTGFSRST` reader - USB OTG FS module reset
pub use CRYPRST_R as OTGFSRST_R;
///Field `RNGRST` writer - RNGRST
pub use CRYPRST_W as RNGRST_W;
///Field `OTGFSRST` writer - USB OTG FS module reset
pub use CRYPRST_W as OTGFSRST_W;
impl R {
    ///Bit 4 - CRYP module reset
    #[inline(always)]
    pub fn cryprst(&self) -> CRYPRST_R {
        CRYPRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - RNGRST
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - USB OTG FS module reset
    #[inline(always)]
    pub fn otgfsrst(&self) -> OTGFSRST_R {
        OTGFSRST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2RSTR")
            .field("cryprst", &self.cryprst())
            .field("otgfsrst", &self.otgfsrst())
            .field("rngrst", &self.rngrst())
            .finish()
    }
}
impl W {
    ///Bit 4 - CRYP module reset
    #[inline(always)]
    pub fn cryprst(&mut self) -> CRYPRST_W<'_, AHB2RSTRrs> {
        CRYPRST_W::new(self, 4)
    }
    ///Bit 6 - RNGRST
    #[inline(always)]
    pub fn rngrst(&mut self) -> RNGRST_W<'_, AHB2RSTRrs> {
        RNGRST_W::new(self, 6)
    }
    ///Bit 7 - USB OTG FS module reset
    #[inline(always)]
    pub fn otgfsrst(&mut self) -> OTGFSRST_W<'_, AHB2RSTRrs> {
        OTGFSRST_W::new(self, 7)
    }
}
/**AHB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F413.html#RCC:AHB2RSTR)*/
pub struct AHB2RSTRrs;
impl crate::RegisterSpec for AHB2RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2rstr::R`](R) reader structure
impl crate::Readable for AHB2RSTRrs {}
///`write(|w| ..)` method takes [`ahb2rstr::W`](W) writer structure
impl crate::Writable for AHB2RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2RSTR to value 0
impl crate::Resettable for AHB2RSTRrs {}
