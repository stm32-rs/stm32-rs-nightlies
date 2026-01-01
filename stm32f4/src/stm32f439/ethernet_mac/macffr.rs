///Register `MACFFR` reader
pub type R = crate::R<MACFFRrs>;
///Register `MACFFR` writer
pub type W = crate::W<MACFFRrs>;
///Field `PM` reader - PM
pub type PM_R = crate::BitReader;
///Field `PM` writer - PM
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HU` reader - HU
pub type HU_R = crate::BitReader;
///Field `HU` writer - HU
pub type HU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HM` reader - HM
pub type HM_R = crate::BitReader;
///Field `HM` writer - HM
pub type HM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAIF` reader - DAIF
pub type DAIF_R = crate::BitReader;
///Field `DAIF` writer - DAIF
pub type DAIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAM` reader - RAM
pub type RAM_R = crate::BitReader;
///Field `RAM` writer - RAM
pub type RAM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BFD` reader - BFD
pub type BFD_R = crate::BitReader;
///Field `BFD` writer - BFD
pub type BFD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PCF` reader - PCF
pub type PCF_R = crate::BitReader;
///Field `PCF` writer - PCF
pub type PCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAIF` reader - SAIF
pub type SAIF_R = crate::BitReader;
///Field `SAIF` writer - SAIF
pub type SAIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAF` reader - SAF
pub type SAF_R = crate::BitReader;
///Field `SAF` writer - SAF
pub type SAF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPF` reader - HPF
pub type HPF_R = crate::BitReader;
///Field `HPF` writer - HPF
pub type HPF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RA` reader - RA
pub type RA_R = crate::BitReader;
///Field `RA` writer - RA
pub type RA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - PM
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HU
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HM
    #[inline(always)]
    pub fn hm(&self) -> HM_R {
        HM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DAIF
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - RAM
    #[inline(always)]
    pub fn ram(&self) -> RAM_R {
        RAM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - BFD
    #[inline(always)]
    pub fn bfd(&self) -> BFD_R {
        BFD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PCF
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SAIF
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SAF
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HPF
    #[inline(always)]
    pub fn hpf(&self) -> HPF_R {
        HPF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 31 - RA
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACFFR")
            .field("pm", &self.pm())
            .field("hu", &self.hu())
            .field("hm", &self.hm())
            .field("daif", &self.daif())
            .field("ram", &self.ram())
            .field("bfd", &self.bfd())
            .field("pcf", &self.pcf())
            .field("saif", &self.saif())
            .field("saf", &self.saf())
            .field("hpf", &self.hpf())
            .field("ra", &self.ra())
            .finish()
    }
}
impl W {
    ///Bit 0 - PM
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<'_, MACFFRrs> {
        PM_W::new(self, 0)
    }
    ///Bit 1 - HU
    #[inline(always)]
    pub fn hu(&mut self) -> HU_W<'_, MACFFRrs> {
        HU_W::new(self, 1)
    }
    ///Bit 2 - HM
    #[inline(always)]
    pub fn hm(&mut self) -> HM_W<'_, MACFFRrs> {
        HM_W::new(self, 2)
    }
    ///Bit 3 - DAIF
    #[inline(always)]
    pub fn daif(&mut self) -> DAIF_W<'_, MACFFRrs> {
        DAIF_W::new(self, 3)
    }
    ///Bit 4 - RAM
    #[inline(always)]
    pub fn ram(&mut self) -> RAM_W<'_, MACFFRrs> {
        RAM_W::new(self, 4)
    }
    ///Bit 5 - BFD
    #[inline(always)]
    pub fn bfd(&mut self) -> BFD_W<'_, MACFFRrs> {
        BFD_W::new(self, 5)
    }
    ///Bit 6 - PCF
    #[inline(always)]
    pub fn pcf(&mut self) -> PCF_W<'_, MACFFRrs> {
        PCF_W::new(self, 6)
    }
    ///Bit 7 - SAIF
    #[inline(always)]
    pub fn saif(&mut self) -> SAIF_W<'_, MACFFRrs> {
        SAIF_W::new(self, 7)
    }
    ///Bit 8 - SAF
    #[inline(always)]
    pub fn saf(&mut self) -> SAF_W<'_, MACFFRrs> {
        SAF_W::new(self, 8)
    }
    ///Bit 9 - HPF
    #[inline(always)]
    pub fn hpf(&mut self) -> HPF_W<'_, MACFFRrs> {
        HPF_W::new(self, 9)
    }
    ///Bit 31 - RA
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W<'_, MACFFRrs> {
        RA_W::new(self, 31)
    }
}
/**Ethernet MAC frame filter register

You can [`read`](crate::Reg::read) this register and get [`macffr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macffr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#Ethernet_MAC:MACFFR)*/
pub struct MACFFRrs;
impl crate::RegisterSpec for MACFFRrs {
    type Ux = u32;
}
///`read()` method returns [`macffr::R`](R) reader structure
impl crate::Readable for MACFFRrs {}
///`write(|w| ..)` method takes [`macffr::W`](W) writer structure
impl crate::Writable for MACFFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACFFR to value 0
impl crate::Resettable for MACFFRrs {}
