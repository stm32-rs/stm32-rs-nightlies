///Register `APB1HRSTR` reader
pub type R = crate::R<APB1HRSTRrs>;
///Register `APB1HRSTR` writer
pub type W = crate::W<APB1HRSTRrs>;
///Field `MDIOSRST` reader - MDIOS reset
pub type MDIOSRST_R = crate::BitReader;
///Field `MDIOSRST` writer - MDIOS reset
pub type MDIOSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDCANRST` reader - FDCAN reset
pub type FDCANRST_R = crate::BitReader;
///Field `FDCANRST` writer - FDCAN reset
pub type FDCANRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UCPD1RST` reader - UCPD1 reset
pub type UCPD1RST_R = crate::BitReader;
///Field `UCPD1RST` writer - UCPD1 reset
pub type UCPD1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 5 - MDIOS reset
    #[inline(always)]
    pub fn mdiosrst(&self) -> MDIOSRST_R {
        MDIOSRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - FDCAN reset
    #[inline(always)]
    pub fn fdcanrst(&self) -> FDCANRST_R {
        FDCANRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 18 - UCPD1 reset
    #[inline(always)]
    pub fn ucpd1rst(&self) -> UCPD1RST_R {
        UCPD1RST_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1HRSTR")
            .field("mdiosrst", &self.mdiosrst())
            .field("fdcanrst", &self.fdcanrst())
            .field("ucpd1rst", &self.ucpd1rst())
            .finish()
    }
}
impl W {
    ///Bit 5 - MDIOS reset
    #[inline(always)]
    pub fn mdiosrst(&mut self) -> MDIOSRST_W<'_, APB1HRSTRrs> {
        MDIOSRST_W::new(self, 5)
    }
    ///Bit 8 - FDCAN reset
    #[inline(always)]
    pub fn fdcanrst(&mut self) -> FDCANRST_W<'_, APB1HRSTRrs> {
        FDCANRST_W::new(self, 8)
    }
    ///Bit 18 - UCPD1 reset
    #[inline(always)]
    pub fn ucpd1rst(&mut self) -> UCPD1RST_W<'_, APB1HRSTRrs> {
        UCPD1RST_W::new(self, 18)
    }
}
/**RCC APB1H reset register

You can [`read`](crate::Reg::read) this register and get [`apb1hrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:APB1HRSTR)*/
pub struct APB1HRSTRrs;
impl crate::RegisterSpec for APB1HRSTRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1hrstr::R`](R) reader structure
impl crate::Readable for APB1HRSTRrs {}
///`write(|w| ..)` method takes [`apb1hrstr::W`](W) writer structure
impl crate::Writable for APB1HRSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1HRSTR to value 0
impl crate::Resettable for APB1HRSTRrs {}
