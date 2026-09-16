///Register `DENABLE` reader
pub type R = crate::R<DENABLErs>;
///Register `DENABLE` writer
pub type W = crate::W<DENABLErs>;
///Field `DFTEN` reader - DFTEN
pub type DFTEN_R = crate::BitReader;
///Field `DFTEN` writer - DFTEN
pub type DFTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGEN` reader - DBGEN
pub type DBGEN_R = crate::BitReader;
///Field `DBGEN` writer - DBGEN
pub type DBGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NIDEN` reader - NIDEN
pub type NIDEN_R = crate::BitReader;
///Field `NIDEN` writer - NIDEN
pub type NIDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEVICEEN` reader - DEVICEEN
pub type DEVICEEN_R = crate::BitReader;
///Field `DEVICEEN` writer - DEVICEEN
pub type DEVICEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HDPEN` reader - HDPEN
pub type HDPEN_R = crate::BitReader;
///Field `HDPEN` writer - HDPEN
pub type HDPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPIDEN` reader - SPIDEN
pub type SPIDEN_R = crate::BitReader;
///Field `SPIDEN` writer - SPIDEN
pub type SPIDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPNIDEN` reader - SPNIDEN
pub type SPNIDEN_R = crate::BitReader;
///Field `SPNIDEN` writer - SPNIDEN
pub type SPNIDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CP15SDISABLE` reader - CP15SDISABLE
pub type CP15SDISABLE_R = crate::FieldReader;
///Field `CP15SDISABLE` writer - CP15SDISABLE
pub type CP15SDISABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CFGSDISABLE` reader - CFGSDISABLE
pub type CFGSDISABLE_R = crate::BitReader;
///Field `CFGSDISABLE` writer - CFGSDISABLE
pub type CFGSDISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGSWENABLE` reader - DBGSWENABLE
pub type DBGSWENABLE_R = crate::BitReader;
///Field `DBGSWENABLE` writer - DBGSWENABLE
pub type DBGSWENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DFTEN
    #[inline(always)]
    pub fn dften(&self) -> DFTEN_R {
        DFTEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DBGEN
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - NIDEN
    #[inline(always)]
    pub fn niden(&self) -> NIDEN_R {
        NIDEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DEVICEEN
    #[inline(always)]
    pub fn deviceen(&self) -> DEVICEEN_R {
        DEVICEEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HDPEN
    #[inline(always)]
    pub fn hdpen(&self) -> HDPEN_R {
        HDPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SPIDEN
    #[inline(always)]
    pub fn spiden(&self) -> SPIDEN_R {
        SPIDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SPNIDEN
    #[inline(always)]
    pub fn spniden(&self) -> SPNIDEN_R {
        SPNIDEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:8 - CP15SDISABLE
    #[inline(always)]
    pub fn cp15sdisable(&self) -> CP15SDISABLE_R {
        CP15SDISABLE_R::new(((self.bits >> 7) & 3) as u8)
    }
    ///Bit 9 - CFGSDISABLE
    #[inline(always)]
    pub fn cfgsdisable(&self) -> CFGSDISABLE_R {
        CFGSDISABLE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - DBGSWENABLE
    #[inline(always)]
    pub fn dbgswenable(&self) -> DBGSWENABLE_R {
        DBGSWENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DENABLE")
            .field("dften", &self.dften())
            .field("dbgen", &self.dbgen())
            .field("niden", &self.niden())
            .field("deviceen", &self.deviceen())
            .field("hdpen", &self.hdpen())
            .field("spiden", &self.spiden())
            .field("spniden", &self.spniden())
            .field("cp15sdisable", &self.cp15sdisable())
            .field("cfgsdisable", &self.cfgsdisable())
            .field("dbgswenable", &self.dbgswenable())
            .finish()
    }
}
impl W {
    ///Bit 0 - DFTEN
    #[inline(always)]
    pub fn dften(&mut self) -> DFTEN_W<'_, DENABLErs> {
        DFTEN_W::new(self, 0)
    }
    ///Bit 1 - DBGEN
    #[inline(always)]
    pub fn dbgen(&mut self) -> DBGEN_W<'_, DENABLErs> {
        DBGEN_W::new(self, 1)
    }
    ///Bit 2 - NIDEN
    #[inline(always)]
    pub fn niden(&mut self) -> NIDEN_W<'_, DENABLErs> {
        NIDEN_W::new(self, 2)
    }
    ///Bit 3 - DEVICEEN
    #[inline(always)]
    pub fn deviceen(&mut self) -> DEVICEEN_W<'_, DENABLErs> {
        DEVICEEN_W::new(self, 3)
    }
    ///Bit 4 - HDPEN
    #[inline(always)]
    pub fn hdpen(&mut self) -> HDPEN_W<'_, DENABLErs> {
        HDPEN_W::new(self, 4)
    }
    ///Bit 5 - SPIDEN
    #[inline(always)]
    pub fn spiden(&mut self) -> SPIDEN_W<'_, DENABLErs> {
        SPIDEN_W::new(self, 5)
    }
    ///Bit 6 - SPNIDEN
    #[inline(always)]
    pub fn spniden(&mut self) -> SPNIDEN_W<'_, DENABLErs> {
        SPNIDEN_W::new(self, 6)
    }
    ///Bits 7:8 - CP15SDISABLE
    #[inline(always)]
    pub fn cp15sdisable(&mut self) -> CP15SDISABLE_W<'_, DENABLErs> {
        CP15SDISABLE_W::new(self, 7)
    }
    ///Bit 9 - CFGSDISABLE
    #[inline(always)]
    pub fn cfgsdisable(&mut self) -> CFGSDISABLE_W<'_, DENABLErs> {
        CFGSDISABLE_W::new(self, 9)
    }
    ///Bit 10 - DBGSWENABLE
    #[inline(always)]
    pub fn dbgswenable(&mut self) -> DBGSWENABLE_W<'_, DENABLErs> {
        DBGSWENABLE_W::new(self, 10)
    }
}
/**reset value depends on OTP secure mode according toTable18: BSEC_DENABLE default values after reset on page181.

You can [`read`](crate::Reg::read) this register and get [`denable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`denable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:DENABLE)*/
pub struct DENABLErs;
impl crate::RegisterSpec for DENABLErs {
    type Ux = u32;
}
///`read()` method returns [`denable::R`](R) reader structure
impl crate::Readable for DENABLErs {}
///`write(|w| ..)` method takes [`denable::W`](W) writer structure
impl crate::Writable for DENABLErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DENABLE to value 0
impl crate::Resettable for DENABLErs {}
