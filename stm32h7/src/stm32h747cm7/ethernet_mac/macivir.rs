#[doc = "Register `MACIVIR` reader"]
pub type R = crate::R<MACIVIRrs>;
#[doc = "Register `MACIVIR` writer"]
pub type W = crate::W<MACIVIRrs>;
#[doc = "Field `VLT` reader - VLAN Tag for Transmit Packets"]
pub type VLT_R = crate::FieldReader<u16>;
#[doc = "Field `VLT` writer - VLAN Tag for Transmit Packets"]
pub type VLT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VLC` reader - VLAN Tag Control in Transmit Packets"]
pub type VLC_R = crate::FieldReader;
#[doc = "Field `VLC` writer - VLAN Tag Control in Transmit Packets"]
pub type VLC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VLP` reader - VLAN Priority Control"]
pub type VLP_R = crate::BitReader;
#[doc = "Field `VLP` writer - VLAN Priority Control"]
pub type VLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSVL` reader - C-VLAN or S-VLAN"]
pub type CSVL_R = crate::BitReader;
#[doc = "Field `CSVL` writer - C-VLAN or S-VLAN"]
pub type CSVL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VLTI` reader - VLAN Tag Input"]
pub type VLTI_R = crate::BitReader;
#[doc = "Field `VLTI` writer - VLAN Tag Input"]
pub type VLTI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - VLAN Tag for Transmit Packets"]
    #[inline(always)]
    pub fn vlt(&self) -> VLT_R {
        VLT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - VLAN Tag Control in Transmit Packets"]
    #[inline(always)]
    pub fn vlc(&self) -> VLC_R {
        VLC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - VLAN Priority Control"]
    #[inline(always)]
    pub fn vlp(&self) -> VLP_R {
        VLP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - C-VLAN or S-VLAN"]
    #[inline(always)]
    pub fn csvl(&self) -> CSVL_R {
        CSVL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - VLAN Tag Input"]
    #[inline(always)]
    pub fn vlti(&self) -> VLTI_R {
        VLTI_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Tag for Transmit Packets"]
    #[inline(always)]
    #[must_use]
    pub fn vlt(&mut self) -> VLT_W<MACIVIRrs> {
        VLT_W::new(self, 0)
    }
    #[doc = "Bits 16:17 - VLAN Tag Control in Transmit Packets"]
    #[inline(always)]
    #[must_use]
    pub fn vlc(&mut self) -> VLC_W<MACIVIRrs> {
        VLC_W::new(self, 16)
    }
    #[doc = "Bit 18 - VLAN Priority Control"]
    #[inline(always)]
    #[must_use]
    pub fn vlp(&mut self) -> VLP_W<MACIVIRrs> {
        VLP_W::new(self, 18)
    }
    #[doc = "Bit 19 - C-VLAN or S-VLAN"]
    #[inline(always)]
    #[must_use]
    pub fn csvl(&mut self) -> CSVL_W<MACIVIRrs> {
        CSVL_W::new(self, 19)
    }
    #[doc = "Bit 20 - VLAN Tag Input"]
    #[inline(always)]
    #[must_use]
    pub fn vlti(&mut self) -> VLTI_W<MACIVIRrs> {
        VLTI_W::new(self, 20)
    }
}
#[doc = "Inner VLAN inclusion register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macivir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macivir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACIVIRrs;
impl crate::RegisterSpec for MACIVIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macivir::R`](R) reader structure"]
impl crate::Readable for MACIVIRrs {}
#[doc = "`write(|w| ..)` method takes [`macivir::W`](W) writer structure"]
impl crate::Writable for MACIVIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACIVIR to value 0"]
impl crate::Resettable for MACIVIRrs {
    const RESET_VALUE: u32 = 0;
}
