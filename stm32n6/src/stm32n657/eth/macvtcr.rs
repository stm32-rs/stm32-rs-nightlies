///Register `MACVTCR` reader
pub type R = crate::R<MACVTCRrs>;
///Register `MACVTCR` writer
pub type W = crate::W<MACVTCRrs>;
///Field `OB` reader - Operation Busy
pub type OB_R = crate::BitReader;
///Field `OB` writer - Operation Busy
pub type OB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CT` reader - Command Type
pub type CT_R = crate::BitReader;
///Field `CT` writer - Command Type
pub type CT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OFS` reader - Offset
pub type OFS_R = crate::FieldReader;
///Field `OFS` writer - Offset
pub type OFS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ETV` reader - Enable 12-Bit VLAN Tag Comparison
pub type ETV_R = crate::BitReader;
///Field `ETV` writer - Enable 12-Bit VLAN Tag Comparison
pub type ETV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VTIM` reader - VLAN Tag Inverse Match Enable
pub type VTIM_R = crate::BitReader;
///Field `VTIM` writer - VLAN Tag Inverse Match Enable
pub type VTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ESVL` reader - Enable S-VLAN
pub type ESVL_R = crate::BitReader;
///Field `ESVL` writer - Enable S-VLAN
pub type ESVL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERSVLM` reader - Enable Receive S-VLAN Match
pub type ERSVLM_R = crate::BitReader;
///Field `ERSVLM` writer - Enable Receive S-VLAN Match
pub type ERSVLM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DOVLTC` reader - Disable VLAN Type Check
pub type DOVLTC_R = crate::BitReader;
///Field `DOVLTC` writer - Disable VLAN Type Check
pub type DOVLTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EVLS` reader - Enable VLAN Tag Stripping on Receive
pub type EVLS_R = crate::FieldReader;
///Field `EVLS` writer - Enable VLAN Tag Stripping on Receive
pub type EVLS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EVLRXS` reader - Enable VLAN Tag in Rx status
pub type EVLRXS_R = crate::BitReader;
///Field `EVLRXS` writer - Enable VLAN Tag in Rx status
pub type EVLRXS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VTHM` reader - VLAN Tag Hash Table Match Enable
pub type VTHM_R = crate::BitReader;
///Field `VTHM` writer - VLAN Tag Hash Table Match Enable
pub type VTHM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EDVLP` reader - Enable Double VLAN Processing
pub type EDVLP_R = crate::BitReader;
///Field `EDVLP` writer - Enable Double VLAN Processing
pub type EDVLP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERIVLT` reader - Enable Inner VLAN Tag
pub type ERIVLT_R = crate::BitReader;
///Field `ERIVLT` writer - Enable Inner VLAN Tag
pub type ERIVLT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EIVLS` reader - Enable Inner VLAN Tag Stripping on Receive
pub type EIVLS_R = crate::FieldReader;
///Field `EIVLS` writer - Enable Inner VLAN Tag Stripping on Receive
pub type EIVLS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EIVLRXS` reader - Enable Inner VLAN Tag in Rx Status
pub type EIVLRXS_R = crate::BitReader;
///Field `EIVLRXS` writer - Enable Inner VLAN Tag in Rx Status
pub type EIVLRXS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Operation Busy
    #[inline(always)]
    pub fn ob(&self) -> OB_R {
        OB_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Command Type
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Offset
    #[inline(always)]
    pub fn ofs(&self) -> OFS_R {
        OFS_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 16 - Enable 12-Bit VLAN Tag Comparison
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - VLAN Tag Inverse Match Enable
    #[inline(always)]
    pub fn vtim(&self) -> VTIM_R {
        VTIM_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Enable S-VLAN
    #[inline(always)]
    pub fn esvl(&self) -> ESVL_R {
        ESVL_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Enable Receive S-VLAN Match
    #[inline(always)]
    pub fn ersvlm(&self) -> ERSVLM_R {
        ERSVLM_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Disable VLAN Type Check
    #[inline(always)]
    pub fn dovltc(&self) -> DOVLTC_R {
        DOVLTC_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - Enable VLAN Tag Stripping on Receive
    #[inline(always)]
    pub fn evls(&self) -> EVLS_R {
        EVLS_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 24 - Enable VLAN Tag in Rx status
    #[inline(always)]
    pub fn evlrxs(&self) -> EVLRXS_R {
        EVLRXS_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - VLAN Tag Hash Table Match Enable
    #[inline(always)]
    pub fn vthm(&self) -> VTHM_R {
        VTHM_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Enable Double VLAN Processing
    #[inline(always)]
    pub fn edvlp(&self) -> EDVLP_R {
        EDVLP_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Enable Inner VLAN Tag
    #[inline(always)]
    pub fn erivlt(&self) -> ERIVLT_R {
        ERIVLT_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:29 - Enable Inner VLAN Tag Stripping on Receive
    #[inline(always)]
    pub fn eivls(&self) -> EIVLS_R {
        EIVLS_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 31 - Enable Inner VLAN Tag in Rx Status
    #[inline(always)]
    pub fn eivlrxs(&self) -> EIVLRXS_R {
        EIVLRXS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACVTCR")
            .field("ob", &self.ob())
            .field("ct", &self.ct())
            .field("ofs", &self.ofs())
            .field("etv", &self.etv())
            .field("vtim", &self.vtim())
            .field("esvl", &self.esvl())
            .field("ersvlm", &self.ersvlm())
            .field("dovltc", &self.dovltc())
            .field("evls", &self.evls())
            .field("evlrxs", &self.evlrxs())
            .field("vthm", &self.vthm())
            .field("edvlp", &self.edvlp())
            .field("erivlt", &self.erivlt())
            .field("eivls", &self.eivls())
            .field("eivlrxs", &self.eivlrxs())
            .finish()
    }
}
impl W {
    ///Bit 0 - Operation Busy
    #[inline(always)]
    pub fn ob(&mut self) -> OB_W<'_, MACVTCRrs> {
        OB_W::new(self, 0)
    }
    ///Bit 1 - Command Type
    #[inline(always)]
    pub fn ct(&mut self) -> CT_W<'_, MACVTCRrs> {
        CT_W::new(self, 1)
    }
    ///Bits 2:3 - Offset
    #[inline(always)]
    pub fn ofs(&mut self) -> OFS_W<'_, MACVTCRrs> {
        OFS_W::new(self, 2)
    }
    ///Bit 16 - Enable 12-Bit VLAN Tag Comparison
    #[inline(always)]
    pub fn etv(&mut self) -> ETV_W<'_, MACVTCRrs> {
        ETV_W::new(self, 16)
    }
    ///Bit 17 - VLAN Tag Inverse Match Enable
    #[inline(always)]
    pub fn vtim(&mut self) -> VTIM_W<'_, MACVTCRrs> {
        VTIM_W::new(self, 17)
    }
    ///Bit 18 - Enable S-VLAN
    #[inline(always)]
    pub fn esvl(&mut self) -> ESVL_W<'_, MACVTCRrs> {
        ESVL_W::new(self, 18)
    }
    ///Bit 19 - Enable Receive S-VLAN Match
    #[inline(always)]
    pub fn ersvlm(&mut self) -> ERSVLM_W<'_, MACVTCRrs> {
        ERSVLM_W::new(self, 19)
    }
    ///Bit 20 - Disable VLAN Type Check
    #[inline(always)]
    pub fn dovltc(&mut self) -> DOVLTC_W<'_, MACVTCRrs> {
        DOVLTC_W::new(self, 20)
    }
    ///Bits 21:22 - Enable VLAN Tag Stripping on Receive
    #[inline(always)]
    pub fn evls(&mut self) -> EVLS_W<'_, MACVTCRrs> {
        EVLS_W::new(self, 21)
    }
    ///Bit 24 - Enable VLAN Tag in Rx status
    #[inline(always)]
    pub fn evlrxs(&mut self) -> EVLRXS_W<'_, MACVTCRrs> {
        EVLRXS_W::new(self, 24)
    }
    ///Bit 25 - VLAN Tag Hash Table Match Enable
    #[inline(always)]
    pub fn vthm(&mut self) -> VTHM_W<'_, MACVTCRrs> {
        VTHM_W::new(self, 25)
    }
    ///Bit 26 - Enable Double VLAN Processing
    #[inline(always)]
    pub fn edvlp(&mut self) -> EDVLP_W<'_, MACVTCRrs> {
        EDVLP_W::new(self, 26)
    }
    ///Bit 27 - Enable Inner VLAN Tag
    #[inline(always)]
    pub fn erivlt(&mut self) -> ERIVLT_W<'_, MACVTCRrs> {
        ERIVLT_W::new(self, 27)
    }
    ///Bits 28:29 - Enable Inner VLAN Tag Stripping on Receive
    #[inline(always)]
    pub fn eivls(&mut self) -> EIVLS_W<'_, MACVTCRrs> {
        EIVLS_W::new(self, 28)
    }
    ///Bit 31 - Enable Inner VLAN Tag in Rx Status
    #[inline(always)]
    pub fn eivlrxs(&mut self) -> EIVLRXS_W<'_, MACVTCRrs> {
        EIVLRXS_W::new(self, 31)
    }
}
/**VLAN tag Control register

You can [`read`](crate::Reg::read) this register and get [`macvtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ETH:MACVTCR)*/
pub struct MACVTCRrs;
impl crate::RegisterSpec for MACVTCRrs {
    type Ux = u32;
}
///`read()` method returns [`macvtcr::R`](R) reader structure
impl crate::Readable for MACVTCRrs {}
///`write(|w| ..)` method takes [`macvtcr::W`](W) writer structure
impl crate::Writable for MACVTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACVTCR to value 0
impl crate::Resettable for MACVTCRrs {}
