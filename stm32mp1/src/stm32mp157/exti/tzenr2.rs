///Register `TZENR2` reader
pub type R = crate::R<TZENR2rs>;
///Register `TZENR2` writer
pub type W = crate::W<TZENR2rs>;
///Field `TZEN41` reader - TZEN41
pub type TZEN41_R = crate::BitReader;
///Field `TZEN41` writer - TZEN41
pub type TZEN41_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN54` reader - TZEN54
pub type TZEN54_R = crate::BitReader;
///Field `TZEN54` writer - TZEN54
pub type TZEN54_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN55` reader - TZEN55
pub type TZEN55_R = crate::BitReader;
///Field `TZEN55` writer - TZEN55
pub type TZEN55_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN56` reader - TZEN56
pub type TZEN56_R = crate::BitReader;
///Field `TZEN56` writer - TZEN56
pub type TZEN56_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN57` reader - TZEN57
pub type TZEN57_R = crate::BitReader;
///Field `TZEN57` writer - TZEN57
pub type TZEN57_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN58` reader - TZEN58
pub type TZEN58_R = crate::BitReader;
///Field `TZEN58` writer - TZEN58
pub type TZEN58_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN59` reader - TZEN59
pub type TZEN59_R = crate::BitReader;
///Field `TZEN59` writer - TZEN59
pub type TZEN59_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN60` reader - TZEN60
pub type TZEN60_R = crate::BitReader;
///Field `TZEN60` writer - TZEN60
pub type TZEN60_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 9 - TZEN41
    #[inline(always)]
    pub fn tzen41(&self) -> TZEN41_R {
        TZEN41_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 22 - TZEN54
    #[inline(always)]
    pub fn tzen54(&self) -> TZEN54_R {
        TZEN54_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - TZEN55
    #[inline(always)]
    pub fn tzen55(&self) -> TZEN55_R {
        TZEN55_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - TZEN56
    #[inline(always)]
    pub fn tzen56(&self) -> TZEN56_R {
        TZEN56_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - TZEN57
    #[inline(always)]
    pub fn tzen57(&self) -> TZEN57_R {
        TZEN57_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - TZEN58
    #[inline(always)]
    pub fn tzen58(&self) -> TZEN58_R {
        TZEN58_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - TZEN59
    #[inline(always)]
    pub fn tzen59(&self) -> TZEN59_R {
        TZEN59_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - TZEN60
    #[inline(always)]
    pub fn tzen60(&self) -> TZEN60_R {
        TZEN60_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZENR2")
            .field("tzen41", &self.tzen41())
            .field("tzen54", &self.tzen54())
            .field("tzen55", &self.tzen55())
            .field("tzen56", &self.tzen56())
            .field("tzen57", &self.tzen57())
            .field("tzen58", &self.tzen58())
            .field("tzen59", &self.tzen59())
            .field("tzen60", &self.tzen60())
            .finish()
    }
}
impl W {
    ///Bit 9 - TZEN41
    #[inline(always)]
    pub fn tzen41(&mut self) -> TZEN41_W<'_, TZENR2rs> {
        TZEN41_W::new(self, 9)
    }
    ///Bit 22 - TZEN54
    #[inline(always)]
    pub fn tzen54(&mut self) -> TZEN54_W<'_, TZENR2rs> {
        TZEN54_W::new(self, 22)
    }
    ///Bit 23 - TZEN55
    #[inline(always)]
    pub fn tzen55(&mut self) -> TZEN55_W<'_, TZENR2rs> {
        TZEN55_W::new(self, 23)
    }
    ///Bit 24 - TZEN56
    #[inline(always)]
    pub fn tzen56(&mut self) -> TZEN56_W<'_, TZENR2rs> {
        TZEN56_W::new(self, 24)
    }
    ///Bit 25 - TZEN57
    #[inline(always)]
    pub fn tzen57(&mut self) -> TZEN57_W<'_, TZENR2rs> {
        TZEN57_W::new(self, 25)
    }
    ///Bit 26 - TZEN58
    #[inline(always)]
    pub fn tzen58(&mut self) -> TZEN58_W<'_, TZENR2rs> {
        TZEN58_W::new(self, 26)
    }
    ///Bit 27 - TZEN59
    #[inline(always)]
    pub fn tzen59(&mut self) -> TZEN59_W<'_, TZENR2rs> {
        TZEN59_W::new(self, 27)
    }
    ///Bit 28 - TZEN60
    #[inline(always)]
    pub fn tzen60(&mut self) -> TZEN60_W<'_, TZENR2rs> {
        TZEN60_W::new(self, 28)
    }
}
/**This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events.

You can [`read`](crate::Reg::read) this register and get [`tzenr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzenr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#EXTI:TZENR2)*/
pub struct TZENR2rs;
impl crate::RegisterSpec for TZENR2rs {
    type Ux = u32;
}
///`read()` method returns [`tzenr2::R`](R) reader structure
impl crate::Readable for TZENR2rs {}
///`write(|w| ..)` method takes [`tzenr2::W`](W) writer structure
impl crate::Writable for TZENR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TZENR2 to value 0
impl crate::Resettable for TZENR2rs {}
