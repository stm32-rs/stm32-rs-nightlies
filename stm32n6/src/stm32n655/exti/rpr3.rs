///Register `RPR3` reader
pub type R = crate::R<RPR3rs>;
///Register `RPR3` writer
pub type W = crate::W<RPR3rs>;
///Field `RPIF66` reader - configurable event input 66rising edge pending bit
pub type RPIF66_R = crate::BitReader;
///Field `RPIF66` writer - configurable event input 66rising edge pending bit
pub type RPIF66_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF68` reader - configurable event input x rising edge pending bit
pub type RPIF68_R = crate::BitReader;
///Field `RPIF68` writer - configurable event input x rising edge pending bit
pub type RPIF68_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF69` reader - configurable event input x rising edge pending bit
pub type RPIF69_R = crate::BitReader;
///Field `RPIF69` writer - configurable event input x rising edge pending bit
pub type RPIF69_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF70` reader - configurable event input x rising edge pending bit
pub type RPIF70_R = crate::BitReader;
///Field `RPIF70` writer - configurable event input x rising edge pending bit
pub type RPIF70_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF71` reader - configurable event input x rising edge pending bit
pub type RPIF71_R = crate::BitReader;
///Field `RPIF71` writer - configurable event input x rising edge pending bit
pub type RPIF71_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF72` reader - configurable event input x rising edge pending bit
pub type RPIF72_R = crate::BitReader;
///Field `RPIF72` writer - configurable event input x rising edge pending bit
pub type RPIF72_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF73` reader - configurable event input x rising edge pending bit
pub type RPIF73_R = crate::BitReader;
///Field `RPIF73` writer - configurable event input x rising edge pending bit
pub type RPIF73_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF74` reader - configurable event input x rising edge pending bit
pub type RPIF74_R = crate::BitReader;
///Field `RPIF74` writer - configurable event input x rising edge pending bit
pub type RPIF74_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - configurable event input 66rising edge pending bit
    #[inline(always)]
    pub fn rpif66(&self) -> RPIF66_R {
        RPIF66_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif68(&self) -> RPIF68_R {
        RPIF68_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif69(&self) -> RPIF69_R {
        RPIF69_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif70(&self) -> RPIF70_R {
        RPIF70_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif71(&self) -> RPIF71_R {
        RPIF71_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif72(&self) -> RPIF72_R {
        RPIF72_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif73(&self) -> RPIF73_R {
        RPIF73_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif74(&self) -> RPIF74_R {
        RPIF74_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RPR3")
            .field("rpif66", &self.rpif66())
            .field("rpif68", &self.rpif68())
            .field("rpif69", &self.rpif69())
            .field("rpif70", &self.rpif70())
            .field("rpif71", &self.rpif71())
            .field("rpif72", &self.rpif72())
            .field("rpif73", &self.rpif73())
            .field("rpif74", &self.rpif74())
            .finish()
    }
}
impl W {
    ///Bit 2 - configurable event input 66rising edge pending bit
    #[inline(always)]
    pub fn rpif66(&mut self) -> RPIF66_W<'_, RPR3rs> {
        RPIF66_W::new(self, 2)
    }
    ///Bit 4 - configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif68(&mut self) -> RPIF68_W<'_, RPR3rs> {
        RPIF68_W::new(self, 4)
    }
    ///Bit 5 - configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif69(&mut self) -> RPIF69_W<'_, RPR3rs> {
        RPIF69_W::new(self, 5)
    }
    ///Bit 6 - configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif70(&mut self) -> RPIF70_W<'_, RPR3rs> {
        RPIF70_W::new(self, 6)
    }
    ///Bit 7 - configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif71(&mut self) -> RPIF71_W<'_, RPR3rs> {
        RPIF71_W::new(self, 7)
    }
    ///Bit 8 - configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif72(&mut self) -> RPIF72_W<'_, RPR3rs> {
        RPIF72_W::new(self, 8)
    }
    ///Bit 9 - configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif73(&mut self) -> RPIF73_W<'_, RPR3rs> {
        RPIF73_W::new(self, 9)
    }
    ///Bit 10 - configurable event input x rising edge pending bit
    #[inline(always)]
    pub fn rpif74(&mut self) -> RPIF74_W<'_, RPR3rs> {
        RPIF74_W::new(self, 10)
    }
}
/**EXTI rising edge pending register

You can [`read`](crate::Reg::read) this register and get [`rpr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#EXTI:RPR3)*/
pub struct RPR3rs;
impl crate::RegisterSpec for RPR3rs {
    type Ux = u32;
}
///`read()` method returns [`rpr3::R`](R) reader structure
impl crate::Readable for RPR3rs {}
///`write(|w| ..)` method takes [`rpr3::W`](W) writer structure
impl crate::Writable for RPR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RPR3 to value 0
impl crate::Resettable for RPR3rs {}
