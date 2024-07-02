///Register `CCIPR2` reader
pub type R = crate::R<CCIPR2rs>;
///Register `CCIPR2` writer
pub type W = crate::W<CCIPR2rs>;
///Field `USART11SEL` reader - USART11 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type USART11SEL_R = crate::FieldReader;
///Field `USART11SEL` writer - USART11 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type USART11SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `USART12SEL` reader - USART12 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type USART12SEL_R = crate::FieldReader;
///Field `USART12SEL` writer - USART12 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type USART12SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LPTIM1SEL` reader - LPTIM1 kernel clock source selection others: reserved, the kernel clock is disabled
pub type LPTIM1SEL_R = crate::FieldReader;
///Field `LPTIM1SEL` writer - LPTIM1 kernel clock source selection others: reserved, the kernel clock is disabled
pub type LPTIM1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LPTIM2SEL` reader - LPTIM2 kernel clock source selection others: reserved, the kernel clock is disabled
pub type LPTIM2SEL_R = crate::FieldReader;
///Field `LPTIM2SEL` writer - LPTIM2 kernel clock source selection others: reserved, the kernel clock is disabled
pub type LPTIM2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LPTIM3SEL` reader - LPTIM3 kernel clock source selection others: reserved, the kernel clock is disabled
pub type LPTIM3SEL_R = crate::FieldReader;
///Field `LPTIM3SEL` writer - LPTIM3 kernel clock source selection others: reserved, the kernel clock is disabled
pub type LPTIM3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LPTIM4SEL` reader - LPTIM4 kernel clock source selection others: reserved, the kernel clock is disabled
pub type LPTIM4SEL_R = crate::FieldReader;
///Field `LPTIM4SEL` writer - LPTIM4 kernel clock source selection others: reserved, the kernel clock is disabled
pub type LPTIM4SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LPTIM5SEL` reader - LPTIM5 kernel clock source selection others: reserved, the kernel clock is disabled
pub type LPTIM5SEL_R = crate::FieldReader;
///Field `LPTIM5SEL` writer - LPTIM5 kernel clock source selection others: reserved, the kernel clock is disabled
pub type LPTIM5SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LPTIM6SEL` reader - LPTIM6 kernel clock source selection others: reserved, the kernel clock is disabled
pub type LPTIM6SEL_R = crate::FieldReader;
///Field `LPTIM6SEL` writer - LPTIM6 kernel clock source selection others: reserved, the kernel clock is disabled
pub type LPTIM6SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - USART11 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn usart11sel(&self) -> USART11SEL_R {
        USART11SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - USART12 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn usart12sel(&self) -> USART12SEL_R {
        USART12SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - LPTIM1 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - LPTIM2 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:18 - LPTIM3 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn lptim3sel(&self) -> LPTIM3SEL_R {
        LPTIM3SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:22 - LPTIM4 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn lptim4sel(&self) -> LPTIM4SEL_R {
        LPTIM4SEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 24:26 - LPTIM5 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn lptim5sel(&self) -> LPTIM5SEL_R {
        LPTIM5SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 28:30 - LPTIM6 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn lptim6sel(&self) -> LPTIM6SEL_R {
        LPTIM6SEL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR2")
            .field("usart11sel", &self.usart11sel())
            .field("usart12sel", &self.usart12sel())
            .field("lptim1sel", &self.lptim1sel())
            .field("lptim2sel", &self.lptim2sel())
            .field("lptim3sel", &self.lptim3sel())
            .field("lptim4sel", &self.lptim4sel())
            .field("lptim5sel", &self.lptim5sel())
            .field("lptim6sel", &self.lptim6sel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - USART11 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn usart11sel(&mut self) -> USART11SEL_W<CCIPR2rs> {
        USART11SEL_W::new(self, 0)
    }
    ///Bits 4:6 - USART12 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn usart12sel(&mut self) -> USART12SEL_W<CCIPR2rs> {
        USART12SEL_W::new(self, 4)
    }
    ///Bits 8:10 - LPTIM1 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<CCIPR2rs> {
        LPTIM1SEL_W::new(self, 8)
    }
    ///Bits 12:14 - LPTIM2 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W<CCIPR2rs> {
        LPTIM2SEL_W::new(self, 12)
    }
    ///Bits 16:18 - LPTIM3 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn lptim3sel(&mut self) -> LPTIM3SEL_W<CCIPR2rs> {
        LPTIM3SEL_W::new(self, 16)
    }
    ///Bits 20:22 - LPTIM4 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn lptim4sel(&mut self) -> LPTIM4SEL_W<CCIPR2rs> {
        LPTIM4SEL_W::new(self, 20)
    }
    ///Bits 24:26 - LPTIM5 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn lptim5sel(&mut self) -> LPTIM5SEL_W<CCIPR2rs> {
        LPTIM5SEL_W::new(self, 24)
    }
    ///Bits 28:30 - LPTIM6 kernel clock source selection others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn lptim6sel(&mut self) -> LPTIM6SEL_W<CCIPR2rs> {
        LPTIM6SEL_W::new(self, 28)
    }
}
/**RCC kernel clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:CCIPR2)*/
pub struct CCIPR2rs;
impl crate::RegisterSpec for CCIPR2rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr2::R`](R) reader structure
impl crate::Readable for CCIPR2rs {}
///`write(|w| ..)` method takes [`ccipr2::W`](W) writer structure
impl crate::Writable for CCIPR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCIPR2 to value 0
impl crate::Resettable for CCIPR2rs {
    const RESET_VALUE: u32 = 0;
}
