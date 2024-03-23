#[doc = "Register `ETH_DMAMR` reader"]
pub type R = crate::R<ETH_DMAMRrs>;
#[doc = "Register `ETH_DMAMR` writer"]
pub type W = crate::W<ETH_DMAMRrs>;
#[doc = "Field `SWR` reader - Software Reset"]
pub type SWR_R = crate::BitReader;
#[doc = "Field `SWR` writer - Software Reset"]
pub type SWR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAA` reader - TAA"]
pub type TAA_R = crate::FieldReader;
#[doc = "Field `TAA` writer - TAA"]
pub type TAA_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TXPR` reader - Transmit priority"]
pub type TXPR_R = crate::BitReader;
#[doc = "Field `TXPR` writer - Transmit priority"]
pub type TXPR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR` reader - Priority ratio"]
pub type PR_R = crate::FieldReader;
#[doc = "Field `PR` writer - Priority ratio"]
pub type PR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `INTM` reader - Interrupt Mode"]
pub type INTM_R = crate::FieldReader;
#[doc = "Field `INTM` writer - Interrupt Mode"]
pub type INTM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:4 - TAA"]
    #[inline(always)]
    pub fn taa(&self) -> TAA_R {
        TAA_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 11 - Transmit priority"]
    #[inline(always)]
    pub fn txpr(&self) -> TXPR_R {
        TXPR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Priority ratio"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Interrupt Mode"]
    #[inline(always)]
    pub fn intm(&self) -> INTM_R {
        INTM_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swr(&mut self) -> SWR_W<ETH_DMAMRrs> {
        SWR_W::new(self, 0)
    }
    #[doc = "Bits 2:4 - TAA"]
    #[inline(always)]
    #[must_use]
    pub fn taa(&mut self) -> TAA_W<ETH_DMAMRrs> {
        TAA_W::new(self, 2)
    }
    #[doc = "Bit 11 - Transmit priority"]
    #[inline(always)]
    #[must_use]
    pub fn txpr(&mut self) -> TXPR_W<ETH_DMAMRrs> {
        TXPR_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - Priority ratio"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<ETH_DMAMRrs> {
        PR_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - Interrupt Mode"]
    #[inline(always)]
    #[must_use]
    pub fn intm(&mut self) -> INTM_W<ETH_DMAMRrs> {
        INTM_W::new(self, 16)
    }
}
#[doc = "DMA mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_dmamr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_dmamr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_DMAMRrs;
impl crate::RegisterSpec for ETH_DMAMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_dmamr::R`](R) reader structure"]
impl crate::Readable for ETH_DMAMRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_dmamr::W`](W) writer structure"]
impl crate::Writable for ETH_DMAMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_DMAMR to value 0x8000"]
impl crate::Resettable for ETH_DMAMRrs {
    const RESET_VALUE: u32 = 0x8000;
}
