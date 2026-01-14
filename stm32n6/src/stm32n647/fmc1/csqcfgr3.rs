///Register `CSQCFGR3` reader
pub type R = crate::R<CSQCFGR3rs>;
///Register `CSQCFGR3` writer
pub type W = crate::W<CSQCFGR3rs>;
///Field `SNBR` reader - Number of sectors to be read/written
pub type SNBR_R = crate::FieldReader;
///Field `SNBR` writer - Number of sectors to be read/written
pub type SNBR_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `AC1T` reader - Address cycle 1 sequencer timings
pub type AC1T_R = crate::BitReader;
///Field `AC1T` writer - Address cycle 1 sequencer timings
pub type AC1T_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AC2T` reader - Address cycle 2 sequencer timings
pub type AC2T_R = crate::BitReader;
///Field `AC2T` writer - Address cycle 2 sequencer timings
pub type AC2T_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AC3T` reader - Address cycle 3 sequencer timings
pub type AC3T_R = crate::BitReader;
///Field `AC3T` writer - Address cycle 3 sequencer timings
pub type AC3T_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AC4T` reader - Address cycle 4sequencer timings
pub type AC4T_R = crate::BitReader;
///Field `AC4T` writer - Address cycle 4sequencer timings
pub type AC4T_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AC5T` reader - Address cycle 5 sequencer timings
pub type AC5T_R = crate::BitReader;
///Field `AC5T` writer - Address cycle 5 sequencer timings
pub type AC5T_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDT` reader - Spare data transfer sequencer timings
pub type SDT_R = crate::BitReader;
///Field `SDT` writer - Spare data transfer sequencer timings
pub type SDT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAC1T` reader - Random Address cycle 1 sequencer timings
pub type RAC1T_R = crate::BitReader;
///Field `RAC1T` writer - Random Address cycle 1 sequencer timings
pub type RAC1T_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAC2T` reader - Random Address cycle 2 sequencer timings
pub type RAC2T_R = crate::BitReader;
///Field `RAC2T` writer - Random Address cycle 2 sequencer timings
pub type RAC2T_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 8:13 - Number of sectors to be read/written
    #[inline(always)]
    pub fn snbr(&self) -> SNBR_R {
        SNBR_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bit 16 - Address cycle 1 sequencer timings
    #[inline(always)]
    pub fn ac1t(&self) -> AC1T_R {
        AC1T_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Address cycle 2 sequencer timings
    #[inline(always)]
    pub fn ac2t(&self) -> AC2T_R {
        AC2T_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Address cycle 3 sequencer timings
    #[inline(always)]
    pub fn ac3t(&self) -> AC3T_R {
        AC3T_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Address cycle 4sequencer timings
    #[inline(always)]
    pub fn ac4t(&self) -> AC4T_R {
        AC4T_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Address cycle 5 sequencer timings
    #[inline(always)]
    pub fn ac5t(&self) -> AC5T_R {
        AC5T_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Spare data transfer sequencer timings
    #[inline(always)]
    pub fn sdt(&self) -> SDT_R {
        SDT_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Random Address cycle 1 sequencer timings
    #[inline(always)]
    pub fn rac1t(&self) -> RAC1T_R {
        RAC1T_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Random Address cycle 2 sequencer timings
    #[inline(always)]
    pub fn rac2t(&self) -> RAC2T_R {
        RAC2T_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSQCFGR3")
            .field("snbr", &self.snbr())
            .field("ac1t", &self.ac1t())
            .field("ac2t", &self.ac2t())
            .field("ac3t", &self.ac3t())
            .field("ac4t", &self.ac4t())
            .field("ac5t", &self.ac5t())
            .field("sdt", &self.sdt())
            .field("rac1t", &self.rac1t())
            .field("rac2t", &self.rac2t())
            .finish()
    }
}
impl W {
    ///Bits 8:13 - Number of sectors to be read/written
    #[inline(always)]
    pub fn snbr(&mut self) -> SNBR_W<'_, CSQCFGR3rs> {
        SNBR_W::new(self, 8)
    }
    ///Bit 16 - Address cycle 1 sequencer timings
    #[inline(always)]
    pub fn ac1t(&mut self) -> AC1T_W<'_, CSQCFGR3rs> {
        AC1T_W::new(self, 16)
    }
    ///Bit 17 - Address cycle 2 sequencer timings
    #[inline(always)]
    pub fn ac2t(&mut self) -> AC2T_W<'_, CSQCFGR3rs> {
        AC2T_W::new(self, 17)
    }
    ///Bit 18 - Address cycle 3 sequencer timings
    #[inline(always)]
    pub fn ac3t(&mut self) -> AC3T_W<'_, CSQCFGR3rs> {
        AC3T_W::new(self, 18)
    }
    ///Bit 19 - Address cycle 4sequencer timings
    #[inline(always)]
    pub fn ac4t(&mut self) -> AC4T_W<'_, CSQCFGR3rs> {
        AC4T_W::new(self, 19)
    }
    ///Bit 20 - Address cycle 5 sequencer timings
    #[inline(always)]
    pub fn ac5t(&mut self) -> AC5T_W<'_, CSQCFGR3rs> {
        AC5T_W::new(self, 20)
    }
    ///Bit 21 - Spare data transfer sequencer timings
    #[inline(always)]
    pub fn sdt(&mut self) -> SDT_W<'_, CSQCFGR3rs> {
        SDT_W::new(self, 21)
    }
    ///Bit 22 - Random Address cycle 1 sequencer timings
    #[inline(always)]
    pub fn rac1t(&mut self) -> RAC1T_W<'_, CSQCFGR3rs> {
        RAC1T_W::new(self, 22)
    }
    ///Bit 23 - Random Address cycle 2 sequencer timings
    #[inline(always)]
    pub fn rac2t(&mut self) -> RAC2T_W<'_, CSQCFGR3rs> {
        RAC2T_W::new(self, 23)
    }
}
/**FMC NAND sequencer configuration register 3

You can [`read`](crate::Reg::read) this register and get [`csqcfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqcfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FMC1:CSQCFGR3)*/
pub struct CSQCFGR3rs;
impl crate::RegisterSpec for CSQCFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`csqcfgr3::R`](R) reader structure
impl crate::Readable for CSQCFGR3rs {}
///`write(|w| ..)` method takes [`csqcfgr3::W`](W) writer structure
impl crate::Writable for CSQCFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSQCFGR3 to value 0
impl crate::Resettable for CSQCFGR3rs {}
