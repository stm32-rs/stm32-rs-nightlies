///Register `MACVTDR` reader
pub type R = crate::R<MACVTDRrs>;
///Register `MACVTDR` writer
pub type W = crate::W<MACVTDRrs>;
///Field `VID` reader - VLAN Tag ID
pub type VID_R = crate::FieldReader<u16>;
///Field `VID` writer - VLAN Tag ID
pub type VID_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `VEN` reader - VLAN Tag Enable
pub type VEN_R = crate::BitReader;
///Field `VEN` writer - VLAN Tag Enable
pub type VEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETV` reader - 12-bit or 16-bit VLAN comparison
pub type ETV_R = crate::BitReader;
///Field `ETV` writer - 12-bit or 16-bit VLAN comparison
pub type ETV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DOVLTC` reader - Disable VLAN Type Comparison
pub type DOVLTC_R = crate::BitReader;
///Field `DOVLTC` writer - Disable VLAN Type Comparison
pub type DOVLTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERSVLM` reader - Enable S-VLAN Match for received Frames
pub type ERSVLM_R = crate::BitReader;
///Field `ERSVLM` writer - Enable S-VLAN Match for received Frames
pub type ERSVLM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERIVLT` reader - Enable Inner VLAN Tag Comparison
pub type ERIVLT_R = crate::BitReader;
///Field `ERIVLT` writer - Enable Inner VLAN Tag Comparison
pub type ERIVLT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMACHEN` reader - DMA Channel Number Enable
pub type DMACHEN_R = crate::BitReader;
///Field `DMACHEN` writer - DMA Channel Number Enable
pub type DMACHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMACHN` reader - DMA Channel Number
pub type DMACHN_R = crate::BitReader;
///Field `DMACHN` writer - DMA Channel Number
pub type DMACHN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - VLAN Tag ID
    #[inline(always)]
    pub fn vid(&self) -> VID_R {
        VID_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - VLAN Tag Enable
    #[inline(always)]
    pub fn ven(&self) -> VEN_R {
        VEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - 12-bit or 16-bit VLAN comparison
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Disable VLAN Type Comparison
    #[inline(always)]
    pub fn dovltc(&self) -> DOVLTC_R {
        DOVLTC_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Enable S-VLAN Match for received Frames
    #[inline(always)]
    pub fn ersvlm(&self) -> ERSVLM_R {
        ERSVLM_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Enable Inner VLAN Tag Comparison
    #[inline(always)]
    pub fn erivlt(&self) -> ERIVLT_R {
        ERIVLT_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - DMA Channel Number Enable
    #[inline(always)]
    pub fn dmachen(&self) -> DMACHEN_R {
        DMACHEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - DMA Channel Number
    #[inline(always)]
    pub fn dmachn(&self) -> DMACHN_R {
        DMACHN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACVTDR")
            .field("vid", &self.vid())
            .field("ven", &self.ven())
            .field("etv", &self.etv())
            .field("dovltc", &self.dovltc())
            .field("ersvlm", &self.ersvlm())
            .field("erivlt", &self.erivlt())
            .field("dmachen", &self.dmachen())
            .field("dmachn", &self.dmachn())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - VLAN Tag ID
    #[inline(always)]
    pub fn vid(&mut self) -> VID_W<'_, MACVTDRrs> {
        VID_W::new(self, 0)
    }
    ///Bit 16 - VLAN Tag Enable
    #[inline(always)]
    pub fn ven(&mut self) -> VEN_W<'_, MACVTDRrs> {
        VEN_W::new(self, 16)
    }
    ///Bit 17 - 12-bit or 16-bit VLAN comparison
    #[inline(always)]
    pub fn etv(&mut self) -> ETV_W<'_, MACVTDRrs> {
        ETV_W::new(self, 17)
    }
    ///Bit 18 - Disable VLAN Type Comparison
    #[inline(always)]
    pub fn dovltc(&mut self) -> DOVLTC_W<'_, MACVTDRrs> {
        DOVLTC_W::new(self, 18)
    }
    ///Bit 19 - Enable S-VLAN Match for received Frames
    #[inline(always)]
    pub fn ersvlm(&mut self) -> ERSVLM_W<'_, MACVTDRrs> {
        ERSVLM_W::new(self, 19)
    }
    ///Bit 20 - Enable Inner VLAN Tag Comparison
    #[inline(always)]
    pub fn erivlt(&mut self) -> ERIVLT_W<'_, MACVTDRrs> {
        ERIVLT_W::new(self, 20)
    }
    ///Bit 24 - DMA Channel Number Enable
    #[inline(always)]
    pub fn dmachen(&mut self) -> DMACHEN_W<'_, MACVTDRrs> {
        DMACHEN_W::new(self, 24)
    }
    ///Bit 25 - DMA Channel Number
    #[inline(always)]
    pub fn dmachn(&mut self) -> DMACHN_W<'_, MACVTDRrs> {
        DMACHN_W::new(self, 25)
    }
}
/**VLAN tag data register

You can [`read`](crate::Reg::read) this register and get [`macvtdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvtdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ETH:MACVTDR)*/
pub struct MACVTDRrs;
impl crate::RegisterSpec for MACVTDRrs {
    type Ux = u32;
}
///`read()` method returns [`macvtdr::R`](R) reader structure
impl crate::Readable for MACVTDRrs {}
///`write(|w| ..)` method takes [`macvtdr::W`](W) writer structure
impl crate::Writable for MACVTDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACVTDR to value 0
impl crate::Resettable for MACVTDRrs {}
