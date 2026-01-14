///Register `AHB4RSTR` reader
pub type R = crate::R<AHB4RSTRrs>;
///Register `AHB4RSTR` writer
pub type W = crate::W<AHB4RSTRrs>;
/**SDMMC1 and SDMMC1 delay blocks reset Set and reset by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMC1RST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<SDMMC1RST> for bool {
    #[inline(always)]
    fn from(variant: SDMMC1RST) -> Self {
        variant as u8 != 0
    }
}
///Field `SDMMC1RST` reader - SDMMC1 and SDMMC1 delay blocks reset Set and reset by software.
pub type SDMMC1RST_R = crate::BitReader<SDMMC1RST>;
impl SDMMC1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SDMMC1RST> {
        match self.bits {
            true => Some(SDMMC1RST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SDMMC1RST::Reset
    }
}
///Field `SDMMC1RST` writer - SDMMC1 and SDMMC1 delay blocks reset Set and reset by software.
pub type SDMMC1RST_W<'a, REG> = crate::BitWriter<'a, REG, SDMMC1RST>;
impl<'a, REG> SDMMC1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC1RST::Reset)
    }
}
///Field `FMCRST` reader - FMC block reset Set and reset by software.
pub use SDMMC1RST_R as FMCRST_R;
///Field `OCTOSPI1RST` reader - OCTOSPI1 block reset Set and reset by software.
pub use SDMMC1RST_R as OCTOSPI1RST_R;
///Field `FMCRST` writer - FMC block reset Set and reset by software.
pub use SDMMC1RST_W as FMCRST_W;
///Field `OCTOSPI1RST` writer - OCTOSPI1 block reset Set and reset by software.
pub use SDMMC1RST_W as OCTOSPI1RST_W;
impl R {
    ///Bit 11 - SDMMC1 and SDMMC1 delay blocks reset Set and reset by software.
    #[inline(always)]
    pub fn sdmmc1rst(&self) -> SDMMC1RST_R {
        SDMMC1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - FMC block reset Set and reset by software.
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - OCTOSPI1 block reset Set and reset by software.
    #[inline(always)]
    pub fn octospi1rst(&self) -> OCTOSPI1RST_R {
        OCTOSPI1RST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB4RSTR")
            .field("sdmmc1rst", &self.sdmmc1rst())
            .field("fmcrst", &self.fmcrst())
            .field("octospi1rst", &self.octospi1rst())
            .finish()
    }
}
impl W {
    ///Bit 11 - SDMMC1 and SDMMC1 delay blocks reset Set and reset by software.
    #[inline(always)]
    pub fn sdmmc1rst(&mut self) -> SDMMC1RST_W<'_, AHB4RSTRrs> {
        SDMMC1RST_W::new(self, 11)
    }
    ///Bit 16 - FMC block reset Set and reset by software.
    #[inline(always)]
    pub fn fmcrst(&mut self) -> FMCRST_W<'_, AHB4RSTRrs> {
        FMCRST_W::new(self, 16)
    }
    ///Bit 20 - OCTOSPI1 block reset Set and reset by software.
    #[inline(always)]
    pub fn octospi1rst(&mut self) -> OCTOSPI1RST_W<'_, AHB4RSTRrs> {
        OCTOSPI1RST_W::new(self, 20)
    }
}
/**RCC AHB4 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb4rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#RCC:AHB4RSTR)*/
pub struct AHB4RSTRrs;
impl crate::RegisterSpec for AHB4RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb4rstr::R`](R) reader structure
impl crate::Readable for AHB4RSTRrs {}
///`write(|w| ..)` method takes [`ahb4rstr::W`](W) writer structure
impl crate::Writable for AHB4RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB4RSTR to value 0
impl crate::Resettable for AHB4RSTRrs {}
