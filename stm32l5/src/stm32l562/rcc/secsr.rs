///Register `SECSR` reader
pub type R = crate::R<SECSRrs>;
///Register `SECSR` writer
pub type W = crate::W<SECSRrs>;
///Field `HSISECF` reader - HSISECF
pub type HSISECF_R = crate::BitReader;
///Field `HSISECF` writer - HSISECF
pub type HSISECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSESECF` reader - HSESECF
pub type HSESECF_R = crate::BitReader;
///Field `HSESECF` writer - HSESECF
pub type HSESECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSISECF` reader - MSISECF
pub type MSISECF_R = crate::BitReader;
///Field `MSISECF` writer - MSISECF
pub type MSISECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSISECF` reader - LSISECF
pub type LSISECF_R = crate::BitReader;
///Field `LSISECF` writer - LSISECF
pub type LSISECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSESECF` reader - LSESECF
pub type LSESECF_R = crate::BitReader;
///Field `LSESECF` writer - LSESECF
pub type LSESECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSCLKSECF` reader - SYSCLKSECF
pub type SYSCLKSECF_R = crate::BitReader;
///Field `SYSCLKSECF` writer - SYSCLKSECF
pub type SYSCLKSECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRESCSECF` reader - PRESCSECF
pub type PRESCSECF_R = crate::BitReader;
///Field `PRESCSECF` writer - PRESCSECF
pub type PRESCSECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLSECF` reader - PLLSECF
pub type PLLSECF_R = crate::BitReader;
///Field `PLLSECF` writer - PLLSECF
pub type PLLSECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLSAI1SECF` reader - PLLSAI1SECF
pub type PLLSAI1SECF_R = crate::BitReader;
///Field `PLLSAI1SECF` writer - PLLSAI1SECF
pub type PLLSAI1SECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLSAI2SECF` reader - PLLSAI2SECF
pub type PLLSAI2SECF_R = crate::BitReader;
///Field `PLLSAI2SECF` writer - PLLSAI2SECF
pub type PLLSAI2SECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK48MSECF` reader - CLK48MSECF
pub type CLK48MSECF_R = crate::BitReader;
///Field `CLK48MSECF` writer - CLK48MSECF
pub type CLK48MSECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSI48SECF` reader - HSI48SECF
pub type HSI48SECF_R = crate::BitReader;
///Field `HSI48SECF` writer - HSI48SECF
pub type HSI48SECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RMVFSECF` reader - RMVFSECF
pub type RMVFSECF_R = crate::BitReader;
///Field `RMVFSECF` writer - RMVFSECF
pub type RMVFSECF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - HSISECF
    #[inline(always)]
    pub fn hsisecf(&self) -> HSISECF_R {
        HSISECF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HSESECF
    #[inline(always)]
    pub fn hsesecf(&self) -> HSESECF_R {
        HSESECF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSISECF
    #[inline(always)]
    pub fn msisecf(&self) -> MSISECF_R {
        MSISECF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LSISECF
    #[inline(always)]
    pub fn lsisecf(&self) -> LSISECF_R {
        LSISECF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - LSESECF
    #[inline(always)]
    pub fn lsesecf(&self) -> LSESECF_R {
        LSESECF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SYSCLKSECF
    #[inline(always)]
    pub fn sysclksecf(&self) -> SYSCLKSECF_R {
        SYSCLKSECF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PRESCSECF
    #[inline(always)]
    pub fn prescsecf(&self) -> PRESCSECF_R {
        PRESCSECF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PLLSECF
    #[inline(always)]
    pub fn pllsecf(&self) -> PLLSECF_R {
        PLLSECF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PLLSAI1SECF
    #[inline(always)]
    pub fn pllsai1secf(&self) -> PLLSAI1SECF_R {
        PLLSAI1SECF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PLLSAI2SECF
    #[inline(always)]
    pub fn pllsai2secf(&self) -> PLLSAI2SECF_R {
        PLLSAI2SECF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CLK48MSECF
    #[inline(always)]
    pub fn clk48msecf(&self) -> CLK48MSECF_R {
        CLK48MSECF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - HSI48SECF
    #[inline(always)]
    pub fn hsi48secf(&self) -> HSI48SECF_R {
        HSI48SECF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - RMVFSECF
    #[inline(always)]
    pub fn rmvfsecf(&self) -> RMVFSECF_R {
        RMVFSECF_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECSR")
            .field("rmvfsecf", &self.rmvfsecf())
            .field("hsi48secf", &self.hsi48secf())
            .field("clk48msecf", &self.clk48msecf())
            .field("pllsai2secf", &self.pllsai2secf())
            .field("pllsai1secf", &self.pllsai1secf())
            .field("pllsecf", &self.pllsecf())
            .field("prescsecf", &self.prescsecf())
            .field("sysclksecf", &self.sysclksecf())
            .field("lsesecf", &self.lsesecf())
            .field("lsisecf", &self.lsisecf())
            .field("msisecf", &self.msisecf())
            .field("hsesecf", &self.hsesecf())
            .field("hsisecf", &self.hsisecf())
            .finish()
    }
}
impl W {
    ///Bit 0 - HSISECF
    #[inline(always)]
    pub fn hsisecf(&mut self) -> HSISECF_W<'_, SECSRrs> {
        HSISECF_W::new(self, 0)
    }
    ///Bit 1 - HSESECF
    #[inline(always)]
    pub fn hsesecf(&mut self) -> HSESECF_W<'_, SECSRrs> {
        HSESECF_W::new(self, 1)
    }
    ///Bit 2 - MSISECF
    #[inline(always)]
    pub fn msisecf(&mut self) -> MSISECF_W<'_, SECSRrs> {
        MSISECF_W::new(self, 2)
    }
    ///Bit 3 - LSISECF
    #[inline(always)]
    pub fn lsisecf(&mut self) -> LSISECF_W<'_, SECSRrs> {
        LSISECF_W::new(self, 3)
    }
    ///Bit 4 - LSESECF
    #[inline(always)]
    pub fn lsesecf(&mut self) -> LSESECF_W<'_, SECSRrs> {
        LSESECF_W::new(self, 4)
    }
    ///Bit 5 - SYSCLKSECF
    #[inline(always)]
    pub fn sysclksecf(&mut self) -> SYSCLKSECF_W<'_, SECSRrs> {
        SYSCLKSECF_W::new(self, 5)
    }
    ///Bit 6 - PRESCSECF
    #[inline(always)]
    pub fn prescsecf(&mut self) -> PRESCSECF_W<'_, SECSRrs> {
        PRESCSECF_W::new(self, 6)
    }
    ///Bit 7 - PLLSECF
    #[inline(always)]
    pub fn pllsecf(&mut self) -> PLLSECF_W<'_, SECSRrs> {
        PLLSECF_W::new(self, 7)
    }
    ///Bit 8 - PLLSAI1SECF
    #[inline(always)]
    pub fn pllsai1secf(&mut self) -> PLLSAI1SECF_W<'_, SECSRrs> {
        PLLSAI1SECF_W::new(self, 8)
    }
    ///Bit 9 - PLLSAI2SECF
    #[inline(always)]
    pub fn pllsai2secf(&mut self) -> PLLSAI2SECF_W<'_, SECSRrs> {
        PLLSAI2SECF_W::new(self, 9)
    }
    ///Bit 10 - CLK48MSECF
    #[inline(always)]
    pub fn clk48msecf(&mut self) -> CLK48MSECF_W<'_, SECSRrs> {
        CLK48MSECF_W::new(self, 10)
    }
    ///Bit 11 - HSI48SECF
    #[inline(always)]
    pub fn hsi48secf(&mut self) -> HSI48SECF_W<'_, SECSRrs> {
        HSI48SECF_W::new(self, 11)
    }
    ///Bit 12 - RMVFSECF
    #[inline(always)]
    pub fn rmvfsecf(&mut self) -> RMVFSECF_W<'_, SECSRrs> {
        RMVFSECF_W::new(self, 12)
    }
}
/**RCC secure status register

You can [`read`](crate::Reg::read) this register and get [`secsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:SECSR)*/
pub struct SECSRrs;
impl crate::RegisterSpec for SECSRrs {
    type Ux = u32;
}
///`read()` method returns [`secsr::R`](R) reader structure
impl crate::Readable for SECSRrs {}
///`write(|w| ..)` method takes [`secsr::W`](W) writer structure
impl crate::Writable for SECSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECSR to value 0
impl crate::Resettable for SECSRrs {}
