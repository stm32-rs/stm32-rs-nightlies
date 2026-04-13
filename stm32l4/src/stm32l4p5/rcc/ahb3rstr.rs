///Register `AHB3RSTR` reader
pub type R = crate::R<AHB3RSTRrs>;
///Register `AHB3RSTR` writer
pub type W = crate::W<AHB3RSTRrs>;
/**Flexible memory controller reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMCRST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset FMC
    Reset = 1,
}
impl From<FMCRST> for bool {
    #[inline(always)]
    fn from(variant: FMCRST) -> Self {
        variant as u8 != 0
    }
}
///Field `FMCRST` reader - Flexible memory controller reset
pub type FMCRST_R = crate::BitReader<FMCRST>;
impl FMCRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FMCRST {
        match self.bits {
            false => FMCRST::NoEffect,
            true => FMCRST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == FMCRST::NoEffect
    }
    ///Reset FMC
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == FMCRST::Reset
    }
}
///Field `FMCRST` writer - Flexible memory controller reset
pub type FMCRST_W<'a, REG> = crate::BitWriter<'a, REG, FMCRST>;
impl<'a, REG> FMCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(FMCRST::NoEffect)
    }
    ///Reset FMC
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(FMCRST::Reset)
    }
}
/**OctoSPI1 memory interface reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSPI1RST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset OctoSPIx
    Reset = 1,
}
impl From<OSPI1RST> for bool {
    #[inline(always)]
    fn from(variant: OSPI1RST) -> Self {
        variant as u8 != 0
    }
}
///Field `OSPI1RST` reader - OctoSPI1 memory interface reset
pub type OSPI1RST_R = crate::BitReader<OSPI1RST>;
impl OSPI1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSPI1RST {
        match self.bits {
            false => OSPI1RST::NoEffect,
            true => OSPI1RST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == OSPI1RST::NoEffect
    }
    ///Reset OctoSPIx
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OSPI1RST::Reset
    }
}
///Field `OSPI1RST` writer - OctoSPI1 memory interface reset
pub type OSPI1RST_W<'a, REG> = crate::BitWriter<'a, REG, OSPI1RST>;
impl<'a, REG> OSPI1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(OSPI1RST::NoEffect)
    }
    ///Reset OctoSPIx
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(OSPI1RST::Reset)
    }
}
///Field `OSPI2RST` reader - OctOSPI2 memory interface reset
pub use OSPI1RST_R as OSPI2RST_R;
///Field `OSPI2RST` writer - OctOSPI2 memory interface reset
pub use OSPI1RST_W as OSPI2RST_W;
impl R {
    ///Bit 0 - Flexible memory controller reset
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - OctoSPI1 memory interface reset
    #[inline(always)]
    pub fn ospi1rst(&self) -> OSPI1RST_R {
        OSPI1RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - OctOSPI2 memory interface reset
    #[inline(always)]
    pub fn ospi2rst(&self) -> OSPI2RST_R {
        OSPI2RST_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3RSTR")
            .field("fmcrst", &self.fmcrst())
            .field("ospi1rst", &self.ospi1rst())
            .field("ospi2rst", &self.ospi2rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - Flexible memory controller reset
    #[inline(always)]
    pub fn fmcrst(&mut self) -> FMCRST_W<'_, AHB3RSTRrs> {
        FMCRST_W::new(self, 0)
    }
    ///Bit 8 - OctoSPI1 memory interface reset
    #[inline(always)]
    pub fn ospi1rst(&mut self) -> OSPI1RST_W<'_, AHB3RSTRrs> {
        OSPI1RST_W::new(self, 8)
    }
    ///Bit 9 - OctOSPI2 memory interface reset
    #[inline(always)]
    pub fn ospi2rst(&mut self) -> OSPI2RST_W<'_, AHB3RSTRrs> {
        OSPI2RST_W::new(self, 9)
    }
}
/**AHB3 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb3rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#RCC:AHB3RSTR)*/
pub struct AHB3RSTRrs;
impl crate::RegisterSpec for AHB3RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb3rstr::R`](R) reader structure
impl crate::Readable for AHB3RSTRrs {}
///`write(|w| ..)` method takes [`ahb3rstr::W`](W) writer structure
impl crate::Writable for AHB3RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3RSTR to value 0
impl crate::Resettable for AHB3RSTRrs {}
