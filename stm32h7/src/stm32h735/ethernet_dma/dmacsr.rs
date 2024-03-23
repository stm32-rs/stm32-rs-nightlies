#[doc = "Register `DMACSR` reader"]
pub type R = crate::R<DMACSRrs>;
#[doc = "Register `DMACSR` writer"]
pub type W = crate::W<DMACSRrs>;
#[doc = "Field `TI` reader - Transmit Interrupt"]
pub type TI_R = crate::BitReader;
#[doc = "Field `TI` writer - Transmit Interrupt"]
pub type TI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPS` reader - Transmit Process Stopped"]
pub type TPS_R = crate::BitReader;
#[doc = "Field `TPS` writer - Transmit Process Stopped"]
pub type TPS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBU` reader - Transmit Buffer Unavailable"]
pub type TBU_R = crate::BitReader;
#[doc = "Field `TBU` writer - Transmit Buffer Unavailable"]
pub type TBU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RI` reader - Receive Interrupt"]
pub type RI_R = crate::BitReader;
#[doc = "Field `RI` writer - Receive Interrupt"]
pub type RI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBU` reader - Receive Buffer Unavailable"]
pub type RBU_R = crate::BitReader;
#[doc = "Field `RBU` writer - Receive Buffer Unavailable"]
pub type RBU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPS` reader - Receive Process Stopped"]
pub type RPS_R = crate::BitReader;
#[doc = "Field `RPS` writer - Receive Process Stopped"]
pub type RPS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWT` reader - Receive Watchdog Timeout"]
pub type RWT_R = crate::BitReader;
#[doc = "Field `RWT` writer - Receive Watchdog Timeout"]
pub type RWT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETI` reader - Early Transmit Interrupt"]
pub type ETI_R = crate::BitReader;
#[doc = "Field `ETI` writer - Early Transmit Interrupt"]
pub type ETI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERI` reader - Early Receive Interrupt"]
pub type ERI_R = crate::BitReader;
#[doc = "Field `ERI` writer - Early Receive Interrupt"]
pub type ERI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBE` reader - Fatal Bus Error"]
pub type FBE_R = crate::BitReader;
#[doc = "Field `FBE` writer - Fatal Bus Error"]
pub type FBE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDE` reader - Context Descriptor Error"]
pub type CDE_R = crate::BitReader;
#[doc = "Field `CDE` writer - Context Descriptor Error"]
pub type CDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary"]
pub type AIS_R = crate::BitReader;
#[doc = "Field `AIS` writer - Abnormal Interrupt Summary"]
pub type AIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NIS` reader - Normal Interrupt Summary"]
pub type NIS_R = crate::BitReader;
#[doc = "Field `NIS` writer - Normal Interrupt Summary"]
pub type NIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEB` reader - Tx DMA Error Bits"]
pub type TEB_R = crate::FieldReader;
#[doc = "Field `TEB` writer - Tx DMA Error Bits"]
pub type TEB_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REB` reader - Rx DMA Error Bits"]
pub type REB_R = crate::FieldReader;
#[doc = "Field `REB` writer - Rx DMA Error Bits"]
pub type REB_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline(always)]
    pub fn tbu(&self) -> TBU_R {
        TBU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline(always)]
    pub fn rbu(&self) -> RBU_R {
        RBU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline(always)]
    pub fn eti(&self) -> ETI_R {
        ETI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Early Receive Interrupt"]
    #[inline(always)]
    pub fn eri(&self) -> ERI_R {
        ERI_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Fatal Bus Error"]
    #[inline(always)]
    pub fn fbe(&self) -> FBE_R {
        FBE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Context Descriptor Error"]
    #[inline(always)]
    pub fn cde(&self) -> CDE_R {
        CDE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Abnormal Interrupt Summary"]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Normal Interrupt Summary"]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Tx DMA Error Bits"]
    #[inline(always)]
    pub fn teb(&self) -> TEB_R {
        TEB_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - Rx DMA Error Bits"]
    #[inline(always)]
    pub fn reb(&self) -> REB_R {
        REB_R::new(((self.bits >> 19) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ti(&mut self) -> TI_W<DMACSRrs> {
        TI_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline(always)]
    #[must_use]
    pub fn tps(&mut self) -> TPS_W<DMACSRrs> {
        TPS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline(always)]
    #[must_use]
    pub fn tbu(&mut self) -> TBU_W<DMACSRrs> {
        TBU_W::new(self, 2)
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ri(&mut self) -> RI_W<DMACSRrs> {
        RI_W::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline(always)]
    #[must_use]
    pub fn rbu(&mut self) -> RBU_W<DMACSRrs> {
        RBU_W::new(self, 7)
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline(always)]
    #[must_use]
    pub fn rps(&mut self) -> RPS_W<DMACSRrs> {
        RPS_W::new(self, 8)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn rwt(&mut self) -> RWT_W<DMACSRrs> {
        RWT_W::new(self, 9)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eti(&mut self) -> ETI_W<DMACSRrs> {
        ETI_W::new(self, 10)
    }
    #[doc = "Bit 11 - Early Receive Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eri(&mut self) -> ERI_W<DMACSRrs> {
        ERI_W::new(self, 11)
    }
    #[doc = "Bit 12 - Fatal Bus Error"]
    #[inline(always)]
    #[must_use]
    pub fn fbe(&mut self) -> FBE_W<DMACSRrs> {
        FBE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Context Descriptor Error"]
    #[inline(always)]
    #[must_use]
    pub fn cde(&mut self) -> CDE_W<DMACSRrs> {
        CDE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Abnormal Interrupt Summary"]
    #[inline(always)]
    #[must_use]
    pub fn ais(&mut self) -> AIS_W<DMACSRrs> {
        AIS_W::new(self, 14)
    }
    #[doc = "Bit 15 - Normal Interrupt Summary"]
    #[inline(always)]
    #[must_use]
    pub fn nis(&mut self) -> NIS_W<DMACSRrs> {
        NIS_W::new(self, 15)
    }
    #[doc = "Bits 16:18 - Tx DMA Error Bits"]
    #[inline(always)]
    #[must_use]
    pub fn teb(&mut self) -> TEB_W<DMACSRrs> {
        TEB_W::new(self, 16)
    }
    #[doc = "Bits 19:21 - Rx DMA Error Bits"]
    #[inline(always)]
    #[must_use]
    pub fn reb(&mut self) -> REB_W<DMACSRrs> {
        REB_W::new(self, 19)
    }
}
#[doc = "Channel status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACSRrs;
impl crate::RegisterSpec for DMACSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacsr::R`](R) reader structure"]
impl crate::Readable for DMACSRrs {}
#[doc = "`write(|w| ..)` method takes [`dmacsr::W`](W) writer structure"]
impl crate::Writable for DMACSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACSR to value 0"]
impl crate::Resettable for DMACSRrs {
    const RESET_VALUE: u32 = 0;
}
