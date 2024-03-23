#[doc = "Register `VMCR` reader"]
pub type R = crate::R<VMCRrs>;
#[doc = "Register `VMCR` writer"]
pub type W = crate::W<VMCRrs>;
#[doc = "Field `VMT` reader - Video mode Type"]
pub type VMT_R = crate::FieldReader;
#[doc = "Field `VMT` writer - Video mode Type"]
pub type VMT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPVSAE` reader - Low-Power Vertical Sync Active Enable"]
pub type LPVSAE_R = crate::BitReader;
#[doc = "Field `LPVSAE` writer - Low-Power Vertical Sync Active Enable"]
pub type LPVSAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPVBPE` reader - Low-power Vertical Back-Porch Enable"]
pub type LPVBPE_R = crate::BitReader;
#[doc = "Field `LPVBPE` writer - Low-power Vertical Back-Porch Enable"]
pub type LPVBPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPVFPE` reader - Low-power Vertical Front-porch Enable"]
pub type LPVFPE_R = crate::BitReader;
#[doc = "Field `LPVFPE` writer - Low-power Vertical Front-porch Enable"]
pub type LPVFPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPVAE` reader - Low-Power Vertical Active Enable"]
pub type LPVAE_R = crate::BitReader;
#[doc = "Field `LPVAE` writer - Low-Power Vertical Active Enable"]
pub type LPVAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPHBPE` reader - Low-Power Horizontal Back-Porch Enable"]
pub type LPHBPE_R = crate::BitReader;
#[doc = "Field `LPHBPE` writer - Low-Power Horizontal Back-Porch Enable"]
pub type LPHBPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPHFPE` reader - Low-Power Horizontal Front-Porch Enable"]
pub type LPHFPE_R = crate::BitReader;
#[doc = "Field `LPHFPE` writer - Low-Power Horizontal Front-Porch Enable"]
pub type LPHFPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBTAAE` reader - Frame Bus-Turn-Around Acknowledge Enable"]
pub type FBTAAE_R = crate::BitReader;
#[doc = "Field `FBTAAE` writer - Frame Bus-Turn-Around Acknowledge Enable"]
pub type FBTAAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCE` reader - Low-Power Command Enable"]
pub type LPCE_R = crate::BitReader;
#[doc = "Field `LPCE` writer - Low-Power Command Enable"]
pub type LPCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGE` reader - Pattern Generator Enable"]
pub type PGE_R = crate::BitReader;
#[doc = "Field `PGE` writer - Pattern Generator Enable"]
pub type PGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGM` reader - Pattern Generator mode"]
pub type PGM_R = crate::BitReader;
#[doc = "Field `PGM` writer - Pattern Generator mode"]
pub type PGM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGO` reader - Pattern Generator Orientation"]
pub type PGO_R = crate::BitReader;
#[doc = "Field `PGO` writer - Pattern Generator Orientation"]
pub type PGO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Video mode Type"]
    #[inline(always)]
    pub fn vmt(&self) -> VMT_R {
        VMT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Low-Power Vertical Sync Active Enable"]
    #[inline(always)]
    pub fn lpvsae(&self) -> LPVSAE_R {
        LPVSAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Low-power Vertical Back-Porch Enable"]
    #[inline(always)]
    pub fn lpvbpe(&self) -> LPVBPE_R {
        LPVBPE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Low-power Vertical Front-porch Enable"]
    #[inline(always)]
    pub fn lpvfpe(&self) -> LPVFPE_R {
        LPVFPE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Low-Power Vertical Active Enable"]
    #[inline(always)]
    pub fn lpvae(&self) -> LPVAE_R {
        LPVAE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Low-Power Horizontal Back-Porch Enable"]
    #[inline(always)]
    pub fn lphbpe(&self) -> LPHBPE_R {
        LPHBPE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Low-Power Horizontal Front-Porch Enable"]
    #[inline(always)]
    pub fn lphfpe(&self) -> LPHFPE_R {
        LPHFPE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Frame Bus-Turn-Around Acknowledge Enable"]
    #[inline(always)]
    pub fn fbtaae(&self) -> FBTAAE_R {
        FBTAAE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Low-Power Command Enable"]
    #[inline(always)]
    pub fn lpce(&self) -> LPCE_R {
        LPCE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Pattern Generator Enable"]
    #[inline(always)]
    pub fn pge(&self) -> PGE_R {
        PGE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Pattern Generator mode"]
    #[inline(always)]
    pub fn pgm(&self) -> PGM_R {
        PGM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Pattern Generator Orientation"]
    #[inline(always)]
    pub fn pgo(&self) -> PGO_R {
        PGO_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Video mode Type"]
    #[inline(always)]
    #[must_use]
    pub fn vmt(&mut self) -> VMT_W<VMCRrs> {
        VMT_W::new(self, 0)
    }
    #[doc = "Bit 8 - Low-Power Vertical Sync Active Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpvsae(&mut self) -> LPVSAE_W<VMCRrs> {
        LPVSAE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Low-power Vertical Back-Porch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpvbpe(&mut self) -> LPVBPE_W<VMCRrs> {
        LPVBPE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Low-power Vertical Front-porch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpvfpe(&mut self) -> LPVFPE_W<VMCRrs> {
        LPVFPE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Low-Power Vertical Active Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpvae(&mut self) -> LPVAE_W<VMCRrs> {
        LPVAE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Low-Power Horizontal Back-Porch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lphbpe(&mut self) -> LPHBPE_W<VMCRrs> {
        LPHBPE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Low-Power Horizontal Front-Porch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lphfpe(&mut self) -> LPHFPE_W<VMCRrs> {
        LPHFPE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Frame Bus-Turn-Around Acknowledge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fbtaae(&mut self) -> FBTAAE_W<VMCRrs> {
        FBTAAE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Low-Power Command Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpce(&mut self) -> LPCE_W<VMCRrs> {
        LPCE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Pattern Generator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pge(&mut self) -> PGE_W<VMCRrs> {
        PGE_W::new(self, 16)
    }
    #[doc = "Bit 20 - Pattern Generator mode"]
    #[inline(always)]
    #[must_use]
    pub fn pgm(&mut self) -> PGM_W<VMCRrs> {
        PGM_W::new(self, 20)
    }
    #[doc = "Bit 24 - Pattern Generator Orientation"]
    #[inline(always)]
    #[must_use]
    pub fn pgo(&mut self) -> PGO_W<VMCRrs> {
        PGO_W::new(self, 24)
    }
}
#[doc = "DSI Host Video mode Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VMCRrs;
impl crate::RegisterSpec for VMCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vmcr::R`](R) reader structure"]
impl crate::Readable for VMCRrs {}
#[doc = "`write(|w| ..)` method takes [`vmcr::W`](W) writer structure"]
impl crate::Writable for VMCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VMCR to value 0"]
impl crate::Resettable for VMCRrs {
    const RESET_VALUE: u32 = 0;
}
