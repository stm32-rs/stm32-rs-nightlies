#[doc = "Register `MACVTR` reader"]
pub type R = crate::R<MACVTRrs>;
#[doc = "Register `MACVTR` writer"]
pub type W = crate::W<MACVTRrs>;
#[doc = "Field `VL` reader - VLAN Tag Identifier for Receive Packets"]
pub type VL_R = crate::FieldReader<u16>;
#[doc = "Field `VL` writer - VLAN Tag Identifier for Receive Packets"]
pub type VL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ETV` reader - Enable 12-Bit VLAN Tag Comparison"]
pub type ETV_R = crate::BitReader;
#[doc = "Field `ETV` writer - Enable 12-Bit VLAN Tag Comparison"]
pub type ETV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTIM` reader - VLAN Tag Inverse Match Enable"]
pub type VTIM_R = crate::BitReader;
#[doc = "Field `VTIM` writer - VLAN Tag Inverse Match Enable"]
pub type VTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESVL` reader - Enable S-VLAN"]
pub type ESVL_R = crate::BitReader;
#[doc = "Field `ESVL` writer - Enable S-VLAN"]
pub type ESVL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERSVLM` reader - Enable Receive S-VLAN Match"]
pub type ERSVLM_R = crate::BitReader;
#[doc = "Field `ERSVLM` writer - Enable Receive S-VLAN Match"]
pub type ERSVLM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOVLTC` reader - Disable VLAN Type Check"]
pub type DOVLTC_R = crate::BitReader;
#[doc = "Field `DOVLTC` writer - Disable VLAN Type Check"]
pub type DOVLTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVLS` reader - Enable VLAN Tag Stripping on Receive"]
pub type EVLS_R = crate::FieldReader;
#[doc = "Field `EVLS` writer - Enable VLAN Tag Stripping on Receive"]
pub type EVLS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EVLRXS` reader - Enable VLAN Tag in Rx status"]
pub type EVLRXS_R = crate::BitReader;
#[doc = "Field `EVLRXS` writer - Enable VLAN Tag in Rx status"]
pub type EVLRXS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTHM` reader - VLAN Tag Hash Table Match Enable"]
pub type VTHM_R = crate::BitReader;
#[doc = "Field `VTHM` writer - VLAN Tag Hash Table Match Enable"]
pub type VTHM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDVLP` reader - Enable Double VLAN Processing"]
pub type EDVLP_R = crate::BitReader;
#[doc = "Field `EDVLP` writer - Enable Double VLAN Processing"]
pub type EDVLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERIVLT` reader - Enable Inner VLAN Tag"]
pub type ERIVLT_R = crate::BitReader;
#[doc = "Field `ERIVLT` writer - Enable Inner VLAN Tag"]
pub type ERIVLT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIVLS` reader - Enable Inner VLAN Tag Stripping on Receive"]
pub type EIVLS_R = crate::FieldReader;
#[doc = "Field `EIVLS` writer - Enable Inner VLAN Tag Stripping on Receive"]
pub type EIVLS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EIVLRXS` reader - Enable Inner VLAN Tag in Rx Status"]
pub type EIVLRXS_R = crate::BitReader;
#[doc = "Field `EIVLRXS` writer - Enable Inner VLAN Tag in Rx Status"]
pub type EIVLRXS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - VLAN Tag Identifier for Receive Packets"]
    #[inline(always)]
    pub fn vl(&self) -> VL_R {
        VL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison"]
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable"]
    #[inline(always)]
    pub fn vtim(&self) -> VTIM_R {
        VTIM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable S-VLAN"]
    #[inline(always)]
    pub fn esvl(&self) -> ESVL_R {
        ESVL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Receive S-VLAN Match"]
    #[inline(always)]
    pub fn ersvlm(&self) -> ERSVLM_R {
        ERSVLM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Disable VLAN Type Check"]
    #[inline(always)]
    pub fn dovltc(&self) -> DOVLTC_R {
        DOVLTC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Enable VLAN Tag Stripping on Receive"]
    #[inline(always)]
    pub fn evls(&self) -> EVLS_R {
        EVLS_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 24 - Enable VLAN Tag in Rx status"]
    #[inline(always)]
    pub fn evlrxs(&self) -> EVLRXS_R {
        EVLRXS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - VLAN Tag Hash Table Match Enable"]
    #[inline(always)]
    pub fn vthm(&self) -> VTHM_R {
        VTHM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Double VLAN Processing"]
    #[inline(always)]
    pub fn edvlp(&self) -> EDVLP_R {
        EDVLP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Inner VLAN Tag"]
    #[inline(always)]
    pub fn erivlt(&self) -> ERIVLT_R {
        ERIVLT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Enable Inner VLAN Tag Stripping on Receive"]
    #[inline(always)]
    pub fn eivls(&self) -> EIVLS_R {
        EIVLS_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - Enable Inner VLAN Tag in Rx Status"]
    #[inline(always)]
    pub fn eivlrxs(&self) -> EIVLRXS_R {
        EIVLRXS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Tag Identifier for Receive Packets"]
    #[inline(always)]
    #[must_use]
    pub fn vl(&mut self) -> VL_W<MACVTRrs> {
        VL_W::new(self, 0)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison"]
    #[inline(always)]
    #[must_use]
    pub fn etv(&mut self) -> ETV_W<MACVTRrs> {
        ETV_W::new(self, 16)
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vtim(&mut self) -> VTIM_W<MACVTRrs> {
        VTIM_W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable S-VLAN"]
    #[inline(always)]
    #[must_use]
    pub fn esvl(&mut self) -> ESVL_W<MACVTRrs> {
        ESVL_W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Receive S-VLAN Match"]
    #[inline(always)]
    #[must_use]
    pub fn ersvlm(&mut self) -> ERSVLM_W<MACVTRrs> {
        ERSVLM_W::new(self, 19)
    }
    #[doc = "Bit 20 - Disable VLAN Type Check"]
    #[inline(always)]
    #[must_use]
    pub fn dovltc(&mut self) -> DOVLTC_W<MACVTRrs> {
        DOVLTC_W::new(self, 20)
    }
    #[doc = "Bits 21:22 - Enable VLAN Tag Stripping on Receive"]
    #[inline(always)]
    #[must_use]
    pub fn evls(&mut self) -> EVLS_W<MACVTRrs> {
        EVLS_W::new(self, 21)
    }
    #[doc = "Bit 24 - Enable VLAN Tag in Rx status"]
    #[inline(always)]
    #[must_use]
    pub fn evlrxs(&mut self) -> EVLRXS_W<MACVTRrs> {
        EVLRXS_W::new(self, 24)
    }
    #[doc = "Bit 25 - VLAN Tag Hash Table Match Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vthm(&mut self) -> VTHM_W<MACVTRrs> {
        VTHM_W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Double VLAN Processing"]
    #[inline(always)]
    #[must_use]
    pub fn edvlp(&mut self) -> EDVLP_W<MACVTRrs> {
        EDVLP_W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Inner VLAN Tag"]
    #[inline(always)]
    #[must_use]
    pub fn erivlt(&mut self) -> ERIVLT_W<MACVTRrs> {
        ERIVLT_W::new(self, 27)
    }
    #[doc = "Bits 28:29 - Enable Inner VLAN Tag Stripping on Receive"]
    #[inline(always)]
    #[must_use]
    pub fn eivls(&mut self) -> EIVLS_W<MACVTRrs> {
        EIVLS_W::new(self, 28)
    }
    #[doc = "Bit 31 - Enable Inner VLAN Tag in Rx Status"]
    #[inline(always)]
    #[must_use]
    pub fn eivlrxs(&mut self) -> EIVLRXS_W<MACVTRrs> {
        EIVLRXS_W::new(self, 31)
    }
}
#[doc = "VLAN tag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macvtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macvtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACVTRrs;
impl crate::RegisterSpec for MACVTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macvtr::R`](R) reader structure"]
impl crate::Readable for MACVTRrs {}
#[doc = "`write(|w| ..)` method takes [`macvtr::W`](W) writer structure"]
impl crate::Writable for MACVTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACVTR to value 0"]
impl crate::Resettable for MACVTRrs {
    const RESET_VALUE: u32 = 0;
}
