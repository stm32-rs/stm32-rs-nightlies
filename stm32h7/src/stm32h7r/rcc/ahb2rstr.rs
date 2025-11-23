///Register `AHB2RSTR` reader
pub type R = crate::R<AHB2RSTRrs>;
///Register `AHB2RSTR` writer
pub type W = crate::W<AHB2RSTRrs>;
/**PSSI block reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSSIRST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<PSSIRST> for bool {
    #[inline(always)]
    fn from(variant: PSSIRST) -> Self {
        variant as u8 != 0
    }
}
///Field `PSSIRST` reader - PSSI block reset
pub type PSSIRST_R = crate::BitReader<PSSIRST>;
impl PSSIRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PSSIRST> {
        match self.bits {
            true => Some(PSSIRST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PSSIRST::Reset
    }
}
///Field `PSSIRST` writer - PSSI block reset
pub type PSSIRST_W<'a, REG> = crate::BitWriter<'a, REG, PSSIRST>;
impl<'a, REG> PSSIRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(PSSIRST::Reset)
    }
}
///Field `SDMMC2RST` reader - SDMMC2 and SDMMC2 delay blocks reset
pub use PSSIRST_R as SDMMC2RST_R;
///Field `CORDICRST` reader - CORDIC reset
pub use PSSIRST_R as CORDICRST_R;
///Field `SDMMC2RST` writer - SDMMC2 and SDMMC2 delay blocks reset
pub use PSSIRST_W as SDMMC2RST_W;
///Field `CORDICRST` writer - CORDIC reset
pub use PSSIRST_W as CORDICRST_W;
impl R {
    ///Bit 1 - PSSI block reset
    #[inline(always)]
    pub fn pssirst(&self) -> PSSIRST_R {
        PSSIRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 9 - SDMMC2 and SDMMC2 delay blocks reset
    #[inline(always)]
    pub fn sdmmc2rst(&self) -> SDMMC2RST_R {
        SDMMC2RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 14 - CORDIC reset
    #[inline(always)]
    pub fn cordicrst(&self) -> CORDICRST_R {
        CORDICRST_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2RSTR")
            .field("pssirst", &self.pssirst())
            .field("sdmmc2rst", &self.sdmmc2rst())
            .field("cordicrst", &self.cordicrst())
            .finish()
    }
}
impl W {
    ///Bit 1 - PSSI block reset
    #[inline(always)]
    pub fn pssirst(&mut self) -> PSSIRST_W<'_, AHB2RSTRrs> {
        PSSIRST_W::new(self, 1)
    }
    ///Bit 9 - SDMMC2 and SDMMC2 delay blocks reset
    #[inline(always)]
    pub fn sdmmc2rst(&mut self) -> SDMMC2RST_W<'_, AHB2RSTRrs> {
        SDMMC2RST_W::new(self, 9)
    }
    ///Bit 14 - CORDIC reset
    #[inline(always)]
    pub fn cordicrst(&mut self) -> CORDICRST_W<'_, AHB2RSTRrs> {
        CORDICRST_W::new(self, 14)
    }
}
/**RCC AHB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:AHB2RSTR)*/
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
