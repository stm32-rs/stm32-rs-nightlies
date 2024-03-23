#[doc = "Register `VMCCR` reader"]
pub type R = crate::R<VMCCRrs>;
#[doc = "Register `VMCCR` writer"]
pub type W = crate::W<VMCCRrs>;
#[doc = "Field `VMT` reader - Video mode Type"]
pub type VMT_R = crate::FieldReader;
#[doc = "Field `VMT` writer - Video mode Type"]
pub type VMT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPVSAE` reader - Low-Power Vertical Sync time Enable"]
pub type LPVSAE_R = crate::BitReader;
#[doc = "Field `LPVSAE` writer - Low-Power Vertical Sync time Enable"]
pub type LPVSAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPVBPE` reader - Low-power Vertical Back-Porch Enable"]
pub type LPVBPE_R = crate::BitReader;
#[doc = "Field `LPVBPE` writer - Low-power Vertical Back-Porch Enable"]
pub type LPVBPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPVFPE` reader - Low-power Vertical Front-Porch Enable"]
pub type LPVFPE_R = crate::BitReader;
#[doc = "Field `LPVFPE` writer - Low-power Vertical Front-Porch Enable"]
pub type LPVFPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVAE` reader - Low-Power Vertical Active Enable"]
pub type LVAE_R = crate::BitReader;
#[doc = "Field `LVAE` writer - Low-Power Vertical Active Enable"]
pub type LVAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPHBPE` reader - Low-power Horizontal Back-Porch Enable"]
pub type LPHBPE_R = crate::BitReader;
#[doc = "Field `LPHBPE` writer - Low-power Horizontal Back-Porch Enable"]
pub type LPHBPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPHFE` reader - Low-Power Horizontal Front-Porch Enable"]
pub type LPHFE_R = crate::BitReader;
#[doc = "Field `LPHFE` writer - Low-Power Horizontal Front-Porch Enable"]
pub type LPHFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBTAAE` reader - Frame BTA Acknowledge Enable"]
pub type FBTAAE_R = crate::BitReader;
#[doc = "Field `FBTAAE` writer - Frame BTA Acknowledge Enable"]
pub type FBTAAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCE` reader - Low-Power Command Enable"]
pub type LPCE_R = crate::BitReader;
#[doc = "Field `LPCE` writer - Low-Power Command Enable"]
pub type LPCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Video mode Type"]
    #[inline(always)]
    pub fn vmt(&self) -> VMT_R {
        VMT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Low-Power Vertical Sync time Enable"]
    #[inline(always)]
    pub fn lpvsae(&self) -> LPVSAE_R {
        LPVSAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Low-power Vertical Back-Porch Enable"]
    #[inline(always)]
    pub fn lpvbpe(&self) -> LPVBPE_R {
        LPVBPE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Low-power Vertical Front-Porch Enable"]
    #[inline(always)]
    pub fn lpvfpe(&self) -> LPVFPE_R {
        LPVFPE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Low-Power Vertical Active Enable"]
    #[inline(always)]
    pub fn lvae(&self) -> LVAE_R {
        LVAE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Low-power Horizontal Back-Porch Enable"]
    #[inline(always)]
    pub fn lphbpe(&self) -> LPHBPE_R {
        LPHBPE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Low-Power Horizontal Front-Porch Enable"]
    #[inline(always)]
    pub fn lphfe(&self) -> LPHFE_R {
        LPHFE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Frame BTA Acknowledge Enable"]
    #[inline(always)]
    pub fn fbtaae(&self) -> FBTAAE_R {
        FBTAAE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Low-Power Command Enable"]
    #[inline(always)]
    pub fn lpce(&self) -> LPCE_R {
        LPCE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Video mode Type"]
    #[inline(always)]
    #[must_use]
    pub fn vmt(&mut self) -> VMT_W<VMCCRrs> {
        VMT_W::new(self, 0)
    }
    #[doc = "Bit 8 - Low-Power Vertical Sync time Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpvsae(&mut self) -> LPVSAE_W<VMCCRrs> {
        LPVSAE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Low-power Vertical Back-Porch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpvbpe(&mut self) -> LPVBPE_W<VMCCRrs> {
        LPVBPE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Low-power Vertical Front-Porch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpvfpe(&mut self) -> LPVFPE_W<VMCCRrs> {
        LPVFPE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Low-Power Vertical Active Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvae(&mut self) -> LVAE_W<VMCCRrs> {
        LVAE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Low-power Horizontal Back-Porch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lphbpe(&mut self) -> LPHBPE_W<VMCCRrs> {
        LPHBPE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Low-Power Horizontal Front-Porch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lphfe(&mut self) -> LPHFE_W<VMCCRrs> {
        LPHFE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Frame BTA Acknowledge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fbtaae(&mut self) -> FBTAAE_W<VMCCRrs> {
        FBTAAE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Low-Power Command Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpce(&mut self) -> LPCE_W<VMCCRrs> {
        LPCE_W::new(self, 15)
    }
}
#[doc = "DSI Host Video mode Current Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
