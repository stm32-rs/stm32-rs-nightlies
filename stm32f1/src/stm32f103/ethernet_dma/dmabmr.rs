///Register `DMABMR` reader
pub type R = crate::R<DMABMRrs>;
///Register `DMABMR` writer
pub type W = crate::W<DMABMRrs>;
///Field `SR` reader - Software reset
pub type SR_R = crate::BitReader;
///Field `SR` writer - Software reset
pub type SR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DA` reader - DMA Arbitration
pub type DA_R = crate::BitReader;
///Field `DA` writer - DMA Arbitration
pub type DA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSL` reader - Descriptor skip length
pub type DSL_R = crate::FieldReader;
///Field `DSL` writer - Descriptor skip length
pub type DSL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `PBL` reader - Programmable burst length
pub type PBL_R = crate::FieldReader;
///Field `PBL` writer - Programmable burst length
pub type PBL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `RTPR` reader - Rx Tx priority ratio
pub type RTPR_R = crate::FieldReader;
///Field `RTPR` writer - Rx Tx priority ratio
pub type RTPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FB` reader - Fixed burst
pub type FB_R = crate::BitReader;
///Field `FB` writer - Fixed burst
pub type FB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDP` reader - Rx DMA PBL
pub type RDP_R = crate::FieldReader;
///Field `RDP` writer - Rx DMA PBL
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `USP` reader - Use separate PBL
pub type USP_R = crate::BitReader;
///Field `USP` writer - Use separate PBL
pub type USP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPM` reader - 4xPBL mode
pub type FPM_R = crate::BitReader;
///Field `FPM` writer - 4xPBL mode
pub type FPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AAB` reader - Address-aligned beats
pub type AAB_R = crate::BitReader;
///Field `AAB` writer - Address-aligned beats
pub type AAB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Software reset
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA Arbitration
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:6 - Descriptor skip length
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    ///Bits 8:13 - Programmable burst length
    #[inline(always)]
    pub fn pbl(&self) -> PBL_R {
        PBL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 14:15 - Rx Tx priority ratio
    #[inline(always)]
    pub fn rtpr(&self) -> RTPR_R {
        RTPR_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - Fixed burst
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:22 - Rx DMA PBL
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    ///Bit 23 - Use separate PBL
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - 4xPBL mode
    #[inline(always)]
    pub fn fpm(&self) -> FPM_R {
        FPM_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Address-aligned beats
    #[inline(always)]
    pub fn aab(&self) -> AAB_R {
        AAB_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMABMR")
            .field("sr", &self.sr())
            .field("da", &self.da())
            .field("dsl", &self.dsl())
            .field("pbl", &self.pbl())
            .field("rtpr", &self.rtpr())
            .field("fb", &self.fb())
            .field("rdp", &self.rdp())
            .field("usp", &self.usp())
            .field("fpm", &self.fpm())
            .field("aab", &self.aab())
            .finish()
    }
}
impl W {
    ///Bit 0 - Software reset
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W<DMABMRrs> {
        SR_W::new(self, 0)
    }
    ///Bit 1 - DMA Arbitration
    #[inline(always)]
    pub fn da(&mut self) -> DA_W<DMABMRrs> {
        DA_W::new(self, 1)
    }
    ///Bits 2:6 - Descriptor skip length
    #[inline(always)]
    pub fn dsl(&mut self) -> DSL_W<DMABMRrs> {
        DSL_W::new(self, 2)
    }
    ///Bits 8:13 - Programmable burst length
    #[inline(always)]
    pub fn pbl(&mut self) -> PBL_W<DMABMRrs> {
        PBL_W::new(self, 8)
    }
    ///Bits 14:15 - Rx Tx priority ratio
    #[inline(always)]
    pub fn rtpr(&mut self) -> RTPR_W<DMABMRrs> {
        RTPR_W::new(self, 14)
    }
    ///Bit 16 - Fixed burst
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W<DMABMRrs> {
        FB_W::new(self, 16)
    }
    ///Bits 17:22 - Rx DMA PBL
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W<DMABMRrs> {
        RDP_W::new(self, 17)
    }
    ///Bit 23 - Use separate PBL
    #[inline(always)]
    pub fn usp(&mut self) -> USP_W<DMABMRrs> {
        USP_W::new(self, 23)
    }
    ///Bit 24 - 4xPBL mode
    #[inline(always)]
    pub fn fpm(&mut self) -> FPM_W<DMABMRrs> {
        FPM_W::new(self, 24)
    }
    ///Bit 25 - Address-aligned beats
    #[inline(always)]
    pub fn aab(&mut self) -> AAB_W<DMABMRrs> {
        AAB_W::new(self, 25)
    }
}
/**Ethernet DMA bus mode register

You can [`read`](crate::Reg::read) this register and get [`dmabmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmabmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#Ethernet_DMA:DMABMR)*/
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
///`reset()` method sets DMABMR to value 0x0002_0101
impl crate::Resettable for DMABMRrs {
    const RESET_VALUE: u32 = 0x0002_0101;
}
