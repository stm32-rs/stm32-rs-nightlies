///Register `PDCRE` reader
pub type R = crate::R<PDCRErs>;
///Register `PDCRE` writer
pub type W = crate::W<PDCRErs>;
///Field `PD0` reader - Port E pull-down bit y (y=0..15)
pub type PD0_R = crate::BitReader;
///Field `PD0` writer - Port E pull-down bit y (y=0..15)
pub type PD0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD1` reader - Port E pull-down bit y (y=0..15)
pub type PD1_R = crate::BitReader;
///Field `PD1` writer - Port E pull-down bit y (y=0..15)
pub type PD1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD2` reader - Port E pull-down bit y (y=0..15)
pub type PD2_R = crate::BitReader;
///Field `PD2` writer - Port E pull-down bit y (y=0..15)
pub type PD2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD3` reader - Port E pull-down bit y (y=0..15)
pub type PD3_R = crate::BitReader;
///Field `PD3` writer - Port E pull-down bit y (y=0..15)
pub type PD3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD4` reader - Port E pull-down bit y (y=0..15)
pub type PD4_R = crate::BitReader;
///Field `PD4` writer - Port E pull-down bit y (y=0..15)
pub type PD4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD5` reader - Port E pull-down bit y (y=0..15)
pub type PD5_R = crate::BitReader;
///Field `PD5` writer - Port E pull-down bit y (y=0..15)
pub type PD5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD6` reader - Port E pull-down bit y (y=0..15)
pub type PD6_R = crate::BitReader;
///Field `PD6` writer - Port E pull-down bit y (y=0..15)
pub type PD6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD7` reader - Port E pull-down bit y (y=0..15)
pub type PD7_R = crate::BitReader;
///Field `PD7` writer - Port E pull-down bit y (y=0..15)
pub type PD7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD8` reader - Port E pull-down bit y (y=0..15)
pub type PD8_R = crate::BitReader;
///Field `PD8` writer - Port E pull-down bit y (y=0..15)
pub type PD8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD9` reader - Port E pull-down bit y (y=0..15)
pub type PD9_R = crate::BitReader;
///Field `PD9` writer - Port E pull-down bit y (y=0..15)
pub type PD9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD10` reader - Port E pull-down bit y (y=0..15)
pub type PD10_R = crate::BitReader;
///Field `PD10` writer - Port E pull-down bit y (y=0..15)
pub type PD10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD11` reader - Port E pull-down bit y (y=0..15)
pub type PD11_R = crate::BitReader;
///Field `PD11` writer - Port E pull-down bit y (y=0..15)
pub type PD11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD12` reader - Port E pull-down bit y (y=0..15)
pub type PD12_R = crate::BitReader;
///Field `PD12` writer - Port E pull-down bit y (y=0..15)
pub type PD12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD13` reader - Port E pull-down bit y (y=0..15)
pub type PD13_R = crate::BitReader;
///Field `PD13` writer - Port E pull-down bit y (y=0..15)
pub type PD13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD14` reader - Port E pull-down bit y (y=0..15)
pub type PD14_R = crate::BitReader;
///Field `PD14` writer - Port E pull-down bit y (y=0..15)
pub type PD14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD15` reader - Port E pull-down bit y (y=0..15)
pub type PD15_R = crate::BitReader;
///Field `PD15` writer - Port E pull-down bit y (y=0..15)
pub type PD15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd8(&self) -> PD8_R {
        PD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd9(&self) -> PD9_R {
        PD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd10(&self) -> PD10_R {
        PD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd11(&self) -> PD11_R {
        PD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd12(&self) -> PD12_R {
        PD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd13(&self) -> PD13_R {
        PD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd14(&self) -> PD14_R {
        PD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd15(&self) -> PD15_R {
        PD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDCRE")
            .field("pd15", &self.pd15())
            .field("pd14", &self.pd14())
            .field("pd13", &self.pd13())
            .field("pd12", &self.pd12())
            .field("pd11", &self.pd11())
            .field("pd10", &self.pd10())
            .field("pd9", &self.pd9())
            .field("pd8", &self.pd8())
            .field("pd7", &self.pd7())
            .field("pd6", &self.pd6())
            .field("pd5", &self.pd5())
            .field("pd4", &self.pd4())
            .field("pd3", &self.pd3())
            .field("pd2", &self.pd2())
            .field("pd1", &self.pd1())
            .field("pd0", &self.pd0())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd0(&mut self) -> PD0_W<PDCRErs> {
        PD0_W::new(self, 0)
    }
    ///Bit 1 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd1(&mut self) -> PD1_W<PDCRErs> {
        PD1_W::new(self, 1)
    }
    ///Bit 2 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd2(&mut self) -> PD2_W<PDCRErs> {
        PD2_W::new(self, 2)
    }
    ///Bit 3 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd3(&mut self) -> PD3_W<PDCRErs> {
        PD3_W::new(self, 3)
    }
    ///Bit 4 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd4(&mut self) -> PD4_W<PDCRErs> {
        PD4_W::new(self, 4)
    }
    ///Bit 5 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd5(&mut self) -> PD5_W<PDCRErs> {
        PD5_W::new(self, 5)
    }
    ///Bit 6 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd6(&mut self) -> PD6_W<PDCRErs> {
        PD6_W::new(self, 6)
    }
    ///Bit 7 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd7(&mut self) -> PD7_W<PDCRErs> {
        PD7_W::new(self, 7)
    }
    ///Bit 8 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd8(&mut self) -> PD8_W<PDCRErs> {
        PD8_W::new(self, 8)
    }
    ///Bit 9 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd9(&mut self) -> PD9_W<PDCRErs> {
        PD9_W::new(self, 9)
    }
    ///Bit 10 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd10(&mut self) -> PD10_W<PDCRErs> {
        PD10_W::new(self, 10)
    }
    ///Bit 11 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd11(&mut self) -> PD11_W<PDCRErs> {
        PD11_W::new(self, 11)
    }
    ///Bit 12 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd12(&mut self) -> PD12_W<PDCRErs> {
        PD12_W::new(self, 12)
    }
    ///Bit 13 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd13(&mut self) -> PD13_W<PDCRErs> {
        PD13_W::new(self, 13)
    }
    ///Bit 14 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd14(&mut self) -> PD14_W<PDCRErs> {
        PD14_W::new(self, 14)
    }
    ///Bit 15 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd15(&mut self) -> PD15_W<PDCRErs> {
        PD15_W::new(self, 15)
    }
}
/**Power Port E pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#PWR:PDCRE)*/
pub struct PDCRErs;
impl crate::RegisterSpec for PDCRErs {
    type Ux = u32;
}
///`read()` method returns [`pdcre::R`](R) reader structure
impl crate::Readable for PDCRErs {}
///`write(|w| ..)` method takes [`pdcre::W`](W) writer structure
impl crate::Writable for PDCRErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PDCRE to value 0
impl crate::Resettable for PDCRErs {
    const RESET_VALUE: u32 = 0;
}