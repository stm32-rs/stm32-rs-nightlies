#[doc = "Reader of register DSI_VMCCR"]
pub type R = crate::R<u32, super::DSI_VMCCR>;
#[doc = "Reader of field `VMT`"]
pub type VMT_R = crate::R<u8, u8>;
#[doc = "Reader of field `LPVSAE`"]
pub type LPVSAE_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPVBPE`"]
pub type LPVBPE_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPVFPE`"]
pub type LPVFPE_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPVAE`"]
pub type LPVAE_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPHBPE`"]
pub type LPHBPE_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPHFE`"]
pub type LPHFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `FBTAAE`"]
pub type FBTAAE_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPCE`"]
pub type LPCE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:1 - Video mode Type"]
    #[inline(always)]
    pub fn vmt(&self) -> VMT_R {
        VMT_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Low-Power Vertical Sync time Enable"]
    #[inline(always)]
    pub fn lpvsae(&self) -> LPVSAE_R {
        LPVSAE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Low-power Vertical Back-Porch Enable"]
    #[inline(always)]
    pub fn lpvbpe(&self) -> LPVBPE_R {
        LPVBPE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Low-power Vertical Front-Porch Enable"]
    #[inline(always)]
    pub fn lpvfpe(&self) -> LPVFPE_R {
        LPVFPE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Low-Power Vertical Active Enable"]
    #[inline(always)]
    pub fn lpvae(&self) -> LPVAE_R {
        LPVAE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Low-power Horizontal Back-Porch Enable"]
    #[inline(always)]
    pub fn lphbpe(&self) -> LPHBPE_R {
        LPHBPE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Low-Power Horizontal Front-Porch Enable"]
    #[inline(always)]
    pub fn lphfe(&self) -> LPHFE_R {
        LPHFE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Frame BTA Acknowledge Enable"]
    #[inline(always)]
    pub fn fbtaae(&self) -> FBTAAE_R {
        FBTAAE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Low-Power Command Enable"]
    #[inline(always)]
    pub fn lpce(&self) -> LPCE_R {
        LPCE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
