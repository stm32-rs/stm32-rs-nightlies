#[doc = "Register `DMABMR` reader"]
pub type R = crate::R<DMABMRrs>;
#[doc = "Register `DMABMR` writer"]
pub type W = crate::W<DMABMRrs>;
#[doc = "Field `SR` reader - Software reset"]
pub type SR_R = crate::BitReader;
#[doc = "Field `SR` writer - Software reset"]
pub type SR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DA` reader - DMA Arbitration"]
pub type DA_R = crate::BitReader;
#[doc = "Field `DA` writer - DMA Arbitration"]
pub type DA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSL` reader - Descriptor skip length"]
pub type DSL_R = crate::FieldReader;
#[doc = "Field `DSL` writer - Descriptor skip length"]
pub type DSL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EDFE` reader - Enhanced descriptor format enable"]
pub type EDFE_R = crate::BitReader;
#[doc = "Field `EDFE` writer - Enhanced descriptor format enable"]
pub type EDFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBL` reader - Programmable burst length"]
pub type PBL_R = crate::FieldReader;
#[doc = "Field `PBL` writer - Programmable burst length"]
pub type PBL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PM` reader - Rx Tx priority ratio"]
pub type PM_R = crate::FieldReader;
#[doc = "Field `PM` writer - Rx Tx priority ratio"]
pub type PM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FB` reader - Fixed burst"]
pub type FB_R = crate::BitReader;
#[doc = "Field `FB` writer - Fixed burst"]
pub type FB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDP` reader - Rx DMA PBL"]
pub type RDP_R = crate::FieldReader;
#[doc = "Field `RDP` writer - Rx DMA PBL"]
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `USP` reader - Use separate PBL"]
pub type USP_R = crate::BitReader;
#[doc = "Field `USP` writer - Use separate PBL"]
pub type USP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPM` reader - 4xPBL mode"]
pub type FPM_R = crate::BitReader;
#[doc = "Field `FPM` writer - 4xPBL mode"]
pub type FPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AAB` reader - Address-aligned beats"]
pub type AAB_R = crate::BitReader;
#[doc = "Field `AAB` writer - Address-aligned beats"]
pub type AAB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB` reader - Mixed burst"]
pub type MB_R = crate::BitReader;
#[doc = "Field `MB` writer - Mixed burst"]
pub type MB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software reset"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Arbitration"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Descriptor skip length"]
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Enhanced descriptor format enable"]
    #[inline(always)]
    pub fn edfe(&self) -> EDFE_R {
        EDFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Programmable burst length"]
    #[inline(always)]
    pub fn pbl(&self) -> PBL_R {
        PBL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - Rx Tx priority ratio"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Fixed burst"]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Use separate PBL"]
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 4xPBL mode"]
    #[inline(always)]
    pub fn fpm(&self) -> FPM_R {
        FPM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Address-aligned beats"]
    #[inline(always)]
    pub fn aab(&self) -> AAB_R {
        AAB_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Mixed burst"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SR_W<DMABMRrs> {
        SR_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Arbitration"]
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DA_W<DMABMRrs> {
        DA_W::new(self, 1)
    }
    #[doc = "Bits 2:6 - Descriptor skip length"]
    #[inline(always)]
    #[must_use]
    pub fn dsl(&mut self) -> DSL_W<DMABMRrs> {
        DSL_W::new(self, 2)
    }
    #[doc = "Bit 7 - Enhanced descriptor format enable"]
    #[inline(always)]
    #[must_use]
    pub fn edfe(&mut self) -> EDFE_W<DMABMRrs> {
        EDFE_W::new(self, 7)
    }
    #[doc = "Bits 8:13 - Programmable burst length"]
    #[inline(always)]
    #[must_use]
    pub fn pbl(&mut self) -> PBL_W<DMABMRrs> {
        PBL_W::new(self, 8)
    }
    #[doc = "Bits 14:15 - Rx Tx priority ratio"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<DMABMRrs> {
        PM_W::new(self, 14)
    }
    #[doc = "Bit 16 - Fixed burst"]
    #[inline(always)]
    #[must_use]
    pub fn fb(&mut self) -> FB_W<DMABMRrs> {
        FB_W::new(self, 16)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL"]
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<DMABMRrs> {
        RDP_W::new(self, 17)
    }
    #[doc = "Bit 23 - Use separate PBL"]
    #[inline(always)]
    #[must_use]
    pub fn usp(&mut self) -> USP_W<DMABMRrs> {
        USP_W::new(self, 23)
    }
    #[doc = "Bit 24 - 4xPBL mode"]
    #[inline(always)]
    #[must_use]
    pub fn fpm(&mut self) -> FPM_W<DMABMRrs> {
        FPM_W::new(self, 24)
    }
    #[doc = "Bit 25 - Address-aligned beats"]
    #[inline(always)]
    #[must_use]
    pub fn aab(&mut self) -> AAB_W<DMABMRrs> {
        AAB_W::new(self, 25)
    }
    #[doc = "Bit 26 - Mixed burst"]
    #[inline(always)]
    #[must_use]
    pub fn mb(&mut self) -> MB_W<DMABMRrs> {
        MB_W::new(self, 26)
    }
}
#[doc = "Ethernet DMA bus mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmabmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmabmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMABMRrs;
impl crate::RegisterSpec for DMABMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmabmr::R`](R) reader structure"]
impl crate::Readable for DMABMRrs {}
#[doc = "`write(|w| ..)` method takes [`dmabmr::W`](W) writer structure"]
impl crate::Writable for DMABMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMABMR to value 0x0002_0101"]
impl crate::Resettable for DMABMRrs {
    const RESET_VALUE: u32 = 0x0002_0101;
}
