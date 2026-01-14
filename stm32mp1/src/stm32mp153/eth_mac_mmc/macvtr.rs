///Register `MACVTR` reader
pub type R = crate::R<MACVTRrs>;
///Register `MACVTR` writer
pub type W = crate::W<MACVTRrs>;
///Field `VL` reader - VL
pub type VL_R = crate::FieldReader<u16>;
///Field `VL` writer - VL
pub type VL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `ETV` reader - ETV
pub type ETV_R = crate::BitReader;
///Field `ETV` writer - ETV
pub type ETV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VTIM` reader - VTIM
pub type VTIM_R = crate::BitReader;
///Field `VTIM` writer - VTIM
pub type VTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ESVL` reader - ESVL
pub type ESVL_R = crate::BitReader;
///Field `ESVL` writer - ESVL
pub type ESVL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERSVLM` reader - ERSVLM
pub type ERSVLM_R = crate::BitReader;
///Field `ERSVLM` writer - ERSVLM
pub type ERSVLM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DOVLTC` reader - DOVLTC
pub type DOVLTC_R = crate::BitReader;
///Field `DOVLTC` writer - DOVLTC
pub type DOVLTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EVLS` reader - EVLS
pub type EVLS_R = crate::FieldReader;
///Field `EVLS` writer - EVLS
pub type EVLS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EVLRXS` reader - EVLRXS
pub type EVLRXS_R = crate::BitReader;
///Field `EVLRXS` writer - EVLRXS
pub type EVLRXS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VTHM` reader - VTHM
pub type VTHM_R = crate::BitReader;
///Field `VTHM` writer - VTHM
pub type VTHM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EDVLP` reader - EDVLP
pub type EDVLP_R = crate::BitReader;
///Field `EDVLP` writer - EDVLP
pub type EDVLP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERIVLT` reader - ERIVLT
pub type ERIVLT_R = crate::BitReader;
///Field `ERIVLT` writer - ERIVLT
pub type ERIVLT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EIVLS` reader - EIVLS
pub type EIVLS_R = crate::FieldReader;
///Field `EIVLS` writer - EIVLS
pub type EIVLS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EIVLRXS` reader - EIVLRXS
pub type EIVLRXS_R = crate::BitReader;
///Field `EIVLRXS` writer - EIVLRXS
pub type EIVLRXS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - VL
    #[inline(always)]
    pub fn vl(&self) -> VL_R {
        VL_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - ETV
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - VTIM
    #[inline(always)]
    pub fn vtim(&self) -> VTIM_R {
        VTIM_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ESVL
    #[inline(always)]
    pub fn esvl(&self) -> ESVL_R {
        ESVL_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - ERSVLM
    #[inline(always)]
    pub fn ersvlm(&self) -> ERSVLM_R {
        ERSVLM_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - DOVLTC
    #[inline(always)]
    pub fn dovltc(&self) -> DOVLTC_R {
        DOVLTC_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - EVLS
    #[inline(always)]
    pub fn evls(&self) -> EVLS_R {
        EVLS_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 24 - EVLRXS
    #[inline(always)]
    pub fn evlrxs(&self) -> EVLRXS_R {
        EVLRXS_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - VTHM
    #[inline(always)]
    pub fn vthm(&self) -> VTHM_R {
        VTHM_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - EDVLP
    #[inline(always)]
    pub fn edvlp(&self) -> EDVLP_R {
        EDVLP_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - ERIVLT
    #[inline(always)]
    pub fn erivlt(&self) -> ERIVLT_R {
        ERIVLT_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:29 - EIVLS
    #[inline(always)]
    pub fn eivls(&self) -> EIVLS_R {
        EIVLS_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 31 - EIVLRXS
    #[inline(always)]
    pub fn eivlrxs(&self) -> EIVLRXS_R {
        EIVLRXS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACVTR")
            .field("vl", &self.vl())
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
    ///Bits 0:15 - VL
    #[inline(always)]
    pub fn vl(&mut self) -> VL_W<'_, MACVTRrs> {
        VL_W::new(self, 0)
    }
    ///Bit 16 - ETV
    #[inline(always)]
    pub fn etv(&mut self) -> ETV_W<'_, MACVTRrs> {
        ETV_W::new(self, 16)
    }
    ///Bit 17 - VTIM
    #[inline(always)]
    pub fn vtim(&mut self) -> VTIM_W<'_, MACVTRrs> {
        VTIM_W::new(self, 17)
    }
    ///Bit 18 - ESVL
    #[inline(always)]
    pub fn esvl(&mut self) -> ESVL_W<'_, MACVTRrs> {
        ESVL_W::new(self, 18)
    }
    ///Bit 19 - ERSVLM
    #[inline(always)]
    pub fn ersvlm(&mut self) -> ERSVLM_W<'_, MACVTRrs> {
        ERSVLM_W::new(self, 19)
    }
    ///Bit 20 - DOVLTC
    #[inline(always)]
    pub fn dovltc(&mut self) -> DOVLTC_W<'_, MACVTRrs> {
        DOVLTC_W::new(self, 20)
    }
    ///Bits 21:22 - EVLS
    #[inline(always)]
    pub fn evls(&mut self) -> EVLS_W<'_, MACVTRrs> {
        EVLS_W::new(self, 21)
    }
    ///Bit 24 - EVLRXS
    #[inline(always)]
    pub fn evlrxs(&mut self) -> EVLRXS_W<'_, MACVTRrs> {
        EVLRXS_W::new(self, 24)
    }
    ///Bit 25 - VTHM
    #[inline(always)]
    pub fn vthm(&mut self) -> VTHM_W<'_, MACVTRrs> {
        VTHM_W::new(self, 25)
    }
    ///Bit 26 - EDVLP
    #[inline(always)]
    pub fn edvlp(&mut self) -> EDVLP_W<'_, MACVTRrs> {
        EDVLP_W::new(self, 26)
    }
    ///Bit 27 - ERIVLT
    #[inline(always)]
    pub fn erivlt(&mut self) -> ERIVLT_W<'_, MACVTRrs> {
        ERIVLT_W::new(self, 27)
    }
    ///Bits 28:29 - EIVLS
    #[inline(always)]
    pub fn eivls(&mut self) -> EIVLS_W<'_, MACVTRrs> {
        EIVLS_W::new(self, 28)
    }
    ///Bit 31 - EIVLRXS
    #[inline(always)]
    pub fn eivlrxs(&mut self) -> EIVLRXS_W<'_, MACVTRrs> {
        EIVLRXS_W::new(self, 31)
    }
}
/**The VLAN Tag register identifies the IEEE 802.1Q VLAN type packets.

You can [`read`](crate::Reg::read) this register and get [`macvtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACVTR)*/
pub struct MACVTRrs;
impl crate::RegisterSpec for MACVTRrs {
    type Ux = u32;
}
///`read()` method returns [`macvtr::R`](R) reader structure
impl crate::Readable for MACVTRrs {}
///`write(|w| ..)` method takes [`macvtr::W`](W) writer structure
impl crate::Writable for MACVTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACVTR to value 0
impl crate::Resettable for MACVTRrs {}
