#[doc = "Register `FDCAN_PSR` reader"]
pub type R = crate::R<FDCAN_PSRrs>;
#[doc = "Register `FDCAN_PSR` writer"]
pub type W = crate::W<FDCAN_PSRrs>;
#[doc = "Field `LEC` reader - Last Error Code"]
pub type LEC_R = crate::FieldReader;
#[doc = "Field `LEC` writer - Last Error Code"]
pub type LEC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ACT` reader - Activity"]
pub type ACT_R = crate::FieldReader;
#[doc = "Field `EP` reader - Error Passive"]
pub type EP_R = crate::BitReader;
#[doc = "Field `EW` reader - Warning Status"]
pub type EW_R = crate::BitReader;
#[doc = "Field `BO` reader - Bus_Off Status"]
pub type BO_R = crate::BitReader;
#[doc = "Field `DLEC` reader - Data Last Error Code"]
pub type DLEC_R = crate::FieldReader;
#[doc = "Field `DLEC` writer - Data Last Error Code"]
pub type DLEC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESI` reader - ESI flag of last received FDCAN Message"]
pub type RESI_R = crate::BitReader;
#[doc = "Field `RESI` writer - ESI flag of last received FDCAN Message"]
pub type RESI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBRS` reader - BRS flag of last received FDCAN Message"]
pub type RBRS_R = crate::BitReader;
#[doc = "Field `RBRS` writer - BRS flag of last received FDCAN Message"]
pub type RBRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REDL` reader - Received FDCAN Message"]
pub type REDL_R = crate::BitReader;
#[doc = "Field `REDL` writer - Received FDCAN Message"]
pub type REDL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PXE` reader - Protocol Exception Event"]
pub type PXE_R = crate::BitReader;
#[doc = "Field `PXE` writer - Protocol Exception Event"]
pub type PXE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDCV` reader - Transmitter Delay Compensation Value"]
pub type TDCV_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Activity"]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Error Passive"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Warning Status"]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus_Off Status"]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Data Last Error Code"]
    #[inline(always)]
    pub fn dlec(&self) -> DLEC_R {
        DLEC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - ESI flag of last received FDCAN Message"]
    #[inline(always)]
    pub fn resi(&self) -> RESI_R {
        RESI_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BRS flag of last received FDCAN Message"]
    #[inline(always)]
    pub fn rbrs(&self) -> RBRS_R {
        RBRS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Received FDCAN Message"]
    #[inline(always)]
    pub fn redl(&self) -> REDL_R {
        REDL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Protocol Exception Event"]
    #[inline(always)]
    pub fn pxe(&self) -> PXE_R {
        PXE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Transmitter Delay Compensation Value"]
    #[inline(always)]
    pub fn tdcv(&self) -> TDCV_R {
        TDCV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    #[must_use]
    pub fn lec(&mut self) -> LEC_W<FDCAN_PSRrs> {
        LEC_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - Data Last Error Code"]
    #[inline(always)]
    #[must_use]
    pub fn dlec(&mut self) -> DLEC_W<FDCAN_PSRrs> {
        DLEC_W::new(self, 8)
    }
    #[doc = "Bit 11 - ESI flag of last received FDCAN Message"]
    #[inline(always)]
    #[must_use]
    pub fn resi(&mut self) -> RESI_W<FDCAN_PSRrs> {
        RESI_W::new(self, 11)
    }
    #[doc = "Bit 12 - BRS flag of last received FDCAN Message"]
    #[inline(always)]
    #[must_use]
    pub fn rbrs(&mut self) -> RBRS_W<FDCAN_PSRrs> {
        RBRS_W::new(self, 12)
    }
    #[doc = "Bit 13 - Received FDCAN Message"]
    #[inline(always)]
    #[must_use]
    pub fn redl(&mut self) -> REDL_W<FDCAN_PSRrs> {
        REDL_W::new(self, 13)
    }
    #[doc = "Bit 14 - Protocol Exception Event"]
    #[inline(always)]
    #[must_use]
    pub fn pxe(&mut self) -> PXE_W<FDCAN_PSRrs> {
        PXE_W::new(self, 14)
    }
}
#[doc = "FDCAN Protocol Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_psr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_psr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_PSRrs;
impl crate::RegisterSpec for FDCAN_PSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_psr::R`](R) reader structure"]
impl crate::Readable for FDCAN_PSRrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_psr::W`](W) writer structure"]
impl crate::Writable for FDCAN_PSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_PSR to value 0x0707"]
impl crate::Resettable for FDCAN_PSRrs {
    const RESET_VALUE: u32 = 0x0707;
}
