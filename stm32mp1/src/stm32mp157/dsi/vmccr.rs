#[doc = "Register `VMCCR` reader"]
pub type R = crate::R<VMCCRrs>;
#[doc = "Field `VMT` reader - VMT"]
pub type VMT_R = crate::FieldReader;
#[doc = "Field `LPVSAE` reader - LPVSAE"]
pub type LPVSAE_R = crate::BitReader;
#[doc = "Field `LPVBPE` reader - LPVBPE"]
pub type LPVBPE_R = crate::BitReader;
#[doc = "Field `LPVFPE` reader - LPVFPE"]
pub type LPVFPE_R = crate::BitReader;
#[doc = "Field `LPVAE` reader - LPVAE"]
pub type LPVAE_R = crate::BitReader;
#[doc = "Field `LPHBPE` reader - LPHBPE"]
pub type LPHBPE_R = crate::BitReader;
#[doc = "Field `LPHFE` reader - LPHFE"]
pub type LPHFE_R = crate::BitReader;
#[doc = "Field `FBTAAE` reader - FBTAAE"]
pub type FBTAAE_R = crate::BitReader;
#[doc = "Field `LPCE` reader - LPCE"]
pub type LPCE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - VMT"]
    #[inline(always)]
    pub fn vmt(&self) -> VMT_R {
        VMT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - LPVSAE"]
    #[inline(always)]
    pub fn lpvsae(&self) -> LPVSAE_R {
        LPVSAE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LPVBPE"]
    #[inline(always)]
    pub fn lpvbpe(&self) -> LPVBPE_R {
        LPVBPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LPVFPE"]
    #[inline(always)]
    pub fn lpvfpe(&self) -> LPVFPE_R {
        LPVFPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LPVAE"]
    #[inline(always)]
    pub fn lpvae(&self) -> LPVAE_R {
        LPVAE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LPHBPE"]
    #[inline(always)]
    pub fn lphbpe(&self) -> LPHBPE_R {
        LPHBPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LPHFE"]
    #[inline(always)]
    pub fn lphfe(&self) -> LPHFE_R {
        LPHFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FBTAAE"]
    #[inline(always)]
    pub fn fbtaae(&self) -> FBTAAE_R {
        FBTAAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LPCE"]
    #[inline(always)]
    pub fn lpce(&self) -> LPCE_R {
        LPCE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "DSI Host video mode current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VMCCRrs;
impl crate::RegisterSpec for VMCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vmccr::R`](R) reader structure"]
impl crate::Readable for VMCCRrs {}
#[doc = "`reset()` method sets VMCCR to value 0"]
impl crate::Resettable for VMCCRrs {
    const RESET_VALUE: u32 = 0;
}
