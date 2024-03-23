#[doc = "Register `VMCCR` reader"]
pub type R = crate::R<VMCCRrs>;
#[doc = "Register `VMCCR` writer"]
pub type W = crate::W<VMCCRrs>;
#[doc = "Field `VMT` reader - Video mode type"]
pub type VMT_R = crate::FieldReader;
#[doc = "Field `VMT` writer - Video mode type"]
pub type VMT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPVSAE` reader - Low-power vertical sync time enable"]
pub type LPVSAE_R = crate::BitReader;
#[doc = "Field `LPVSAE` writer - Low-power vertical sync time enable"]
pub type LPVSAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPVBPE` reader - Low-power vertical back-porch enable"]
pub type LPVBPE_R = crate::BitReader;
#[doc = "Field `LPVBPE` writer - Low-power vertical back-porch enable"]
pub type LPVBPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPVFPE` reader - Low-power vertical front-porch enable"]
pub type LPVFPE_R = crate::BitReader;
#[doc = "Field `LPVFPE` writer - Low-power vertical front-porch enable"]
pub type LPVFPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPVAE` reader - Low-power vertical active enable"]
pub type LPVAE_R = crate::BitReader;
#[doc = "Field `LPVAE` writer - Low-power vertical active enable"]
pub type LPVAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPHBPE` reader - Low-power horizontal back-porch enable"]
pub type LPHBPE_R = crate::BitReader;
#[doc = "Field `LPHBPE` writer - Low-power horizontal back-porch enable"]
pub type LPHBPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPHFE` reader - Low-power horizontal front-porch enable"]
pub type LPHFE_R = crate::BitReader;
#[doc = "Field `LPHFE` writer - Low-power horizontal front-porch enable"]
pub type LPHFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBTAAE` reader - Frame BTA acknowledge enable"]
pub type FBTAAE_R = crate::BitReader;
#[doc = "Field `FBTAAE` writer - Frame BTA acknowledge enable"]
pub type FBTAAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCE` reader - Low-power command enable"]
pub type LPCE_R = crate::BitReader;
#[doc = "Field `LPCE` writer - Low-power command enable"]
pub type LPCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Video mode type"]
    #[inline(always)]
    pub fn vmt(&self) -> VMT_R {
        VMT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Low-power vertical sync time enable"]
    #[inline(always)]
    pub fn lpvsae(&self) -> LPVSAE_R {
        LPVSAE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Low-power vertical back-porch enable"]
    #[inline(always)]
    pub fn lpvbpe(&self) -> LPVBPE_R {
        LPVBPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Low-power vertical front-porch enable"]
    #[inline(always)]
    pub fn lpvfpe(&self) -> LPVFPE_R {
        LPVFPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Low-power vertical active enable"]
    #[inline(always)]
    pub fn lpvae(&self) -> LPVAE_R {
        LPVAE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Low-power horizontal back-porch enable"]
    #[inline(always)]
    pub fn lphbpe(&self) -> LPHBPE_R {
        LPHBPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Low-power horizontal front-porch enable"]
    #[inline(always)]
    pub fn lphfe(&self) -> LPHFE_R {
        LPHFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Frame BTA acknowledge enable"]
    #[inline(always)]
    pub fn fbtaae(&self) -> FBTAAE_R {
        FBTAAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Low-power command enable"]
    #[inline(always)]
    pub fn lpce(&self) -> LPCE_R {
        LPCE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Video mode type"]
    #[inline(always)]
    #[must_use]
    pub fn vmt(&mut self) -> VMT_W<VMCCRrs> {
        VMT_W::new(self, 0)
    }
    #[doc = "Bit 2 - Low-power vertical sync time enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpvsae(&mut self) -> LPVSAE_W<VMCCRrs> {
        LPVSAE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Low-power vertical back-porch enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpvbpe(&mut self) -> LPVBPE_W<VMCCRrs> {
        LPVBPE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Low-power vertical front-porch enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpvfpe(&mut self) -> LPVFPE_W<VMCCRrs> {
        LPVFPE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Low-power vertical active enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpvae(&mut self) -> LPVAE_W<VMCCRrs> {
        LPVAE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Low-power horizontal back-porch enable"]
    #[inline(always)]
    #[must_use]
    pub fn lphbpe(&mut self) -> LPHBPE_W<VMCCRrs> {
        LPHBPE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Low-power horizontal front-porch enable"]
    #[inline(always)]
    #[must_use]
    pub fn lphfe(&mut self) -> LPHFE_W<VMCCRrs> {
        LPHFE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Frame BTA acknowledge enable"]
    #[inline(always)]
    #[must_use]
    pub fn fbtaae(&mut self) -> FBTAAE_W<VMCCRrs> {
        FBTAAE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Low-power command enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpce(&mut self) -> LPCE_W<VMCCRrs> {
        LPCE_W::new(self, 9)
    }
}
#[doc = "DSI Host video mode current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VMCCRrs;
impl crate::RegisterSpec for VMCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vmccr::R`](R) reader structure"]
impl crate::Readable for VMCCRrs {}
#[doc = "`write(|w| ..)` method takes [`vmccr::W`](W) writer structure"]
impl crate::Writable for VMCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VMCCR to value 0"]
impl crate::Resettable for VMCCRrs {
    const RESET_VALUE: u32 = 0;
}
