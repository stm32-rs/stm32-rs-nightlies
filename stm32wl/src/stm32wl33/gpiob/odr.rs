///Register `ODR` reader
pub type R = crate::R<ODRrs>;
///Register `ODR` writer
pub type W = crate::W<ODRrs>;
///Field `OD0` reader - OD0: Port B output data bit These bits can be read and written by software
pub type OD0_R = crate::BitReader;
///Field `OD0` writer - OD0: Port B output data bit These bits can be read and written by software
pub type OD0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OD1` reader - OD1: Port B output data bit These bits can be read and written by software
pub type OD1_R = crate::BitReader;
///Field `OD1` writer - OD1: Port B output data bit These bits can be read and written by software
pub type OD1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OD2` reader - OD2: Port B output data bit These bits can be read and written by software
pub type OD2_R = crate::BitReader;
///Field `OD2` writer - OD2: Port B output data bit These bits can be read and written by software
pub type OD2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OD3` reader - OD3: Port B output data bit These bits can be read and written by software
pub type OD3_R = crate::BitReader;
///Field `OD3` writer - OD3: Port B output data bit These bits can be read and written by software
pub type OD3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OD4` reader - OD4: Port B output data bit These bits can be read and written by software
pub type OD4_R = crate::BitReader;
///Field `OD4` writer - OD4: Port B output data bit These bits can be read and written by software
pub type OD4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OD5` reader - OD5: Port B output data bit These bits can be read and written by software
pub type OD5_R = crate::BitReader;
///Field `OD5` writer - OD5: Port B output data bit These bits can be read and written by software
pub type OD5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OD6` reader - OD6: Port B output data bit These bits can be read and written by software
pub type OD6_R = crate::BitReader;
///Field `OD6` writer - OD6: Port B output data bit These bits can be read and written by software
pub type OD6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OD7` reader - OD7: Port B output data bit These bits can be read and written by software
pub type OD7_R = crate::BitReader;
///Field `OD7` writer - OD7: Port B output data bit These bits can be read and written by software
pub type OD7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OD8` reader - OD8: Port B output data bit These bits can be read and written by software
pub type OD8_R = crate::BitReader;
///Field `OD8` writer - OD8: Port B output data bit These bits can be read and written by software
pub type OD8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OD9` reader - OD9: Port B output data bit These bits can be read and written by software
pub type OD9_R = crate::BitReader;
///Field `OD9` writer - OD9: Port B output data bit These bits can be read and written by software
pub type OD9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OD10` reader - OD10: Port B output data bit These bits can be read and written by software
pub type OD10_R = crate::BitReader;
///Field `OD10` writer - OD10: Port B output data bit These bits can be read and written by software
pub type OD10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OD11` reader - OD11: Port B output data bit These bits can be read and written by software
pub type OD11_R = crate::BitReader;
///Field `OD11` writer - OD11: Port B output data bit These bits can be read and written by software
pub type OD11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OD12` reader - OD12: Port B output data bit These bits can be read and written by software
pub type OD12_R = crate::BitReader;
///Field `OD12` writer - OD12: Port B output data bit These bits can be read and written by software
pub type OD12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OD13` reader - OD13: Port B output data bit These bits can be read and written by software
pub type OD13_R = crate::BitReader;
///Field `OD13` writer - OD13: Port B output data bit These bits can be read and written by software
pub type OD13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OD14` reader - OD14: Port B output data bit These bits can be read and written by software
pub type OD14_R = crate::BitReader;
///Field `OD14` writer - OD14: Port B output data bit These bits can be read and written by software
pub type OD14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OD15` reader - OD15: Port B output data bit These bits can be read and written by software
pub type OD15_R = crate::BitReader;
///Field `OD15` writer - OD15: Port B output data bit These bits can be read and written by software
pub type OD15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - OD0: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od0(&self) -> OD0_R {
        OD0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - OD1: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od1(&self) -> OD1_R {
        OD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - OD2: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od2(&self) -> OD2_R {
        OD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - OD3: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od3(&self) -> OD3_R {
        OD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - OD4: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od4(&self) -> OD4_R {
        OD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - OD5: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od5(&self) -> OD5_R {
        OD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - OD6: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od6(&self) -> OD6_R {
        OD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - OD7: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od7(&self) -> OD7_R {
        OD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - OD8: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od8(&self) -> OD8_R {
        OD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - OD9: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od9(&self) -> OD9_R {
        OD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - OD10: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od10(&self) -> OD10_R {
        OD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - OD11: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od11(&self) -> OD11_R {
        OD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - OD12: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od12(&self) -> OD12_R {
        OD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - OD13: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od13(&self) -> OD13_R {
        OD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - OD14: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od14(&self) -> OD14_R {
        OD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - OD15: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od15(&self) -> OD15_R {
        OD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ODR")
            .field("od0", &self.od0())
            .field("od1", &self.od1())
            .field("od2", &self.od2())
            .field("od3", &self.od3())
            .field("od4", &self.od4())
            .field("od5", &self.od5())
            .field("od6", &self.od6())
            .field("od7", &self.od7())
            .field("od8", &self.od8())
            .field("od9", &self.od9())
            .field("od10", &self.od10())
            .field("od11", &self.od11())
            .field("od12", &self.od12())
            .field("od13", &self.od13())
            .field("od14", &self.od14())
            .field("od15", &self.od15())
            .finish()
    }
}
impl W {
    ///Bit 0 - OD0: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od0(&mut self) -> OD0_W<'_, ODRrs> {
        OD0_W::new(self, 0)
    }
    ///Bit 1 - OD1: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od1(&mut self) -> OD1_W<'_, ODRrs> {
        OD1_W::new(self, 1)
    }
    ///Bit 2 - OD2: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od2(&mut self) -> OD2_W<'_, ODRrs> {
        OD2_W::new(self, 2)
    }
    ///Bit 3 - OD3: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od3(&mut self) -> OD3_W<'_, ODRrs> {
        OD3_W::new(self, 3)
    }
    ///Bit 4 - OD4: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od4(&mut self) -> OD4_W<'_, ODRrs> {
        OD4_W::new(self, 4)
    }
    ///Bit 5 - OD5: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od5(&mut self) -> OD5_W<'_, ODRrs> {
        OD5_W::new(self, 5)
    }
    ///Bit 6 - OD6: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od6(&mut self) -> OD6_W<'_, ODRrs> {
        OD6_W::new(self, 6)
    }
    ///Bit 7 - OD7: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od7(&mut self) -> OD7_W<'_, ODRrs> {
        OD7_W::new(self, 7)
    }
    ///Bit 8 - OD8: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od8(&mut self) -> OD8_W<'_, ODRrs> {
        OD8_W::new(self, 8)
    }
    ///Bit 9 - OD9: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od9(&mut self) -> OD9_W<'_, ODRrs> {
        OD9_W::new(self, 9)
    }
    ///Bit 10 - OD10: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od10(&mut self) -> OD10_W<'_, ODRrs> {
        OD10_W::new(self, 10)
    }
    ///Bit 11 - OD11: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od11(&mut self) -> OD11_W<'_, ODRrs> {
        OD11_W::new(self, 11)
    }
    ///Bit 12 - OD12: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od12(&mut self) -> OD12_W<'_, ODRrs> {
        OD12_W::new(self, 12)
    }
    ///Bit 13 - OD13: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od13(&mut self) -> OD13_W<'_, ODRrs> {
        OD13_W::new(self, 13)
    }
    ///Bit 14 - OD14: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od14(&mut self) -> OD14_W<'_, ODRrs> {
        OD14_W::new(self, 14)
    }
    ///Bit 15 - OD15: Port B output data bit These bits can be read and written by software
    #[inline(always)]
    pub fn od15(&mut self) -> OD15_W<'_, ODRrs> {
        OD15_W::new(self, 15)
    }
}
/**ODR register

You can [`read`](crate::Reg::read) this register and get [`odr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#GPIOB:ODR)*/
pub struct ODRrs;
impl crate::RegisterSpec for ODRrs {
    type Ux = u32;
}
///`read()` method returns [`odr::R`](R) reader structure
impl crate::Readable for ODRrs {}
///`write(|w| ..)` method takes [`odr::W`](W) writer structure
impl crate::Writable for ODRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ODR to value 0
impl crate::Resettable for ODRrs {}
