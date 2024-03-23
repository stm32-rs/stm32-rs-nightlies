#[doc = "Register `PSR` reader"]
pub type R = crate::R<PSRrs>;
#[doc = "Register `PSR` writer"]
pub type W = crate::W<PSRrs>;
#[doc = "Field `LEC` reader - LEC"]
pub type LEC_R = crate::FieldReader;
#[doc = "Field `LEC` writer - LEC"]
pub type LEC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ACT` writer - ACT"]
pub type ACT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP` reader - EP"]
pub type EP_R = crate::BitReader;
#[doc = "Field `EP` writer - EP"]
pub type EP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EW` reader - EW"]
pub type EW_R = crate::BitReader;
#[doc = "Field `EW` writer - EW"]
pub type EW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BO` reader - BO"]
pub type BO_R = crate::BitReader;
#[doc = "Field `BO` writer - BO"]
pub type BO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLEC` writer - DLEC"]
pub type DLEC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESI` reader - RESI"]
pub type RESI_R = crate::BitReader;
#[doc = "Field `RESI` writer - RESI"]
pub type RESI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBRS` reader - RBRS"]
pub type RBRS_R = crate::BitReader;
#[doc = "Field `RBRS` writer - RBRS"]
pub type RBRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REDL` reader - REDL"]
pub type REDL_R = crate::BitReader;
#[doc = "Field `REDL` writer - REDL"]
pub type REDL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PXE` reader - PXE"]
pub type PXE_R = crate::BitReader;
#[doc = "Field `PXE` writer - PXE"]
pub type PXE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDCV` reader - TDCV"]
pub type TDCV_R = crate::FieldReader;
#[doc = "Field `TDCV` writer - TDCV"]
pub type TDCV_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:2 - LEC"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - EP"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EW"]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BO"]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - RESI"]
    #[inline(always)]
    pub fn resi(&self) -> RESI_R {
        RESI_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RBRS"]
    #[inline(always)]
    pub fn rbrs(&self) -> RBRS_R {
        RBRS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - REDL"]
    #[inline(always)]
    pub fn redl(&self) -> REDL_R {
        REDL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PXE"]
    #[inline(always)]
    pub fn pxe(&self) -> PXE_R {
        PXE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:22 - TDCV"]
    #[inline(always)]
    pub fn tdcv(&self) -> TDCV_R {
        TDCV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - LEC"]
    #[inline(always)]
    #[must_use]
    pub fn lec(&mut self) -> LEC_W<PSRrs> {
        LEC_W::new(self, 0)
    }
    #[doc = "Bits 3:4 - ACT"]
    #[inline(always)]
    #[must_use]
    pub fn act(&mut self) -> ACT_W<PSRrs> {
        ACT_W::new(self, 3)
    }
    #[doc = "Bit 5 - EP"]
    #[inline(always)]
    #[must_use]
    pub fn ep(&mut self) -> EP_W<PSRrs> {
        EP_W::new(self, 5)
    }
    #[doc = "Bit 6 - EW"]
    #[inline(always)]
    #[must_use]
    pub fn ew(&mut self) -> EW_W<PSRrs> {
        EW_W::new(self, 6)
    }
    #[doc = "Bit 7 - BO"]
    #[inline(always)]
    #[must_use]
    pub fn bo(&mut self) -> BO_W<PSRrs> {
        BO_W::new(self, 7)
    }
    #[doc = "Bits 8:10 - DLEC"]
    #[inline(always)]
    #[must_use]
    pub fn dlec(&mut self) -> DLEC_W<PSRrs> {
        DLEC_W::new(self, 8)
    }
    #[doc = "Bit 11 - RESI"]
    #[inline(always)]
    #[must_use]
    pub fn resi(&mut self) -> RESI_W<PSRrs> {
        RESI_W::new(self, 11)
    }
    #[doc = "Bit 12 - RBRS"]
    #[inline(always)]
    #[must_use]
    pub fn rbrs(&mut self) -> RBRS_W<PSRrs> {
        RBRS_W::new(self, 12)
    }
    #[doc = "Bit 13 - REDL"]
    #[inline(always)]
    #[must_use]
    pub fn redl(&mut self) -> REDL_W<PSRrs> {
        REDL_W::new(self, 13)
    }
    #[doc = "Bit 14 - PXE"]
    #[inline(always)]
    #[must_use]
    pub fn pxe(&mut self) -> PXE_W<PSRrs> {
        PXE_W::new(self, 14)
    }
    #[doc = "Bits 16:22 - TDCV"]
    #[inline(always)]
    #[must_use]
    pub fn tdcv(&mut self) -> TDCV_W<PSRrs> {
        TDCV_W::new(self, 16)
    }
}
#[doc = "FDCAN Protocol Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSRrs;
impl crate::RegisterSpec for PSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr::R`](R) reader structure"]
impl crate::Readable for PSRrs {}
#[doc = "`write(|w| ..)` method takes [`psr::W`](W) writer structure"]
impl crate::Writable for PSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSR to value 0x0707"]
impl crate::Resettable for PSRrs {
    const RESET_VALUE: u32 = 0x0707;
}
