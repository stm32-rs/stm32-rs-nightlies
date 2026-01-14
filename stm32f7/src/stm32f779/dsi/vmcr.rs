///Register `VMCR` reader
pub type R = crate::R<VMCRrs>;
///Register `VMCR` writer
pub type W = crate::W<VMCRrs>;
///Field `VMT` reader - Video mode Type
pub type VMT_R = crate::FieldReader;
///Field `VMT` writer - Video mode Type
pub type VMT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LPVSAE` reader - Low-Power Vertical Sync Active Enable
pub type LPVSAE_R = crate::BitReader;
///Field `LPVSAE` writer - Low-Power Vertical Sync Active Enable
pub type LPVSAE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPVBPE` reader - Low-power Vertical Back-Porch Enable
pub type LPVBPE_R = crate::BitReader;
///Field `LPVBPE` writer - Low-power Vertical Back-Porch Enable
pub type LPVBPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPVFPE` reader - Low-power Vertical Front-porch Enable
pub type LPVFPE_R = crate::BitReader;
///Field `LPVFPE` writer - Low-power Vertical Front-porch Enable
pub type LPVFPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPVAE` reader - Low-Power Vertical Active Enable
pub type LPVAE_R = crate::BitReader;
///Field `LPVAE` writer - Low-Power Vertical Active Enable
pub type LPVAE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPHBPE` reader - Low-Power Horizontal Back-Porch Enable
pub type LPHBPE_R = crate::BitReader;
///Field `LPHBPE` writer - Low-Power Horizontal Back-Porch Enable
pub type LPHBPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPHFPE` reader - Low-Power Horizontal Front-Porch Enable
pub type LPHFPE_R = crate::BitReader;
///Field `LPHFPE` writer - Low-Power Horizontal Front-Porch Enable
pub type LPHFPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FBTAAE` reader - Frame Bus-Turn-Around Acknowledge Enable
pub type FBTAAE_R = crate::BitReader;
///Field `FBTAAE` writer - Frame Bus-Turn-Around Acknowledge Enable
pub type FBTAAE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPCE` reader - Low-Power Command Enable
pub type LPCE_R = crate::BitReader;
///Field `LPCE` writer - Low-Power Command Enable
pub type LPCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PGE` reader - Pattern Generator Enable
pub type PGE_R = crate::BitReader;
///Field `PGE` writer - Pattern Generator Enable
pub type PGE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PGM` reader - Pattern Generator mode
pub type PGM_R = crate::BitReader;
///Field `PGM` writer - Pattern Generator mode
pub type PGM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PGO` reader - Pattern Generator Orientation
pub type PGO_R = crate::BitReader;
///Field `PGO` writer - Pattern Generator Orientation
pub type PGO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Video mode Type
    #[inline(always)]
    pub fn vmt(&self) -> VMT_R {
        VMT_R::new((self.bits & 3) as u8)
    }
    ///Bit 8 - Low-Power Vertical Sync Active Enable
    #[inline(always)]
    pub fn lpvsae(&self) -> LPVSAE_R {
        LPVSAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Low-power Vertical Back-Porch Enable
    #[inline(always)]
    pub fn lpvbpe(&self) -> LPVBPE_R {
        LPVBPE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Low-power Vertical Front-porch Enable
    #[inline(always)]
    pub fn lpvfpe(&self) -> LPVFPE_R {
        LPVFPE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Low-Power Vertical Active Enable
    #[inline(always)]
    pub fn lpvae(&self) -> LPVAE_R {
        LPVAE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Low-Power Horizontal Back-Porch Enable
    #[inline(always)]
    pub fn lphbpe(&self) -> LPHBPE_R {
        LPHBPE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Low-Power Horizontal Front-Porch Enable
    #[inline(always)]
    pub fn lphfpe(&self) -> LPHFPE_R {
        LPHFPE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Frame Bus-Turn-Around Acknowledge Enable
    #[inline(always)]
    pub fn fbtaae(&self) -> FBTAAE_R {
        FBTAAE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Low-Power Command Enable
    #[inline(always)]
    pub fn lpce(&self) -> LPCE_R {
        LPCE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Pattern Generator Enable
    #[inline(always)]
    pub fn pge(&self) -> PGE_R {
        PGE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - Pattern Generator mode
    #[inline(always)]
    pub fn pgm(&self) -> PGM_R {
        PGM_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - Pattern Generator Orientation
    #[inline(always)]
    pub fn pgo(&self) -> PGO_R {
        PGO_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VMCR")
            .field("vmt", &self.vmt())
            .field("lpvsae", &self.lpvsae())
            .field("lpvbpe", &self.lpvbpe())
            .field("lpvfpe", &self.lpvfpe())
            .field("lpvae", &self.lpvae())
            .field("lphbpe", &self.lphbpe())
            .field("lphfpe", &self.lphfpe())
            .field("fbtaae", &self.fbtaae())
            .field("lpce", &self.lpce())
            .field("pge", &self.pge())
            .field("pgm", &self.pgm())
            .field("pgo", &self.pgo())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Video mode Type
    #[inline(always)]
    pub fn vmt(&mut self) -> VMT_W<'_, VMCRrs> {
        VMT_W::new(self, 0)
    }
    ///Bit 8 - Low-Power Vertical Sync Active Enable
    #[inline(always)]
    pub fn lpvsae(&mut self) -> LPVSAE_W<'_, VMCRrs> {
        LPVSAE_W::new(self, 8)
    }
    ///Bit 9 - Low-power Vertical Back-Porch Enable
    #[inline(always)]
    pub fn lpvbpe(&mut self) -> LPVBPE_W<'_, VMCRrs> {
        LPVBPE_W::new(self, 9)
    }
    ///Bit 10 - Low-power Vertical Front-porch Enable
    #[inline(always)]
    pub fn lpvfpe(&mut self) -> LPVFPE_W<'_, VMCRrs> {
        LPVFPE_W::new(self, 10)
    }
    ///Bit 11 - Low-Power Vertical Active Enable
    #[inline(always)]
    pub fn lpvae(&mut self) -> LPVAE_W<'_, VMCRrs> {
        LPVAE_W::new(self, 11)
    }
    ///Bit 12 - Low-Power Horizontal Back-Porch Enable
    #[inline(always)]
    pub fn lphbpe(&mut self) -> LPHBPE_W<'_, VMCRrs> {
        LPHBPE_W::new(self, 12)
    }
    ///Bit 13 - Low-Power Horizontal Front-Porch Enable
    #[inline(always)]
    pub fn lphfpe(&mut self) -> LPHFPE_W<'_, VMCRrs> {
        LPHFPE_W::new(self, 13)
    }
    ///Bit 14 - Frame Bus-Turn-Around Acknowledge Enable
    #[inline(always)]
    pub fn fbtaae(&mut self) -> FBTAAE_W<'_, VMCRrs> {
        FBTAAE_W::new(self, 14)
    }
    ///Bit 15 - Low-Power Command Enable
    #[inline(always)]
    pub fn lpce(&mut self) -> LPCE_W<'_, VMCRrs> {
        LPCE_W::new(self, 15)
    }
    ///Bit 16 - Pattern Generator Enable
    #[inline(always)]
    pub fn pge(&mut self) -> PGE_W<'_, VMCRrs> {
        PGE_W::new(self, 16)
    }
    ///Bit 20 - Pattern Generator mode
    #[inline(always)]
    pub fn pgm(&mut self) -> PGM_W<'_, VMCRrs> {
        PGM_W::new(self, 20)
    }
    ///Bit 24 - Pattern Generator Orientation
    #[inline(always)]
    pub fn pgo(&mut self) -> PGO_W<'_, VMCRrs> {
        PGO_W::new(self, 24)
    }
}
/**DSI Host Video mode Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F779.html#DSI:VMCR)*/
pub struct VMCRrs;
impl crate::RegisterSpec for VMCRrs {
    type Ux = u32;
}
///`read()` method returns [`vmcr::R`](R) reader structure
impl crate::Readable for VMCRrs {}
///`write(|w| ..)` method takes [`vmcr::W`](W) writer structure
impl crate::Writable for VMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VMCR to value 0
impl crate::Resettable for VMCRrs {}
