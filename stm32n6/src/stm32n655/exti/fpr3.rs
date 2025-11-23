///Register `FPR3` reader
pub type R = crate::R<FPR3rs>;
///Register `FPR3` writer
pub type W = crate::W<FPR3rs>;
///Field `FPIF66` reader - configurable event input 66 falling edge pending bit
pub type FPIF66_R = crate::BitReader;
///Field `FPIF66` writer - configurable event input 66 falling edge pending bit
pub type FPIF66_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF68` reader - configurable event input x falling edge pending bit
pub type FPIF68_R = crate::BitReader;
///Field `FPIF68` writer - configurable event input x falling edge pending bit
pub type FPIF68_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF69` reader - configurable event input x falling edge pending bit
pub type FPIF69_R = crate::BitReader;
///Field `FPIF69` writer - configurable event input x falling edge pending bit
pub type FPIF69_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF70` reader - configurable event input x falling edge pending bit
pub type FPIF70_R = crate::BitReader;
///Field `FPIF70` writer - configurable event input x falling edge pending bit
pub type FPIF70_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF71` reader - configurable event input x falling edge pending bit
pub type FPIF71_R = crate::BitReader;
///Field `FPIF71` writer - configurable event input x falling edge pending bit
pub type FPIF71_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF72` reader - configurable event input x falling edge pending bit
pub type FPIF72_R = crate::BitReader;
///Field `FPIF72` writer - configurable event input x falling edge pending bit
pub type FPIF72_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF73` reader - configurable event input x falling edge pending bit
pub type FPIF73_R = crate::BitReader;
///Field `FPIF73` writer - configurable event input x falling edge pending bit
pub type FPIF73_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF74` reader - configurable event input x falling edge pending bit
pub type FPIF74_R = crate::BitReader;
///Field `FPIF74` writer - configurable event input x falling edge pending bit
pub type FPIF74_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - configurable event input 66 falling edge pending bit
    #[inline(always)]
    pub fn fpif66(&self) -> FPIF66_R {
        FPIF66_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - configurable event input x falling edge pending bit
    #[inline(always)]
    pub fn fpif68(&self) -> FPIF68_R {
        FPIF68_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - configurable event input x falling edge pending bit
    #[inline(always)]
    pub fn fpif69(&self) -> FPIF69_R {
        FPIF69_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - configurable event input x falling edge pending bit
    #[inline(always)]
    pub fn fpif70(&self) -> FPIF70_R {
        FPIF70_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - configurable event input x falling edge pending bit
    #[inline(always)]
    pub fn fpif71(&self) -> FPIF71_R {
        FPIF71_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - configurable event input x falling edge pending bit
    #[inline(always)]
    pub fn fpif72(&self) -> FPIF72_R {
        FPIF72_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - configurable event input x falling edge pending bit
    #[inline(always)]
    pub fn fpif73(&self) -> FPIF73_R {
        FPIF73_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - configurable event input x falling edge pending bit
    #[inline(always)]
    pub fn fpif74(&self) -> FPIF74_R {
        FPIF74_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPR3")
            .field("fpif66", &self.fpif66())
            .field("fpif68", &self.fpif68())
            .field("fpif69", &self.fpif69())
            .field("fpif70", &self.fpif70())
            .field("fpif71", &self.fpif71())
            .field("fpif72", &self.fpif72())
            .field("fpif73", &self.fpif73())
            .field("fpif74", &self.fpif74())
            .finish()
    }
}
impl W {
    ///Bit 2 - configurable event input 66 falling edge pending bit
    #[inline(always)]
    pub fn fpif66(&mut self) -> FPIF66_W<'_, FPR3rs> {
        FPIF66_W::new(self, 2)
    }
    ///Bit 4 - configurable event input x falling edge pending bit
    #[inline(always)]
    pub fn fpif68(&mut self) -> FPIF68_W<'_, FPR3rs> {
        FPIF68_W::new(self, 4)
    }
    ///Bit 5 - configurable event input x falling edge pending bit
    #[inline(always)]
    pub fn fpif69(&mut self) -> FPIF69_W<'_, FPR3rs> {
        FPIF69_W::new(self, 5)
    }
    ///Bit 6 - configurable event input x falling edge pending bit
    #[inline(always)]
    pub fn fpif70(&mut self) -> FPIF70_W<'_, FPR3rs> {
        FPIF70_W::new(self, 6)
    }
    ///Bit 7 - configurable event input x falling edge pending bit
    #[inline(always)]
    pub fn fpif71(&mut self) -> FPIF71_W<'_, FPR3rs> {
        FPIF71_W::new(self, 7)
    }
    ///Bit 8 - configurable event input x falling edge pending bit
    #[inline(always)]
    pub fn fpif72(&mut self) -> FPIF72_W<'_, FPR3rs> {
        FPIF72_W::new(self, 8)
    }
    ///Bit 9 - configurable event input x falling edge pending bit
    #[inline(always)]
    pub fn fpif73(&mut self) -> FPIF73_W<'_, FPR3rs> {
        FPIF73_W::new(self, 9)
    }
    ///Bit 10 - configurable event input x falling edge pending bit
    #[inline(always)]
    pub fn fpif74(&mut self) -> FPIF74_W<'_, FPR3rs> {
        FPIF74_W::new(self, 10)
    }
}
/**EXTI falling edge pending register

You can [`read`](crate::Reg::read) this register and get [`fpr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#EXTI:FPR3)*/
pub struct FPR3rs;
impl crate::RegisterSpec for FPR3rs {
    type Ux = u32;
}
///`read()` method returns [`fpr3::R`](R) reader structure
impl crate::Readable for FPR3rs {}
///`write(|w| ..)` method takes [`fpr3::W`](W) writer structure
impl crate::Writable for FPR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FPR3 to value 0
impl crate::Resettable for FPR3rs {}
