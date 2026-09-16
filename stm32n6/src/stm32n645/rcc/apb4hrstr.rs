///Register `APB4HRSTR` reader
pub type R = crate::R<APB4HRSTRrs>;
///Register `APB4HRSTR` writer
pub type W = crate::W<APB4HRSTRrs>;
///Field `SYSCFGRST` reader - SYSCFG reset
pub type SYSCFGRST_R = crate::BitReader;
///Field `SYSCFGRST` writer - SYSCFG reset
pub type SYSCFGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTSRST` reader - DTS reset
pub type DTSRST_R = crate::BitReader;
///Field `DTSRST` writer - DTS reset
pub type DTSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSPERFMRST` reader - BUSPERFM reset
pub type BUSPERFMRST_R = crate::BitReader;
///Field `BUSPERFMRST` writer - BUSPERFM reset
pub type BUSPERFMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SYSCFG reset
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - DTS reset
    #[inline(always)]
    pub fn dtsrst(&self) -> DTSRST_R {
        DTSRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - BUSPERFM reset
    #[inline(always)]
    pub fn busperfmrst(&self) -> BUSPERFMRST_R {
        BUSPERFMRST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB4HRSTR")
            .field("syscfgrst", &self.syscfgrst())
            .field("dtsrst", &self.dtsrst())
            .field("busperfmrst", &self.busperfmrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - SYSCFG reset
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<'_, APB4HRSTRrs> {
        SYSCFGRST_W::new(self, 0)
    }
    ///Bit 2 - DTS reset
    #[inline(always)]
    pub fn dtsrst(&mut self) -> DTSRST_W<'_, APB4HRSTRrs> {
        DTSRST_W::new(self, 2)
    }
    ///Bit 4 - BUSPERFM reset
    #[inline(always)]
    pub fn busperfmrst(&mut self) -> BUSPERFMRST_W<'_, APB4HRSTRrs> {
        BUSPERFMRST_W::new(self, 4)
    }
}
/**RCC APB4H reset register

You can [`read`](crate::Reg::read) this register and get [`apb4hrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4hrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:APB4HRSTR)*/
pub struct APB4HRSTRrs;
impl crate::RegisterSpec for APB4HRSTRrs {
    type Ux = u32;
}
///`read()` method returns [`apb4hrstr::R`](R) reader structure
impl crate::Readable for APB4HRSTRrs {}
///`write(|w| ..)` method takes [`apb4hrstr::W`](W) writer structure
impl crate::Writable for APB4HRSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB4HRSTR to value 0
impl crate::Resettable for APB4HRSTRrs {}
