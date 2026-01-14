///Register `RPR2` reader
pub type R = crate::R<RPR2rs>;
///Register `RPR2` writer
pub type W = crate::W<RPR2rs>;
///Field `RPIF39` reader - Configurable event input x rising edge pending bit
pub type RPIF39_R = crate::BitReader;
///Field `RPIF39` writer - Configurable event input x rising edge pending bit
pub type RPIF39_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF40` reader - Configurable event input x rising edge pending bit
pub type RPIF40_R = crate::BitReader;
///Field `RPIF40` writer - Configurable event input x rising edge pending bit
pub type RPIF40_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF51` reader - Configurable event input 51 rising edge pending bit
pub type RPIF51_R = crate::BitReader;
///Field `RPIF51` writer - Configurable event input 51 rising edge pending bit
pub type RPIF51_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF54` reader - Configurable event input 54 rising edge pending bit
pub type RPIF54_R = crate::BitReader;
///Field `RPIF54` writer - Configurable event input 54 rising edge pending bit
pub type RPIF54_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF56` reader - Configurable event input 56 rising edge pending bit
pub type RPIF56_R = crate::BitReader;
///Field `RPIF56` writer - Configurable event input 56 rising edge pending bit
pub type RPIF56_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 7 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif39(&self) -> RPIF39_R {
        RPIF39_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif40(&self) -> RPIF40_R {
        RPIF40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 19 - Configurable event input 51 rising edge pending bit
    #[inline(always)]
    pub fn rpif51(&self) -> RPIF51_R {
        RPIF51_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 22 - Configurable event input 54 rising edge pending bit
    #[inline(always)]
    pub fn rpif54(&self) -> RPIF54_R {
        RPIF54_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - Configurable event input 56 rising edge pending bit
    #[inline(always)]
    pub fn rpif56(&self) -> RPIF56_R {
        RPIF56_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RPR2")
            .field("rpif39", &self.rpif39())
            .field("rpif40", &self.rpif40())
            .field("rpif51", &self.rpif51())
            .field("rpif54", &self.rpif54())
            .field("rpif56", &self.rpif56())
            .finish()
    }
}
impl W {
    ///Bit 7 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif39(&mut self) -> RPIF39_W<'_, RPR2rs> {
        RPIF39_W::new(self, 7)
    }
    ///Bit 8 - Configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif40(&mut self) -> RPIF40_W<'_, RPR2rs> {
        RPIF40_W::new(self, 8)
    }
    ///Bit 19 - Configurable event input 51 rising edge pending bit
    #[inline(always)]
    pub fn rpif51(&mut self) -> RPIF51_W<'_, RPR2rs> {
        RPIF51_W::new(self, 19)
    }
    ///Bit 22 - Configurable event input 54 rising edge pending bit
    #[inline(always)]
    pub fn rpif54(&mut self) -> RPIF54_W<'_, RPR2rs> {
        RPIF54_W::new(self, 22)
    }
    ///Bit 24 - Configurable event input 56 rising edge pending bit
    #[inline(always)]
    pub fn rpif56(&mut self) -> RPIF56_W<'_, RPR2rs> {
        RPIF56_W::new(self, 24)
    }
}
/**EXTI rising edge pending register

You can [`read`](crate::Reg::read) this register and get [`rpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#EXTI:RPR2)*/
pub struct RPR2rs;
impl crate::RegisterSpec for RPR2rs {
    type Ux = u32;
}
///`read()` method returns [`rpr2::R`](R) reader structure
impl crate::Readable for RPR2rs {}
///`write(|w| ..)` method takes [`rpr2::W`](W) writer structure
impl crate::Writable for RPR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RPR2 to value 0
impl crate::Resettable for RPR2rs {}
