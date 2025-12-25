///Register `DX1DQTR` reader
pub type R = crate::R<DX1DQTRrs>;
///Register `DX1DQTR` writer
pub type W = crate::W<DX1DQTRrs>;
///Field `DQDLY0` reader - DQDLY0
pub type DQDLY0_R = crate::FieldReader;
///Field `DQDLY0` writer - DQDLY0
pub type DQDLY0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DQDLY1` reader - DQDLY1
pub type DQDLY1_R = crate::FieldReader;
///Field `DQDLY1` writer - DQDLY1
pub type DQDLY1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DQDLY2` reader - DQDLY2
pub type DQDLY2_R = crate::FieldReader;
///Field `DQDLY2` writer - DQDLY2
pub type DQDLY2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DQDLY3` reader - DQDLY3
pub type DQDLY3_R = crate::FieldReader;
///Field `DQDLY3` writer - DQDLY3
pub type DQDLY3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DQDLY4` reader - DQDLY4
pub type DQDLY4_R = crate::FieldReader;
///Field `DQDLY4` writer - DQDLY4
pub type DQDLY4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DQDLY5` reader - DQDLY5
pub type DQDLY5_R = crate::FieldReader;
///Field `DQDLY5` writer - DQDLY5
pub type DQDLY5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DQDLY6` reader - DQDLY6
pub type DQDLY6_R = crate::FieldReader;
///Field `DQDLY6` writer - DQDLY6
pub type DQDLY6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DQDLY7` reader - DQDLY7
pub type DQDLY7_R = crate::FieldReader;
///Field `DQDLY7` writer - DQDLY7
pub type DQDLY7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - DQDLY0
    #[inline(always)]
    pub fn dqdly0(&self) -> DQDLY0_R {
        DQDLY0_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - DQDLY1
    #[inline(always)]
    pub fn dqdly1(&self) -> DQDLY1_R {
        DQDLY1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - DQDLY2
    #[inline(always)]
    pub fn dqdly2(&self) -> DQDLY2_R {
        DQDLY2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - DQDLY3
    #[inline(always)]
    pub fn dqdly3(&self) -> DQDLY3_R {
        DQDLY3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - DQDLY4
    #[inline(always)]
    pub fn dqdly4(&self) -> DQDLY4_R {
        DQDLY4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - DQDLY5
    #[inline(always)]
    pub fn dqdly5(&self) -> DQDLY5_R {
        DQDLY5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - DQDLY6
    #[inline(always)]
    pub fn dqdly6(&self) -> DQDLY6_R {
        DQDLY6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - DQDLY7
    #[inline(always)]
    pub fn dqdly7(&self) -> DQDLY7_R {
        DQDLY7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DX1DQTR")
            .field("dqdly0", &self.dqdly0())
            .field("dqdly1", &self.dqdly1())
            .field("dqdly2", &self.dqdly2())
            .field("dqdly3", &self.dqdly3())
            .field("dqdly4", &self.dqdly4())
            .field("dqdly5", &self.dqdly5())
            .field("dqdly6", &self.dqdly6())
            .field("dqdly7", &self.dqdly7())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - DQDLY0
    #[inline(always)]
    pub fn dqdly0(&mut self) -> DQDLY0_W<'_, DX1DQTRrs> {
        DQDLY0_W::new(self, 0)
    }
    ///Bits 4:7 - DQDLY1
    #[inline(always)]
    pub fn dqdly1(&mut self) -> DQDLY1_W<'_, DX1DQTRrs> {
        DQDLY1_W::new(self, 4)
    }
    ///Bits 8:11 - DQDLY2
    #[inline(always)]
    pub fn dqdly2(&mut self) -> DQDLY2_W<'_, DX1DQTRrs> {
        DQDLY2_W::new(self, 8)
    }
    ///Bits 12:15 - DQDLY3
    #[inline(always)]
    pub fn dqdly3(&mut self) -> DQDLY3_W<'_, DX1DQTRrs> {
        DQDLY3_W::new(self, 12)
    }
    ///Bits 16:19 - DQDLY4
    #[inline(always)]
    pub fn dqdly4(&mut self) -> DQDLY4_W<'_, DX1DQTRrs> {
        DQDLY4_W::new(self, 16)
    }
    ///Bits 20:23 - DQDLY5
    #[inline(always)]
    pub fn dqdly5(&mut self) -> DQDLY5_W<'_, DX1DQTRrs> {
        DQDLY5_W::new(self, 20)
    }
    ///Bits 24:27 - DQDLY6
    #[inline(always)]
    pub fn dqdly6(&mut self) -> DQDLY6_W<'_, DX1DQTRrs> {
        DQDLY6_W::new(self, 24)
    }
    ///Bits 28:31 - DQDLY7
    #[inline(always)]
    pub fn dqdly7(&mut self) -> DQDLY7_W<'_, DX1DQTRrs> {
        DQDLY7_W::new(self, 28)
    }
}
/**DDRPHYC byte lane 1 DQT register

You can [`read`](crate::Reg::read) this register and get [`dx1dqtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dx1dqtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRPHYC:DX1DQTR)*/
pub struct DX1DQTRrs;
impl crate::RegisterSpec for DX1DQTRrs {
    type Ux = u32;
}
///`read()` method returns [`dx1dqtr::R`](R) reader structure
impl crate::Readable for DX1DQTRrs {}
///`write(|w| ..)` method takes [`dx1dqtr::W`](W) writer structure
impl crate::Writable for DX1DQTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DX1DQTR to value 0xffff_ffff
impl crate::Resettable for DX1DQTRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
