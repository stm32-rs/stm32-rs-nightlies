#[doc = "Register `HSPI_HLCR` reader"]
pub type R = crate::R<HSPI_HLCRrs>;
#[doc = "Register `HSPI_HLCR` writer"]
pub type W = crate::W<HSPI_HLCRrs>;
#[doc = "Field `LM` reader - Latency mode This bit selects the Latency mode."]
pub type LM_R = crate::BitReader;
#[doc = "Field `LM` writer - Latency mode This bit selects the Latency mode."]
pub type LM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WZL` reader - Write zero latency This bit enables zero latency on write operations."]
pub type WZL_R = crate::BitReader;
#[doc = "Field `WZL` writer - Write zero latency This bit enables zero latency on write operations."]
pub type WZL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TACC` reader - 7: 0\\]: Access time Device access time expressed in number of communication clock cycles"]
pub type TACC_R = crate::FieldReader;
#[doc = "Field `TACC` writer - 7: 0\\]: Access time Device access time expressed in number of communication clock cycles"]
pub type TACC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRWR` reader - Read write recovery time Device read write recovery time expressed in number of communication clock cycles"]
pub type TRWR_R = crate::FieldReader;
#[doc = "Field `TRWR` writer - Read write recovery time Device read write recovery time expressed in number of communication clock cycles"]
pub type TRWR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Latency mode This bit selects the Latency mode."]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write zero latency This bit enables zero latency on write operations."]
    #[inline(always)]
    pub fn wzl(&self) -> WZL_R {
        WZL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:15 - 7: 0\\]: Access time Device access time expressed in number of communication clock cycles"]
    #[inline(always)]
    pub fn tacc(&self) -> TACC_R {
        TACC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Read write recovery time Device read write recovery time expressed in number of communication clock cycles"]
    #[inline(always)]
    pub fn trwr(&self) -> TRWR_R {
        TRWR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Latency mode This bit selects the Latency mode."]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<HSPI_HLCRrs> {
        LM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write zero latency This bit enables zero latency on write operations."]
    #[inline(always)]
    #[must_use]
    pub fn wzl(&mut self) -> WZL_W<HSPI_HLCRrs> {
        WZL_W::new(self, 1)
    }
    #[doc = "Bits 8:15 - 7: 0\\]: Access time Device access time expressed in number of communication clock cycles"]
    #[inline(always)]
    #[must_use]
    pub fn tacc(&mut self) -> TACC_W<HSPI_HLCRrs> {
        TACC_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Read write recovery time Device read write recovery time expressed in number of communication clock cycles"]
    #[inline(always)]
    #[must_use]
    pub fn trwr(&mut self) -> TRWR_W<HSPI_HLCRrs> {
        TRWR_W::new(self, 16)
    }
}
#[doc = "HSPI HyperBus latency configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_hlcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_hlcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSPI_HLCRrs;
impl crate::RegisterSpec for HSPI_HLCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hspi_hlcr::R`](R) reader structure"]
impl crate::Readable for HSPI_HLCRrs {}
#[doc = "`write(|w| ..)` method takes [`hspi_hlcr::W`](W) writer structure"]
impl crate::Writable for HSPI_HLCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSPI_HLCR to value 0"]
impl crate::Resettable for HSPI_HLCRrs {
    const RESET_VALUE: u32 = 0;
}
