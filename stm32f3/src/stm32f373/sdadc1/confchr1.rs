///Register `CONFCHR1` reader
pub type R = crate::R<CONFCHR1rs>;
///Register `CONFCHR1` writer
pub type W = crate::W<CONFCHR1rs>;
///Field `CONFCH0` reader - CONFCH0
pub type CONFCH0_R = crate::FieldReader;
///Field `CONFCH0` writer - CONFCH0
pub type CONFCH0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CONFCH1` reader - CONFCH1
pub type CONFCH1_R = crate::FieldReader;
///Field `CONFCH1` writer - CONFCH1
pub type CONFCH1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CONFCH2` reader - CONFCH2
pub type CONFCH2_R = crate::FieldReader;
///Field `CONFCH2` writer - CONFCH2
pub type CONFCH2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CONFCH3` reader - CONFCH3
pub type CONFCH3_R = crate::FieldReader;
///Field `CONFCH3` writer - CONFCH3
pub type CONFCH3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CONFCH4` reader - CONFCH4
pub type CONFCH4_R = crate::FieldReader;
///Field `CONFCH4` writer - CONFCH4
pub type CONFCH4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CONFCH5` reader - CONFCH5
pub type CONFCH5_R = crate::FieldReader;
///Field `CONFCH5` writer - CONFCH5
pub type CONFCH5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CONFCH6` reader - CONFCH6
pub type CONFCH6_R = crate::FieldReader;
///Field `CONFCH6` writer - CONFCH6
pub type CONFCH6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CONFCH7` reader - CONFCH7
pub type CONFCH7_R = crate::FieldReader;
///Field `CONFCH7` writer - CONFCH7
pub type CONFCH7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - CONFCH0
    #[inline(always)]
    pub fn confch0(&self) -> CONFCH0_R {
        CONFCH0_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:5 - CONFCH1
    #[inline(always)]
    pub fn confch1(&self) -> CONFCH1_R {
        CONFCH1_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:9 - CONFCH2
    #[inline(always)]
    pub fn confch2(&self) -> CONFCH2_R {
        CONFCH2_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 12:13 - CONFCH3
    #[inline(always)]
    pub fn confch3(&self) -> CONFCH3_R {
        CONFCH3_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 16:17 - CONFCH4
    #[inline(always)]
    pub fn confch4(&self) -> CONFCH4_R {
        CONFCH4_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 20:21 - CONFCH5
    #[inline(always)]
    pub fn confch5(&self) -> CONFCH5_R {
        CONFCH5_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 24:25 - CONFCH6
    #[inline(always)]
    pub fn confch6(&self) -> CONFCH6_R {
        CONFCH6_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 28:29 - CONFCH7
    #[inline(always)]
    pub fn confch7(&self) -> CONFCH7_R {
        CONFCH7_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFCHR1")
            .field("confch7", &self.confch7())
            .field("confch6", &self.confch6())
            .field("confch5", &self.confch5())
            .field("confch4", &self.confch4())
            .field("confch3", &self.confch3())
            .field("confch2", &self.confch2())
            .field("confch1", &self.confch1())
            .field("confch0", &self.confch0())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - CONFCH0
    #[inline(always)]
    pub fn confch0(&mut self) -> CONFCH0_W<'_, CONFCHR1rs> {
        CONFCH0_W::new(self, 0)
    }
    ///Bits 4:5 - CONFCH1
    #[inline(always)]
    pub fn confch1(&mut self) -> CONFCH1_W<'_, CONFCHR1rs> {
        CONFCH1_W::new(self, 4)
    }
    ///Bits 8:9 - CONFCH2
    #[inline(always)]
    pub fn confch2(&mut self) -> CONFCH2_W<'_, CONFCHR1rs> {
        CONFCH2_W::new(self, 8)
    }
    ///Bits 12:13 - CONFCH3
    #[inline(always)]
    pub fn confch3(&mut self) -> CONFCH3_W<'_, CONFCHR1rs> {
        CONFCH3_W::new(self, 12)
    }
    ///Bits 16:17 - CONFCH4
    #[inline(always)]
    pub fn confch4(&mut self) -> CONFCH4_W<'_, CONFCHR1rs> {
        CONFCH4_W::new(self, 16)
    }
    ///Bits 20:21 - CONFCH5
    #[inline(always)]
    pub fn confch5(&mut self) -> CONFCH5_W<'_, CONFCHR1rs> {
        CONFCH5_W::new(self, 20)
    }
    ///Bits 24:25 - CONFCH6
    #[inline(always)]
    pub fn confch6(&mut self) -> CONFCH6_W<'_, CONFCHR1rs> {
        CONFCH6_W::new(self, 24)
    }
    ///Bits 28:29 - CONFCH7
    #[inline(always)]
    pub fn confch7(&mut self) -> CONFCH7_W<'_, CONFCHR1rs> {
        CONFCH7_W::new(self, 28)
    }
}
/**channel configuration register 1

You can [`read`](crate::Reg::read) this register and get [`confchr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confchr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:CONFCHR1)*/
pub struct CONFCHR1rs;
impl crate::RegisterSpec for CONFCHR1rs {
    type Ux = u32;
}
///`read()` method returns [`confchr1::R`](R) reader structure
impl crate::Readable for CONFCHR1rs {}
///`write(|w| ..)` method takes [`confchr1::W`](W) writer structure
impl crate::Writable for CONFCHR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CONFCHR1 to value 0
impl crate::Resettable for CONFCHR1rs {}
