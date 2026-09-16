///Register `FPR2` reader
pub type R = crate::R<FPR2rs>;
///Register `FPR2` writer
pub type W = crate::W<FPR2rs>;
///Field `FPIF39` reader - Configurable event input x falling edge pending bit
pub type FPIF39_R = crate::BitReader;
///Field `FPIF39` writer - Configurable event input x falling edge pending bit
pub type FPIF39_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF40` reader - Configurable event input x falling edge pending bit
pub type FPIF40_R = crate::BitReader;
///Field `FPIF40` writer - Configurable event input x falling edge pending bit
pub type FPIF40_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF51` reader - Configurable event input 51 falling edge pending bit
pub type FPIF51_R = crate::BitReader;
///Field `FPIF51` writer - Configurable event input 51 falling edge pending bit
pub type FPIF51_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF54` reader - Configurable event input 54 falling edge pending bit
pub type FPIF54_R = crate::BitReader;
///Field `FPIF54` writer - Configurable event input 54 falling edge pending bit
pub type FPIF54_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF56` reader - Configurable event input 56 falling edge pending bit
pub type FPIF56_R = crate::BitReader;
///Field `FPIF56` writer - Configurable event input 56 falling edge pending bit
pub type FPIF56_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 7 - Configurable event input x falling edge pending bit
    #[inline(always)]
    pub fn fpif39(&self) -> FPIF39_R {
        FPIF39_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Configurable event input x falling edge pending bit
    #[inline(always)]
    pub fn fpif40(&self) -> FPIF40_R {
        FPIF40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 19 - Configurable event input 51 falling edge pending bit
    #[inline(always)]
    pub fn fpif51(&self) -> FPIF51_R {
        FPIF51_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 22 - Configurable event input 54 falling edge pending bit
    #[inline(always)]
    pub fn fpif54(&self) -> FPIF54_R {
        FPIF54_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - Configurable event input 56 falling edge pending bit
    #[inline(always)]
    pub fn fpif56(&self) -> FPIF56_R {
        FPIF56_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPR2")
            .field("fpif39", &self.fpif39())
            .field("fpif40", &self.fpif40())
            .field("fpif51", &self.fpif51())
            .field("fpif54", &self.fpif54())
            .field("fpif56", &self.fpif56())
            .finish()
    }
}
impl W {
    ///Bit 7 - Configurable event input x falling edge pending bit
    #[inline(always)]
    pub fn fpif39(&mut self) -> FPIF39_W<'_, FPR2rs> {
        FPIF39_W::new(self, 7)
    }
    ///Bit 8 - Configurable event input x falling edge pending bit
    #[inline(always)]
    pub fn fpif40(&mut self) -> FPIF40_W<'_, FPR2rs> {
        FPIF40_W::new(self, 8)
    }
    ///Bit 19 - Configurable event input 51 falling edge pending bit
    #[inline(always)]
    pub fn fpif51(&mut self) -> FPIF51_W<'_, FPR2rs> {
        FPIF51_W::new(self, 19)
    }
    ///Bit 22 - Configurable event input 54 falling edge pending bit
    #[inline(always)]
    pub fn fpif54(&mut self) -> FPIF54_W<'_, FPR2rs> {
        FPIF54_W::new(self, 22)
    }
    ///Bit 24 - Configurable event input 56 falling edge pending bit
    #[inline(always)]
    pub fn fpif56(&mut self) -> FPIF56_W<'_, FPR2rs> {
        FPIF56_W::new(self, 24)
    }
}
/**EXTI falling edge pending register

You can [`read`](crate::Reg::read) this register and get [`fpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#EXTI:FPR2)*/
pub struct FPR2rs;
impl crate::RegisterSpec for FPR2rs {
    type Ux = u32;
}
///`read()` method returns [`fpr2::R`](R) reader structure
impl crate::Readable for FPR2rs {}
///`write(|w| ..)` method takes [`fpr2::W`](W) writer structure
impl crate::Writable for FPR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FPR2 to value 0
impl crate::Resettable for FPR2rs {}
