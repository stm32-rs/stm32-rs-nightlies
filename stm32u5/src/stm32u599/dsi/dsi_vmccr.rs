#[doc = "Register `DSI_VMCCR` reader"]
pub type R = crate::R<DSI_VMCCRrs>;
#[doc = "Field `VMT` reader - Video mode type This field returns the current video mode transmission type: 1x: Burst mode"]
pub type VMT_R = crate::FieldReader;
#[doc = "Field `LPVSAE` reader - Low-power vertical sync time enable This bit returns the current state of return to low-power inside the vertical sync time (VSA) period when timing allows."]
pub type LPVSAE_R = crate::BitReader;
#[doc = "Field `LPVBPE` reader - Low-power vertical back-porch enable This bit returns the current state of return to low-power inside the vertical back-porch (VBP) period when timing allows."]
pub type LPVBPE_R = crate::BitReader;
#[doc = "Field `LPVFPE` reader - Low-power vertical front-porch enable This bit returns the current state of return to low-power inside the vertical front-porch (VFP) period when timing allows."]
pub type LPVFPE_R = crate::BitReader;
#[doc = "Field `LPVAE` reader - Low-power vertical active enable This bit returns the current state of return to low-power inside the vertical active (VACT) period when timing allows."]
pub type LPVAE_R = crate::BitReader;
#[doc = "Field `LPHBPE` reader - Low-power horizontal back-porch enable This bit returns the current state of return to low-power inside the horizontal back-porch (HBP) period when timing allows."]
pub type LPHBPE_R = crate::BitReader;
#[doc = "Field `LPHFE` reader - Low-power horizontal front-porch enable This bit returns the current state of return to low-power inside the horizontal front-porch (HFP) period when timing allows."]
pub type LPHFE_R = crate::BitReader;
#[doc = "Field `FBTAAE` reader - Frame BTA acknowledge enable This bit returns the current state of request for an acknowledge response at the end of a frame."]
pub type FBTAAE_R = crate::BitReader;
#[doc = "Field `LPCE` reader - Low-power command enable This bit returns the current command transmission state in low-power mode."]
pub type LPCE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Video mode type This field returns the current video mode transmission type: 1x: Burst mode"]
    #[inline(always)]
    pub fn vmt(&self) -> VMT_R {
        VMT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Low-power vertical sync time enable This bit returns the current state of return to low-power inside the vertical sync time (VSA) period when timing allows."]
    #[inline(always)]
    pub fn lpvsae(&self) -> LPVSAE_R {
        LPVSAE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Low-power vertical back-porch enable This bit returns the current state of return to low-power inside the vertical back-porch (VBP) period when timing allows."]
    #[inline(always)]
    pub fn lpvbpe(&self) -> LPVBPE_R {
        LPVBPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Low-power vertical front-porch enable This bit returns the current state of return to low-power inside the vertical front-porch (VFP) period when timing allows."]
    #[inline(always)]
    pub fn lpvfpe(&self) -> LPVFPE_R {
        LPVFPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Low-power vertical active enable This bit returns the current state of return to low-power inside the vertical active (VACT) period when timing allows."]
    #[inline(always)]
    pub fn lpvae(&self) -> LPVAE_R {
        LPVAE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Low-power horizontal back-porch enable This bit returns the current state of return to low-power inside the horizontal back-porch (HBP) period when timing allows."]
    #[inline(always)]
    pub fn lphbpe(&self) -> LPHBPE_R {
        LPHBPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Low-power horizontal front-porch enable This bit returns the current state of return to low-power inside the horizontal front-porch (HFP) period when timing allows."]
    #[inline(always)]
    pub fn lphfe(&self) -> LPHFE_R {
        LPHFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Frame BTA acknowledge enable This bit returns the current state of request for an acknowledge response at the end of a frame."]
    #[inline(always)]
    pub fn fbtaae(&self) -> FBTAAE_R {
        FBTAAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Low-power command enable This bit returns the current command transmission state in low-power mode."]
    #[inline(always)]
    pub fn lpce(&self) -> LPCE_R {
        LPCE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "DSI Host video mode current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vmccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VMCCRrs;
impl crate::RegisterSpec for DSI_VMCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vmccr::R`](R) reader structure"]
impl crate::Readable for DSI_VMCCRrs {}
#[doc = "`reset()` method sets DSI_VMCCR to value 0"]
impl crate::Resettable for DSI_VMCCRrs {
    const RESET_VALUE: u32 = 0;
}
