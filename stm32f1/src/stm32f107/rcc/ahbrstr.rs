///Register `AHBRSTR` reader
pub type R = crate::R<AHBRSTRrs>;
///Register `AHBRSTR` writer
pub type W = crate::W<AHBRSTRrs>;
/**USB OTG FS reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTGFSRST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<OTGFSRST> for bool {
    #[inline(always)]
    fn from(variant: OTGFSRST) -> Self {
        variant as u8 != 0
    }
}
///Field `OTGFSRST` reader - USB OTG FS reset
pub type OTGFSRST_R = crate::BitReader<OTGFSRST>;
impl OTGFSRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<OTGFSRST> {
        match self.bits {
            true => Some(OTGFSRST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OTGFSRST::Reset
    }
}
///Field `OTGFSRST` writer - USB OTG FS reset
pub type OTGFSRST_W<'a, REG> = crate::BitWriter<'a, REG, OTGFSRST>;
impl<'a, REG> OTGFSRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(OTGFSRST::Reset)
    }
}
///Field `ETHMACRST` reader - Ethernet MAC reset
pub use OTGFSRST_R as ETHMACRST_R;
///Field `ETHMACRST` writer - Ethernet MAC reset
pub use OTGFSRST_W as ETHMACRST_W;
impl R {
    ///Bit 12 - USB OTG FS reset
    #[inline(always)]
    pub fn otgfsrst(&self) -> OTGFSRST_R {
        OTGFSRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - Ethernet MAC reset
    #[inline(always)]
    pub fn ethmacrst(&self) -> ETHMACRST_R {
        ETHMACRST_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBRSTR")
            .field("otgfsrst", &self.otgfsrst())
            .field("ethmacrst", &self.ethmacrst())
            .finish()
    }
}
impl W {
    ///Bit 12 - USB OTG FS reset
    #[inline(always)]
    pub fn otgfsrst(&mut self) -> OTGFSRST_W<'_, AHBRSTRrs> {
        OTGFSRST_W::new(self, 12)
    }
    ///Bit 14 - Ethernet MAC reset
    #[inline(always)]
    pub fn ethmacrst(&mut self) -> ETHMACRST_W<'_, AHBRSTRrs> {
        ETHMACRST_W::new(self, 14)
    }
}
/**AHB peripheral clock reset register (RCC_AHBRSTR)

You can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#RCC:AHBRSTR)*/
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
