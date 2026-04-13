///Register `APB1RSTR2` reader
pub type R = crate::R<APB1RSTR2rs>;
///Register `APB1RSTR2` writer
pub type W = crate::W<APB1RSTR2rs>;
///Field `CRSRST` reader - clock recovery system reset
pub type CRSRST_R = crate::BitReader;
///Field `CRSRST` writer - clock recovery system reset
pub type CRSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MDIOSRST` reader - MDIOS block reset
pub type MDIOSRST_R = crate::BitReader;
///Field `MDIOSRST` writer - MDIOS block reset
pub type MDIOSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDCANRST` reader - FDCAN block reset
pub type FDCANRST_R = crate::BitReader;
///Field `FDCANRST` writer - FDCAN block reset
pub type FDCANRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UCPD1RST` reader - UCPD block reset
pub type UCPD1RST_R = crate::BitReader;
///Field `UCPD1RST` writer - UCPD block reset
pub type UCPD1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - clock recovery system reset
    #[inline(always)]
    pub fn crsrst(&self) -> CRSRST_R {
        CRSRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - MDIOS block reset
    #[inline(always)]
    pub fn mdiosrst(&self) -> MDIOSRST_R {
        MDIOSRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - FDCAN block reset
    #[inline(always)]
    pub fn fdcanrst(&self) -> FDCANRST_R {
        FDCANRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 27 - UCPD block reset
    #[inline(always)]
    pub fn ucpd1rst(&self) -> UCPD1RST_R {
        UCPD1RST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1RSTR2")
            .field("crsrst", &self.crsrst())
            .field("mdiosrst", &self.mdiosrst())
            .field("fdcanrst", &self.fdcanrst())
            .field("ucpd1rst", &self.ucpd1rst())
            .finish()
    }
}
impl W {
    ///Bit 1 - clock recovery system reset
    #[inline(always)]
    pub fn crsrst(&mut self) -> CRSRST_W<'_, APB1RSTR2rs> {
        CRSRST_W::new(self, 1)
    }
    ///Bit 5 - MDIOS block reset
    #[inline(always)]
    pub fn mdiosrst(&mut self) -> MDIOSRST_W<'_, APB1RSTR2rs> {
        MDIOSRST_W::new(self, 5)
    }
    ///Bit 8 - FDCAN block reset
    #[inline(always)]
    pub fn fdcanrst(&mut self) -> FDCANRST_W<'_, APB1RSTR2rs> {
        FDCANRST_W::new(self, 8)
    }
    ///Bit 27 - UCPD block reset
    #[inline(always)]
    pub fn ucpd1rst(&mut self) -> UCPD1RST_W<'_, APB1RSTR2rs> {
        UCPD1RST_W::new(self, 27)
    }
}
/**RCC APB1 peripheral reset register 2

You can [`read`](crate::Reg::read) this register and get [`apb1rstr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:APB1RSTR2)*/
pub struct APB1RSTR2rs;
impl crate::RegisterSpec for APB1RSTR2rs {
    type Ux = u32;
}
///`read()` method returns [`apb1rstr2::R`](R) reader structure
impl crate::Readable for APB1RSTR2rs {}
///`write(|w| ..)` method takes [`apb1rstr2::W`](W) writer structure
impl crate::Writable for APB1RSTR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1RSTR2 to value 0
impl crate::Resettable for APB1RSTR2rs {}
