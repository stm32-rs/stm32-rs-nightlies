///Register `MACVIR` reader
pub type R = crate::R<MACVIRrs>;
///Register `MACVIR` writer
pub type W = crate::W<MACVIRrs>;
///Field `VLT` reader - VLT
pub type VLT_R = crate::FieldReader<u16>;
///Field `VLT` writer - VLT
pub type VLT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `VLC` reader - VLC
pub type VLC_R = crate::FieldReader;
///Field `VLC` writer - VLC
pub type VLC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `VLP` reader - VLP
pub type VLP_R = crate::BitReader;
///Field `VLP` writer - VLP
pub type VLP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSVL` reader - CSVL
pub type CSVL_R = crate::BitReader;
///Field `CSVL` writer - CSVL
pub type CSVL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VLTI` reader - VLTI
pub type VLTI_R = crate::BitReader;
///Field `VLTI` writer - VLTI
pub type VLTI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - VLT
    #[inline(always)]
    pub fn vlt(&self) -> VLT_R {
        VLT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:17 - VLC
    #[inline(always)]
    pub fn vlc(&self) -> VLC_R {
        VLC_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - VLP
    #[inline(always)]
    pub fn vlp(&self) -> VLP_R {
        VLP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - CSVL
    #[inline(always)]
    pub fn csvl(&self) -> CSVL_R {
        CSVL_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - VLTI
    #[inline(always)]
    pub fn vlti(&self) -> VLTI_R {
        VLTI_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACVIR")
            .field("vlt", &self.vlt())
            .field("vlc", &self.vlc())
            .field("vlp", &self.vlp())
            .field("csvl", &self.csvl())
            .field("vlti", &self.vlti())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - VLT
    #[inline(always)]
    pub fn vlt(&mut self) -> VLT_W<MACVIRrs> {
        VLT_W::new(self, 0)
    }
    ///Bits 16:17 - VLC
    #[inline(always)]
    pub fn vlc(&mut self) -> VLC_W<MACVIRrs> {
        VLC_W::new(self, 16)
    }
    ///Bit 18 - VLP
    #[inline(always)]
    pub fn vlp(&mut self) -> VLP_W<MACVIRrs> {
        VLP_W::new(self, 18)
    }
    ///Bit 19 - CSVL
    #[inline(always)]
    pub fn csvl(&mut self) -> CSVL_W<MACVIRrs> {
        CSVL_W::new(self, 19)
    }
    ///Bit 20 - VLTI
    #[inline(always)]
    pub fn vlti(&mut self) -> VLTI_W<MACVIRrs> {
        VLTI_W::new(self, 20)
    }
}
/**The VLAN Tag Inclusion or Replacement register contains the VLAN tag for insertion or replacement in the Transmit packets. It also contains the VLAN tag insertion controls.

You can [`read`](crate::Reg::read) this register and get [`macvir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:MACVIR)*/
pub struct MACVIRrs;
impl crate::RegisterSpec for MACVIRrs {
    type Ux = u32;
}
///`read()` method returns [`macvir::R`](R) reader structure
impl crate::Readable for MACVIRrs {}
///`write(|w| ..)` method takes [`macvir::W`](W) writer structure
impl crate::Writable for MACVIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MACVIR to value 0
impl crate::Resettable for MACVIRrs {
    const RESET_VALUE: u32 = 0;
}