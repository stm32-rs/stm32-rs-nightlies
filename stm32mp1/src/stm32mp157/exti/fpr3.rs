///Register `FPR3` reader
pub type R = crate::R<FPR3rs>;
///Register `FPR3` writer
pub type W = crate::W<FPR3rs>;
///Field `FPIF65` reader - FPIF65
pub type FPIF65_R = crate::BitReader;
///Field `FPIF65` writer - FPIF65
pub type FPIF65_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF66` reader - FPIF66
pub type FPIF66_R = crate::BitReader;
///Field `FPIF66` writer - FPIF66
pub type FPIF66_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF68` reader - FPIF68
pub type FPIF68_R = crate::BitReader;
///Field `FPIF68` writer - FPIF68
pub type FPIF68_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF73` reader - FPIF73
pub type FPIF73_R = crate::BitReader;
///Field `FPIF73` writer - FPIF73
pub type FPIF73_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF74` reader - FPIF74
pub type FPIF74_R = crate::BitReader;
///Field `FPIF74` writer - FPIF74
pub type FPIF74_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - FPIF65
    #[inline(always)]
    pub fn fpif65(&self) -> FPIF65_R {
        FPIF65_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FPIF66
    #[inline(always)]
    pub fn fpif66(&self) -> FPIF66_R {
        FPIF66_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - FPIF68
    #[inline(always)]
    pub fn fpif68(&self) -> FPIF68_R {
        FPIF68_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 9 - FPIF73
    #[inline(always)]
    pub fn fpif73(&self) -> FPIF73_R {
        FPIF73_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - FPIF74
    #[inline(always)]
    pub fn fpif74(&self) -> FPIF74_R {
        FPIF74_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPR3")
            .field("fpif65", &self.fpif65())
            .field("fpif66", &self.fpif66())
            .field("fpif68", &self.fpif68())
            .field("fpif73", &self.fpif73())
            .field("fpif74", &self.fpif74())
            .finish()
    }
}
impl W {
    ///Bit 1 - FPIF65
    #[inline(always)]
    pub fn fpif65(&mut self) -> FPIF65_W<'_, FPR3rs> {
        FPIF65_W::new(self, 1)
    }
    ///Bit 2 - FPIF66
    #[inline(always)]
    pub fn fpif66(&mut self) -> FPIF66_W<'_, FPR3rs> {
        FPIF66_W::new(self, 2)
    }
    ///Bit 4 - FPIF68
    #[inline(always)]
    pub fn fpif68(&mut self) -> FPIF68_W<'_, FPR3rs> {
        FPIF68_W::new(self, 4)
    }
    ///Bit 9 - FPIF73
    #[inline(always)]
    pub fn fpif73(&mut self) -> FPIF73_W<'_, FPR3rs> {
        FPIF73_W::new(self, 9)
    }
    ///Bit 10 - FPIF74
    #[inline(always)]
    pub fn fpif74(&mut self) -> FPIF74_W<'_, FPR3rs> {
        FPIF74_W::new(self, 10)
    }
}
/**Contains only register bits for configurable events.

You can [`read`](crate::Reg::read) this register and get [`fpr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#EXTI:FPR3)*/
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
