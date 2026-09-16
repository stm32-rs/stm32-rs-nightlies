///Register `OSPEEDR` reader
pub type R = crate::R<OSPEEDRrs>;
///Register `OSPEEDR` writer
pub type W = crate::W<OSPEEDRrs>;
///Field `OSPEEDR0` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR0_R = crate::FieldReader;
///Field `OSPEEDR0` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR1` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR1_R = crate::FieldReader;
///Field `OSPEEDR1` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR2` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR2_R = crate::FieldReader;
///Field `OSPEEDR2` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR3` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR3_R = crate::FieldReader;
///Field `OSPEEDR3` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR4` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR4_R = crate::FieldReader;
///Field `OSPEEDR4` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR5` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR5_R = crate::FieldReader;
///Field `OSPEEDR5` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR6` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR6_R = crate::FieldReader;
///Field `OSPEEDR6` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR7` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR7_R = crate::FieldReader;
///Field `OSPEEDR7` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR8` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR8_R = crate::FieldReader;
///Field `OSPEEDR8` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR8_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR9` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR9_R = crate::FieldReader;
///Field `OSPEEDR9` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR9_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR10` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR10_R = crate::FieldReader;
///Field `OSPEEDR10` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR10_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR11` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR11_R = crate::FieldReader;
///Field `OSPEEDR11` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR11_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR12` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR12_R = crate::FieldReader;
///Field `OSPEEDR12` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR12_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR13` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR13_R = crate::FieldReader;
///Field `OSPEEDR13` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR13_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR14` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR14_R = crate::FieldReader;
///Field `OSPEEDR14` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR14_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR15` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR15_R = crate::FieldReader;
///Field `OSPEEDR15` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR15_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr0(&self) -> OSPEEDR0_R {
        OSPEEDR0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr1(&self) -> OSPEEDR1_R {
        OSPEEDR1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr2(&self) -> OSPEEDR2_R {
        OSPEEDR2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr3(&self) -> OSPEEDR3_R {
        OSPEEDR3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr4(&self) -> OSPEEDR4_R {
        OSPEEDR4_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr5(&self) -> OSPEEDR5_R {
        OSPEEDR5_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr6(&self) -> OSPEEDR6_R {
        OSPEEDR6_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr7(&self) -> OSPEEDR7_R {
        OSPEEDR7_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr8(&self) -> OSPEEDR8_R {
        OSPEEDR8_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr9(&self) -> OSPEEDR9_R {
        OSPEEDR9_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr10(&self) -> OSPEEDR10_R {
        OSPEEDR10_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr11(&self) -> OSPEEDR11_R {
        OSPEEDR11_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr12(&self) -> OSPEEDR12_R {
        OSPEEDR12_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr13(&self) -> OSPEEDR13_R {
        OSPEEDR13_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr14(&self) -> OSPEEDR14_R {
        OSPEEDR14_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr15(&self) -> OSPEEDR15_R {
        OSPEEDR15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSPEEDR")
            .field("ospeedr15", &self.ospeedr15())
            .field("ospeedr14", &self.ospeedr14())
            .field("ospeedr13", &self.ospeedr13())
            .field("ospeedr12", &self.ospeedr12())
            .field("ospeedr11", &self.ospeedr11())
            .field("ospeedr10", &self.ospeedr10())
            .field("ospeedr9", &self.ospeedr9())
            .field("ospeedr8", &self.ospeedr8())
            .field("ospeedr7", &self.ospeedr7())
            .field("ospeedr6", &self.ospeedr6())
            .field("ospeedr5", &self.ospeedr5())
            .field("ospeedr4", &self.ospeedr4())
            .field("ospeedr3", &self.ospeedr3())
            .field("ospeedr2", &self.ospeedr2())
            .field("ospeedr1", &self.ospeedr1())
            .field("ospeedr0", &self.ospeedr0())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr0(&mut self) -> OSPEEDR0_W<'_, OSPEEDRrs> {
        OSPEEDR0_W::new(self, 0)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr1(&mut self) -> OSPEEDR1_W<'_, OSPEEDRrs> {
        OSPEEDR1_W::new(self, 2)
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr2(&mut self) -> OSPEEDR2_W<'_, OSPEEDRrs> {
        OSPEEDR2_W::new(self, 4)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr3(&mut self) -> OSPEEDR3_W<'_, OSPEEDRrs> {
        OSPEEDR3_W::new(self, 6)
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr4(&mut self) -> OSPEEDR4_W<'_, OSPEEDRrs> {
        OSPEEDR4_W::new(self, 8)
    }
    ///Bits 10:11 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr5(&mut self) -> OSPEEDR5_W<'_, OSPEEDRrs> {
        OSPEEDR5_W::new(self, 10)
    }
    ///Bits 12:13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr6(&mut self) -> OSPEEDR6_W<'_, OSPEEDRrs> {
        OSPEEDR6_W::new(self, 12)
    }
    ///Bits 14:15 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr7(&mut self) -> OSPEEDR7_W<'_, OSPEEDRrs> {
        OSPEEDR7_W::new(self, 14)
    }
    ///Bits 16:17 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr8(&mut self) -> OSPEEDR8_W<'_, OSPEEDRrs> {
        OSPEEDR8_W::new(self, 16)
    }
    ///Bits 18:19 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr9(&mut self) -> OSPEEDR9_W<'_, OSPEEDRrs> {
        OSPEEDR9_W::new(self, 18)
    }
    ///Bits 20:21 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr10(&mut self) -> OSPEEDR10_W<'_, OSPEEDRrs> {
        OSPEEDR10_W::new(self, 20)
    }
    ///Bits 22:23 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr11(&mut self) -> OSPEEDR11_W<'_, OSPEEDRrs> {
        OSPEEDR11_W::new(self, 22)
    }
    ///Bits 24:25 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr12(&mut self) -> OSPEEDR12_W<'_, OSPEEDRrs> {
        OSPEEDR12_W::new(self, 24)
    }
    ///Bits 26:27 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr13(&mut self) -> OSPEEDR13_W<'_, OSPEEDRrs> {
        OSPEEDR13_W::new(self, 26)
    }
    ///Bits 28:29 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr14(&mut self) -> OSPEEDR14_W<'_, OSPEEDRrs> {
        OSPEEDR14_W::new(self, 28)
    }
    ///Bits 30:31 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr15(&mut self) -> OSPEEDR15_W<'_, OSPEEDRrs> {
        OSPEEDR15_W::new(self, 30)
    }
}
/**GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`ospeedr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#GPIOK:OSPEEDR)*/
pub struct OSPEEDRrs;
impl crate::RegisterSpec for OSPEEDRrs {
    type Ux = u32;
}
///`read()` method returns [`ospeedr::R`](R) reader structure
impl crate::Readable for OSPEEDRrs {}
///`write(|w| ..)` method takes [`ospeedr::W`](W) writer structure
impl crate::Writable for OSPEEDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OSPEEDR to value 0
impl crate::Resettable for OSPEEDRrs {}
