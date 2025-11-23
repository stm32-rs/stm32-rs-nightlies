///Register `AHB2RSTR` reader
pub type R = crate::R<AHB2RSTRrs>;
///Register `AHB2RSTR` writer
pub type W = crate::W<AHB2RSTRrs>;
/**digital camera interface block reset (DCMI or PSSI depending which IP is active) Set and reset by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMI_PSSIRST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<DCMI_PSSIRST> for bool {
    #[inline(always)]
    fn from(variant: DCMI_PSSIRST) -> Self {
        variant as u8 != 0
    }
}
///Field `DCMI_PSSIRST` reader - digital camera interface block reset (DCMI or PSSI depending which IP is active) Set and reset by software.
pub type DCMI_PSSIRST_R = crate::BitReader<DCMI_PSSIRST>;
impl DCMI_PSSIRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DCMI_PSSIRST> {
        match self.bits {
            true => Some(DCMI_PSSIRST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DCMI_PSSIRST::Reset
    }
}
///Field `DCMI_PSSIRST` writer - digital camera interface block reset (DCMI or PSSI depending which IP is active) Set and reset by software.
pub type DCMI_PSSIRST_W<'a, REG> = crate::BitWriter<'a, REG, DCMI_PSSIRST>;
impl<'a, REG> DCMI_PSSIRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DCMI_PSSIRST::Reset)
    }
}
///Field `HSEMRST` reader - HSEM block reset Set and reset by software.
pub use DCMI_PSSIRST_R as HSEMRST_R;
///Field `CRYPTRST` reader - cryptography block reset Set and reset by software.
pub use DCMI_PSSIRST_R as CRYPTRST_R;
///Field `HASHRST` reader - hash block reset Set and reset by software.
pub use DCMI_PSSIRST_R as HASHRST_R;
///Field `RNGRST` reader - random number generator block reset Set and reset by software.
pub use DCMI_PSSIRST_R as RNGRST_R;
///Field `SDMMC2RST` reader - SDMMC2 and SDMMC2 delay blocks reset Set and reset by software.
pub use DCMI_PSSIRST_R as SDMMC2RST_R;
///Field `BDMA1RST` reader - BDMA1 reset (DFSDM dedicated DMA) Set and reset by software.
pub use DCMI_PSSIRST_R as BDMA1RST_R;
///Field `HSEMRST` writer - HSEM block reset Set and reset by software.
pub use DCMI_PSSIRST_W as HSEMRST_W;
///Field `CRYPTRST` writer - cryptography block reset Set and reset by software.
pub use DCMI_PSSIRST_W as CRYPTRST_W;
///Field `HASHRST` writer - hash block reset Set and reset by software.
pub use DCMI_PSSIRST_W as HASHRST_W;
///Field `RNGRST` writer - random number generator block reset Set and reset by software.
pub use DCMI_PSSIRST_W as RNGRST_W;
///Field `SDMMC2RST` writer - SDMMC2 and SDMMC2 delay blocks reset Set and reset by software.
pub use DCMI_PSSIRST_W as SDMMC2RST_W;
///Field `BDMA1RST` writer - BDMA1 reset (DFSDM dedicated DMA) Set and reset by software.
pub use DCMI_PSSIRST_W as BDMA1RST_W;
impl R {
    ///Bit 0 - digital camera interface block reset (DCMI or PSSI depending which IP is active) Set and reset by software.
    #[inline(always)]
    pub fn dcmi_pssirst(&self) -> DCMI_PSSIRST_R {
        DCMI_PSSIRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - HSEM block reset Set and reset by software.
    #[inline(always)]
    pub fn hsemrst(&self) -> HSEMRST_R {
        HSEMRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - cryptography block reset Set and reset by software.
    #[inline(always)]
    pub fn cryptrst(&self) -> CRYPTRST_R {
        CRYPTRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - hash block reset Set and reset by software.
    #[inline(always)]
    pub fn hashrst(&self) -> HASHRST_R {
        HASHRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - random number generator block reset Set and reset by software.
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - SDMMC2 and SDMMC2 delay blocks reset Set and reset by software.
    #[inline(always)]
    pub fn sdmmc2rst(&self) -> SDMMC2RST_R {
        SDMMC2RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - BDMA1 reset (DFSDM dedicated DMA) Set and reset by software.
    #[inline(always)]
    pub fn bdma1rst(&self) -> BDMA1RST_R {
        BDMA1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2RSTR")
            .field("dcmi_pssirst", &self.dcmi_pssirst())
            .field("hsemrst", &self.hsemrst())
            .field("cryptrst", &self.cryptrst())
            .field("hashrst", &self.hashrst())
            .field("rngrst", &self.rngrst())
            .field("sdmmc2rst", &self.sdmmc2rst())
            .field("bdma1rst", &self.bdma1rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - digital camera interface block reset (DCMI or PSSI depending which IP is active) Set and reset by software.
    #[inline(always)]
    pub fn dcmi_pssirst(&mut self) -> DCMI_PSSIRST_W<'_, AHB2RSTRrs> {
        DCMI_PSSIRST_W::new(self, 0)
    }
    ///Bit 2 - HSEM block reset Set and reset by software.
    #[inline(always)]
    pub fn hsemrst(&mut self) -> HSEMRST_W<'_, AHB2RSTRrs> {
        HSEMRST_W::new(self, 2)
    }
    ///Bit 4 - cryptography block reset Set and reset by software.
    #[inline(always)]
    pub fn cryptrst(&mut self) -> CRYPTRST_W<'_, AHB2RSTRrs> {
        CRYPTRST_W::new(self, 4)
    }
    ///Bit 5 - hash block reset Set and reset by software.
    #[inline(always)]
    pub fn hashrst(&mut self) -> HASHRST_W<'_, AHB2RSTRrs> {
        HASHRST_W::new(self, 5)
    }
    ///Bit 6 - random number generator block reset Set and reset by software.
    #[inline(always)]
    pub fn rngrst(&mut self) -> RNGRST_W<'_, AHB2RSTRrs> {
        RNGRST_W::new(self, 6)
    }
    ///Bit 9 - SDMMC2 and SDMMC2 delay blocks reset Set and reset by software.
    #[inline(always)]
    pub fn sdmmc2rst(&mut self) -> SDMMC2RST_W<'_, AHB2RSTRrs> {
        SDMMC2RST_W::new(self, 9)
    }
    ///Bit 11 - BDMA1 reset (DFSDM dedicated DMA) Set and reset by software.
    #[inline(always)]
    pub fn bdma1rst(&mut self) -> BDMA1RST_W<'_, AHB2RSTRrs> {
        BDMA1RST_W::new(self, 11)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7A3.html#RCC:AHB2RSTR)*/
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
