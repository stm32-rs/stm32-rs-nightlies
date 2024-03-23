#[doc = "Register `DMAMR` reader"]
pub type R = crate::R<DMAMRrs>;
#[doc = "Register `DMAMR` writer"]
pub type W = crate::W<DMAMRrs>;
#[doc = "Field `SWR` reader - Software Reset"]
pub type SWR_R = crate::BitReader;
#[doc = "Field `SWR` writer - Software Reset"]
pub type SWR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DA` reader - DMA Tx or Rx Arbitration Scheme"]
pub type DA_R = crate::BitReader;
#[doc = "Field `DA` writer - DMA Tx or Rx Arbitration Scheme"]
pub type DA_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 1 - DMA Tx or Rx Arbitration Scheme"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 1) != 0)
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
    pub fn swr(&mut self) -> SWR_W<DMAMRrs> {
        SWR_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Tx or Rx Arbitration Scheme"]
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DA_W<DMAMRrs> {
        DA_W::new(self, 1)
    }
    #[doc = "Bit 11 - Transmit priority"]
    #[inline(always)]
    #[must_use]
    pub fn txpr(&mut self) -> TXPR_W<DMAMRrs> {
        TXPR_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - Priority ratio"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<DMAMRrs> {
        PR_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - Interrupt Mode"]
    #[inline(always)]
    #[must_use]
    pub fn intm(&mut self) -> INTM_W<DMAMRrs> {
        INTM_W::new(self, 16)
    }
}
#[doc = "DMA mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAMRrs;
impl crate::RegisterSpec for DMAMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamr::R`](R) reader structure"]
impl crate::Readable for DMAMRrs {}
#[doc = "`write(|w| ..)` method takes [`dmamr::W`](W) writer structure"]
impl crate::Writable for DMAMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAMR to value 0"]
impl crate::Resettable for DMAMRrs {
    const RESET_VALUE: u32 = 0;
}
