///Register `VC0CFGR1` reader
pub type R = crate::R<VC0CFGR1rs>;
///Register `VC0CFGR1` writer
pub type W = crate::W<VC0CFGR1rs>;
///Field `ALLDT` reader - All data types enable for the virtual channel x
pub type ALLDT_R = crate::BitReader;
///Field `ALLDT` writer - All data types enable for the virtual channel x
pub type ALLDT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DT0EN` reader - Data type 0 enable
pub type DT0EN_R = crate::BitReader;
///Field `DT0EN` writer - Data type 0 enable
pub type DT0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DT1EN` reader - Data type 1 enable
pub type DT1EN_R = crate::BitReader;
///Field `DT1EN` writer - Data type 1 enable
pub type DT1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DT2EN` reader - Data type 2 enable
pub type DT2EN_R = crate::BitReader;
///Field `DT2EN` writer - Data type 2 enable
pub type DT2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DT3EN` reader - Data type 3 enable
pub type DT3EN_R = crate::BitReader;
///Field `DT3EN` writer - Data type 3 enable
pub type DT3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DT4EN` reader - Data type 4 enable
pub type DT4EN_R = crate::BitReader;
///Field `DT4EN` writer - Data type 4 enable
pub type DT4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DT5EN` reader - Data type 5 enable
pub type DT5EN_R = crate::BitReader;
///Field `DT5EN` writer - Data type 5 enable
pub type DT5EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DT6EN` reader - Data type 6 enable
pub type DT6EN_R = crate::BitReader;
///Field `DT6EN` writer - Data type 6 enable
pub type DT6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDTFT` reader - Common format for all data types
pub type CDTFT_R = crate::FieldReader;
///Field `CDTFT` writer - Common format for all data types
pub type CDTFT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DT0` reader - Data type 0 class selection for virtual channel x
pub type DT0_R = crate::FieldReader;
///Field `DT0` writer - Data type 0 class selection for virtual channel x
pub type DT0_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `DT0FT` reader - Data type 0 format
pub type DT0FT_R = crate::FieldReader;
///Field `DT0FT` writer - Data type 0 format
pub type DT0FT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bit 0 - All data types enable for the virtual channel x
    #[inline(always)]
    pub fn alldt(&self) -> ALLDT_R {
        ALLDT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Data type 0 enable
    #[inline(always)]
    pub fn dt0en(&self) -> DT0EN_R {
        DT0EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Data type 1 enable
    #[inline(always)]
    pub fn dt1en(&self) -> DT1EN_R {
        DT1EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Data type 2 enable
    #[inline(always)]
    pub fn dt2en(&self) -> DT2EN_R {
        DT2EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Data type 3 enable
    #[inline(always)]
    pub fn dt3en(&self) -> DT3EN_R {
        DT3EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Data type 4 enable
    #[inline(always)]
    pub fn dt4en(&self) -> DT4EN_R {
        DT4EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Data type 5 enable
    #[inline(always)]
    pub fn dt5en(&self) -> DT5EN_R {
        DT5EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Data type 6 enable
    #[inline(always)]
    pub fn dt6en(&self) -> DT6EN_R {
        DT6EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:12 - Common format for all data types
    #[inline(always)]
    pub fn cdtft(&self) -> CDTFT_R {
        CDTFT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:21 - Data type 0 class selection for virtual channel x
    #[inline(always)]
    pub fn dt0(&self) -> DT0_R {
        DT0_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:28 - Data type 0 format
    #[inline(always)]
    pub fn dt0ft(&self) -> DT0FT_R {
        DT0FT_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VC0CFGR1")
            .field("alldt", &self.alldt())
            .field("dt0en", &self.dt0en())
            .field("dt1en", &self.dt1en())
            .field("dt2en", &self.dt2en())
            .field("dt3en", &self.dt3en())
            .field("dt4en", &self.dt4en())
            .field("dt5en", &self.dt5en())
            .field("dt6en", &self.dt6en())
            .field("cdtft", &self.cdtft())
            .field("dt0", &self.dt0())
            .field("dt0ft", &self.dt0ft())
            .finish()
    }
}
impl W {
    ///Bit 0 - All data types enable for the virtual channel x
    #[inline(always)]
    pub fn alldt(&mut self) -> ALLDT_W<'_, VC0CFGR1rs> {
        ALLDT_W::new(self, 0)
    }
    ///Bit 1 - Data type 0 enable
    #[inline(always)]
    pub fn dt0en(&mut self) -> DT0EN_W<'_, VC0CFGR1rs> {
        DT0EN_W::new(self, 1)
    }
    ///Bit 2 - Data type 1 enable
    #[inline(always)]
    pub fn dt1en(&mut self) -> DT1EN_W<'_, VC0CFGR1rs> {
        DT1EN_W::new(self, 2)
    }
    ///Bit 3 - Data type 2 enable
    #[inline(always)]
    pub fn dt2en(&mut self) -> DT2EN_W<'_, VC0CFGR1rs> {
        DT2EN_W::new(self, 3)
    }
    ///Bit 4 - Data type 3 enable
    #[inline(always)]
    pub fn dt3en(&mut self) -> DT3EN_W<'_, VC0CFGR1rs> {
        DT3EN_W::new(self, 4)
    }
    ///Bit 5 - Data type 4 enable
    #[inline(always)]
    pub fn dt4en(&mut self) -> DT4EN_W<'_, VC0CFGR1rs> {
        DT4EN_W::new(self, 5)
    }
    ///Bit 6 - Data type 5 enable
    #[inline(always)]
    pub fn dt5en(&mut self) -> DT5EN_W<'_, VC0CFGR1rs> {
        DT5EN_W::new(self, 6)
    }
    ///Bit 7 - Data type 6 enable
    #[inline(always)]
    pub fn dt6en(&mut self) -> DT6EN_W<'_, VC0CFGR1rs> {
        DT6EN_W::new(self, 7)
    }
    ///Bits 8:12 - Common format for all data types
    #[inline(always)]
    pub fn cdtft(&mut self) -> CDTFT_W<'_, VC0CFGR1rs> {
        CDTFT_W::new(self, 8)
    }
    ///Bits 16:21 - Data type 0 class selection for virtual channel x
    #[inline(always)]
    pub fn dt0(&mut self) -> DT0_W<'_, VC0CFGR1rs> {
        DT0_W::new(self, 16)
    }
    ///Bits 24:28 - Data type 0 format
    #[inline(always)]
    pub fn dt0ft(&mut self) -> DT0FT_W<'_, VC0CFGR1rs> {
        DT0FT_W::new(self, 24)
    }
}
/**CSI-2 Host virtual channel 0 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`vc0cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vc0cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#CSI:VC0CFGR1)*/
pub struct VC0CFGR1rs;
impl crate::RegisterSpec for VC0CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`vc0cfgr1::R`](R) reader structure
impl crate::Readable for VC0CFGR1rs {}
///`write(|w| ..)` method takes [`vc0cfgr1::W`](W) writer structure
impl crate::Writable for VC0CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VC0CFGR1 to value 0
impl crate::Resettable for VC0CFGR1rs {}
