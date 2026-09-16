///Register `DMABMR` reader
pub type R = crate::R<DMABMRrs>;
///Register `DMABMR` writer
pub type W = crate::W<DMABMRrs>;
///Field `SR` reader - SR
pub type SR_R = crate::BitReader;
///Field `SR` writer - SR
pub type SR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DA` reader - DA
pub type DA_R = crate::BitReader;
///Field `DA` writer - DA
pub type DA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSL` reader - DSL
pub type DSL_R = crate::FieldReader;
///Field `DSL` writer - DSL
pub type DSL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `EDFE` reader - EDFE
pub type EDFE_R = crate::BitReader;
///Field `EDFE` writer - EDFE
pub type EDFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PBL` reader - PBL
pub type PBL_R = crate::FieldReader;
///Field `PBL` writer - PBL
pub type PBL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `RTPR` reader - RTPR
pub type RTPR_R = crate::FieldReader;
///Field `RTPR` writer - RTPR
pub type RTPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FB` reader - FB
pub type FB_R = crate::BitReader;
///Field `FB` writer - FB
pub type FB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDP` reader - RDP
pub type RDP_R = crate::FieldReader;
///Field `RDP` writer - RDP
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `USP` reader - USP
pub type USP_R = crate::BitReader;
///Field `USP` writer - USP
pub type USP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPM` reader - FPM
pub type FPM_R = crate::BitReader;
///Field `FPM` writer - FPM
pub type FPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AAB` reader - AAB
pub type AAB_R = crate::BitReader;
///Field `AAB` writer - AAB
pub type AAB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MB` reader - MB
pub type MB_R = crate::BitReader;
///Field `MB` writer - MB
pub type MB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SR
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DA
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:6 - DSL
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    ///Bit 7 - EDFE
    #[inline(always)]
    pub fn edfe(&self) -> EDFE_R {
        EDFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:13 - PBL
    #[inline(always)]
    pub fn pbl(&self) -> PBL_R {
        PBL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 14:15 - RTPR
    #[inline(always)]
    pub fn rtpr(&self) -> RTPR_R {
        RTPR_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - FB
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:22 - RDP
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    ///Bit 23 - USP
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - FPM
    #[inline(always)]
    pub fn fpm(&self) -> FPM_R {
        FPM_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - AAB
    #[inline(always)]
    pub fn aab(&self) -> AAB_R {
        AAB_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - MB
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMABMR")
            .field("sr", &self.sr())
            .field("da", &self.da())
            .field("dsl", &self.dsl())
            .field("edfe", &self.edfe())
            .field("pbl", &self.pbl())
            .field("rtpr", &self.rtpr())
            .field("fb", &self.fb())
            .field("rdp", &self.rdp())
            .field("usp", &self.usp())
            .field("fpm", &self.fpm())
            .field("aab", &self.aab())
            .field("mb", &self.mb())
            .finish()
    }
}
impl W {
    ///Bit 0 - SR
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W<'_, DMABMRrs> {
        SR_W::new(self, 0)
    }
    ///Bit 1 - DA
    #[inline(always)]
    pub fn da(&mut self) -> DA_W<'_, DMABMRrs> {
        DA_W::new(self, 1)
    }
    ///Bits 2:6 - DSL
    #[inline(always)]
    pub fn dsl(&mut self) -> DSL_W<'_, DMABMRrs> {
        DSL_W::new(self, 2)
    }
    ///Bit 7 - EDFE
    #[inline(always)]
    pub fn edfe(&mut self) -> EDFE_W<'_, DMABMRrs> {
        EDFE_W::new(self, 7)
    }
    ///Bits 8:13 - PBL
    #[inline(always)]
    pub fn pbl(&mut self) -> PBL_W<'_, DMABMRrs> {
        PBL_W::new(self, 8)
    }
    ///Bits 14:15 - RTPR
    #[inline(always)]
    pub fn rtpr(&mut self) -> RTPR_W<'_, DMABMRrs> {
        RTPR_W::new(self, 14)
    }
    ///Bit 16 - FB
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W<'_, DMABMRrs> {
        FB_W::new(self, 16)
    }
    ///Bits 17:22 - RDP
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W<'_, DMABMRrs> {
        RDP_W::new(self, 17)
    }
    ///Bit 23 - USP
    #[inline(always)]
    pub fn usp(&mut self) -> USP_W<'_, DMABMRrs> {
        USP_W::new(self, 23)
    }
    ///Bit 24 - FPM
    #[inline(always)]
    pub fn fpm(&mut self) -> FPM_W<'_, DMABMRrs> {
        FPM_W::new(self, 24)
    }
    ///Bit 25 - AAB
    #[inline(always)]
    pub fn aab(&mut self) -> AAB_W<'_, DMABMRrs> {
        AAB_W::new(self, 25)
    }
    ///Bit 26 - MB
    #[inline(always)]
    pub fn mb(&mut self) -> MB_W<'_, DMABMRrs> {
        MB_W::new(self, 26)
    }
}
/**Ethernet DMA bus mode register

You can [`read`](crate::Reg::read) this register and get [`dmabmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmabmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#Ethernet_DMA:DMABMR)*/
pub struct DMABMRrs;
impl crate::RegisterSpec for DMABMRrs {
    type Ux = u32;
}
///`read()` method returns [`dmabmr::R`](R) reader structure
impl crate::Readable for DMABMRrs {}
///`write(|w| ..)` method takes [`dmabmr::W`](W) writer structure
impl crate::Writable for DMABMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMABMR to value 0x2101
impl crate::Resettable for DMABMRrs {
    const RESET_VALUE: u32 = 0x2101;
}
