#[doc = "Register `AHB2RSTR` reader"]
pub type R = crate::R<AHB2RSTRrs>;
#[doc = "Register `AHB2RSTR` writer"]
pub type W = crate::W<AHB2RSTRrs>;
#[doc = "digital camera interface block reset (DCMI or PSSI depending which IP is active) Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMI_PSSIRST {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<DCMI_PSSIRST> for bool {
    #[inline(always)]
    fn from(variant: DCMI_PSSIRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMI_PSSIRST` reader - digital camera interface block reset (DCMI or PSSI depending which IP is active) Set and reset by software."]
pub type DCMI_PSSIRST_R = crate::BitReader<DCMI_PSSIRST>;
impl DCMI_PSSIRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DCMI_PSSIRST> {
        match self.bits {
            true => Some(DCMI_PSSIRST::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DCMI_PSSIRST::Reset
    }
}
#[doc = "Field `DCMI_PSSIRST` writer - digital camera interface block reset (DCMI or PSSI depending which IP is active) Set and reset by software."]
pub type DCMI_PSSIRST_W<'a, REG> = crate::BitWriter<'a, REG, DCMI_PSSIRST>;
impl<'a, REG> DCMI_PSSIRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DCMI_PSSIRST::Reset)
    }
}
#[doc = "Field `HSEMRST` reader - HSEM block reset Set and reset by software."]
pub use DCMI_PSSIRST_R as HSEMRST_R;
#[doc = "Field `CRYPTRST` reader - cryptography block reset Set and reset by software."]
pub use DCMI_PSSIRST_R as CRYPTRST_R;
#[doc = "Field `HASHRST` reader - hash block reset Set and reset by software."]
pub use DCMI_PSSIRST_R as HASHRST_R;
#[doc = "Field `RNGRST` reader - random number generator block reset Set and reset by software."]
pub use DCMI_PSSIRST_R as RNGRST_R;
#[doc = "Field `SDMMC2RST` reader - SDMMC2 and SDMMC2 delay blocks reset Set and reset by software."]
pub use DCMI_PSSIRST_R as SDMMC2RST_R;
#[doc = "Field `BDMA1RST` reader - BDMA1 reset (DFSDM dedicated DMA) Set and reset by software."]
pub use DCMI_PSSIRST_R as BDMA1RST_R;
#[doc = "Field `HSEMRST` writer - HSEM block reset Set and reset by software."]
pub use DCMI_PSSIRST_W as HSEMRST_W;
#[doc = "Field `CRYPTRST` writer - cryptography block reset Set and reset by software."]
pub use DCMI_PSSIRST_W as CRYPTRST_W;
#[doc = "Field `HASHRST` writer - hash block reset Set and reset by software."]
pub use DCMI_PSSIRST_W as HASHRST_W;
#[doc = "Field `RNGRST` writer - random number generator block reset Set and reset by software."]
pub use DCMI_PSSIRST_W as RNGRST_W;
#[doc = "Field `SDMMC2RST` writer - SDMMC2 and SDMMC2 delay blocks reset Set and reset by software."]
pub use DCMI_PSSIRST_W as SDMMC2RST_W;
#[doc = "Field `BDMA1RST` writer - BDMA1 reset (DFSDM dedicated DMA) Set and reset by software."]
pub use DCMI_PSSIRST_W as BDMA1RST_W;
impl R {
    #[doc = "Bit 0 - digital camera interface block reset (DCMI or PSSI depending which IP is active) Set and reset by software."]
    #[inline(always)]
    pub fn dcmi_pssirst(&self) -> DCMI_PSSIRST_R {
        DCMI_PSSIRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - HSEM block reset Set and reset by software."]
    #[inline(always)]
    pub fn hsemrst(&self) -> HSEMRST_R {
        HSEMRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - cryptography block reset Set and reset by software."]
    #[inline(always)]
    pub fn cryptrst(&self) -> CRYPTRST_R {
        CRYPTRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - hash block reset Set and reset by software."]
    #[inline(always)]
    pub fn hashrst(&self) -> HASHRST_R {
        HASHRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - random number generator block reset Set and reset by software."]
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 delay blocks reset Set and reset by software."]
    #[inline(always)]
    pub fn sdmmc2rst(&self) -> SDMMC2RST_R {
        SDMMC2RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - BDMA1 reset (DFSDM dedicated DMA) Set and reset by software."]
    #[inline(always)]
    pub fn bdma1rst(&self) -> BDMA1RST_R {
        BDMA1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - digital camera interface block reset (DCMI or PSSI depending which IP is active) Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn dcmi_pssirst(&mut self) -> DCMI_PSSIRST_W<AHB2RSTRrs> {
        DCMI_PSSIRST_W::new(self, 0)
    }
    #[doc = "Bit 2 - HSEM block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn hsemrst(&mut self) -> HSEMRST_W<AHB2RSTRrs> {
        HSEMRST_W::new(self, 2)
    }
    #[doc = "Bit 4 - cryptography block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn cryptrst(&mut self) -> CRYPTRST_W<AHB2RSTRrs> {
        CRYPTRST_W::new(self, 4)
    }
    #[doc = "Bit 5 - hash block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn hashrst(&mut self) -> HASHRST_W<AHB2RSTRrs> {
        HASHRST_W::new(self, 5)
    }
    #[doc = "Bit 6 - random number generator block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn rngrst(&mut self) -> RNGRST_W<AHB2RSTRrs> {
        RNGRST_W::new(self, 6)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 delay blocks reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2rst(&mut self) -> SDMMC2RST_W<AHB2RSTRrs> {
        SDMMC2RST_W::new(self, 9)
    }
    #[doc = "Bit 11 - BDMA1 reset (DFSDM dedicated DMA) Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn bdma1rst(&mut self) -> BDMA1RST_W<AHB2RSTRrs> {
        BDMA1RST_W::new(self, 11)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB2RSTRrs;
impl crate::RegisterSpec for AHB2RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2rstr::R`](R) reader structure"]
impl crate::Readable for AHB2RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb2rstr::W`](W) writer structure"]
impl crate::Writable for AHB2RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB2RSTR to value 0"]
impl crate::Resettable for AHB2RSTRrs {
    const RESET_VALUE: u32 = 0;
}
