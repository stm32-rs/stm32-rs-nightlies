///Register `FS_HCCHAR4` reader
pub type R = crate::R<FS_HCCHAR4rs>;
///Register `FS_HCCHAR4` writer
pub type W = crate::W<FS_HCCHAR4rs>;
///Field `MPSIZ` reader - Maximum packet size
pub type MPSIZ_R = crate::FieldReader<u16>;
///Field `MPSIZ` writer - Maximum packet size
pub type MPSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `EPNUM` reader - Endpoint number
pub type EPNUM_R = crate::FieldReader;
///Field `EPNUM` writer - Endpoint number
pub type EPNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `EPDIR` reader - Endpoint direction
pub type EPDIR_R = crate::BitReader;
///Field `EPDIR` writer - Endpoint direction
pub type EPDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSDEV` reader - Low-speed device
pub type LSDEV_R = crate::BitReader;
///Field `LSDEV` writer - Low-speed device
pub type LSDEV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPTYP` reader - Endpoint type
pub type EPTYP_R = crate::FieldReader;
///Field `EPTYP` writer - Endpoint type
pub type EPTYP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MCNT` reader - Multicount
pub type MCNT_R = crate::FieldReader;
///Field `MCNT` writer - Multicount
pub type MCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DAD` reader - Device address
pub type DAD_R = crate::FieldReader;
///Field `DAD` writer - Device address
pub type DAD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `ODDFRM` reader - Odd frame
pub type ODDFRM_R = crate::BitReader;
///Field `ODDFRM` writer - Odd frame
pub type ODDFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHDIS` reader - Channel disable
pub type CHDIS_R = crate::BitReader;
///Field `CHDIS` writer - Channel disable
pub type CHDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHENA` reader - Channel enable
pub type CHENA_R = crate::BitReader;
///Field `CHENA` writer - Channel enable
pub type CHENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:10 - Maximum packet size
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 11:14 - Endpoint number
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    ///Bit 15 - Endpoint direction
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - Low-speed device
    #[inline(always)]
    pub fn lsdev(&self) -> LSDEV_R {
        LSDEV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - Endpoint type
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - Multicount
    #[inline(always)]
    pub fn mcnt(&self) -> MCNT_R {
        MCNT_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:28 - Device address
    #[inline(always)]
    pub fn dad(&self) -> DAD_R {
        DAD_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    ///Bit 29 - Odd frame
    #[inline(always)]
    pub fn oddfrm(&self) -> ODDFRM_R {
        ODDFRM_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Channel disable
    #[inline(always)]
    pub fn chdis(&self) -> CHDIS_R {
        CHDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Channel enable
    #[inline(always)]
    pub fn chena(&self) -> CHENA_R {
        CHENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_HCCHAR4")
            .field("mpsiz", &self.mpsiz())
            .field("epnum", &self.epnum())
            .field("epdir", &self.epdir())
            .field("lsdev", &self.lsdev())
            .field("eptyp", &self.eptyp())
            .field("mcnt", &self.mcnt())
            .field("dad", &self.dad())
            .field("oddfrm", &self.oddfrm())
            .field("chdis", &self.chdis())
            .field("chena", &self.chena())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - Maximum packet size
    #[inline(always)]
    pub fn mpsiz(&mut self) -> MPSIZ_W<'_, FS_HCCHAR4rs> {
        MPSIZ_W::new(self, 0)
    }
    ///Bits 11:14 - Endpoint number
    #[inline(always)]
    pub fn epnum(&mut self) -> EPNUM_W<'_, FS_HCCHAR4rs> {
        EPNUM_W::new(self, 11)
    }
    ///Bit 15 - Endpoint direction
    #[inline(always)]
    pub fn epdir(&mut self) -> EPDIR_W<'_, FS_HCCHAR4rs> {
        EPDIR_W::new(self, 15)
    }
    ///Bit 17 - Low-speed device
    #[inline(always)]
    pub fn lsdev(&mut self) -> LSDEV_W<'_, FS_HCCHAR4rs> {
        LSDEV_W::new(self, 17)
    }
    ///Bits 18:19 - Endpoint type
    #[inline(always)]
    pub fn eptyp(&mut self) -> EPTYP_W<'_, FS_HCCHAR4rs> {
        EPTYP_W::new(self, 18)
    }
    ///Bits 20:21 - Multicount
    #[inline(always)]
    pub fn mcnt(&mut self) -> MCNT_W<'_, FS_HCCHAR4rs> {
        MCNT_W::new(self, 20)
    }
    ///Bits 22:28 - Device address
    #[inline(always)]
    pub fn dad(&mut self) -> DAD_W<'_, FS_HCCHAR4rs> {
        DAD_W::new(self, 22)
    }
    ///Bit 29 - Odd frame
    #[inline(always)]
    pub fn oddfrm(&mut self) -> ODDFRM_W<'_, FS_HCCHAR4rs> {
        ODDFRM_W::new(self, 29)
    }
    ///Bit 30 - Channel disable
    #[inline(always)]
    pub fn chdis(&mut self) -> CHDIS_W<'_, FS_HCCHAR4rs> {
        CHDIS_W::new(self, 30)
    }
    ///Bit 31 - Channel enable
    #[inline(always)]
    pub fn chena(&mut self) -> CHENA_W<'_, FS_HCCHAR4rs> {
        CHENA_W::new(self, 31)
    }
}
/**OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)

You can [`read`](crate::Reg::read) this register and get [`fs_hcchar4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_hcchar4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_FS_HOST:FS_HCCHAR4)*/
pub struct FS_HCCHAR4rs;
impl crate::RegisterSpec for FS_HCCHAR4rs {
    type Ux = u32;
}
///`read()` method returns [`fs_hcchar4::R`](R) reader structure
impl crate::Readable for FS_HCCHAR4rs {}
///`write(|w| ..)` method takes [`fs_hcchar4::W`](W) writer structure
impl crate::Writable for FS_HCCHAR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FS_HCCHAR4 to value 0
impl crate::Resettable for FS_HCCHAR4rs {}
