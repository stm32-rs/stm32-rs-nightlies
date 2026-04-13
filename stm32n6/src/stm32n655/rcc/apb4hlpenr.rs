///Register `APB4HLPENR` reader
pub type R = crate::R<APB4HLPENRrs>;
///Register `APB4HLPENR` writer
pub type W = crate::W<APB4HLPENRrs>;
///Field `SYSCFGLPEN` reader - SYSCFG sleep enable
pub type SYSCFGLPEN_R = crate::BitReader;
///Field `SYSCFGLPEN` writer - SYSCFG sleep enable
pub type SYSCFGLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSECLPEN` reader - BSEC sleep enable
pub type BSECLPEN_R = crate::BitReader;
///Field `BSECLPEN` writer - BSEC sleep enable
pub type BSECLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTSLPEN` reader - DTS sleep enable
pub type DTSLPEN_R = crate::BitReader;
///Field `DTSLPEN` writer - DTS sleep enable
pub type DTSLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSPERFMLPEN` reader - BUSPERFM sleep enable
pub type BUSPERFMLPEN_R = crate::BitReader;
///Field `BUSPERFMLPEN` writer - BUSPERFM sleep enable
pub type BUSPERFMLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SYSCFG sleep enable
    #[inline(always)]
    pub fn syscfglpen(&self) -> SYSCFGLPEN_R {
        SYSCFGLPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BSEC sleep enable
    #[inline(always)]
    pub fn bseclpen(&self) -> BSECLPEN_R {
        BSECLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DTS sleep enable
    #[inline(always)]
    pub fn dtslpen(&self) -> DTSLPEN_R {
        DTSLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - BUSPERFM sleep enable
    #[inline(always)]
    pub fn busperfmlpen(&self) -> BUSPERFMLPEN_R {
        BUSPERFMLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB4HLPENR")
            .field("syscfglpen", &self.syscfglpen())
            .field("bseclpen", &self.bseclpen())
            .field("dtslpen", &self.dtslpen())
            .field("busperfmlpen", &self.busperfmlpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - SYSCFG sleep enable
    #[inline(always)]
    pub fn syscfglpen(&mut self) -> SYSCFGLPEN_W<'_, APB4HLPENRrs> {
        SYSCFGLPEN_W::new(self, 0)
    }
    ///Bit 1 - BSEC sleep enable
    #[inline(always)]
    pub fn bseclpen(&mut self) -> BSECLPEN_W<'_, APB4HLPENRrs> {
        BSECLPEN_W::new(self, 1)
    }
    ///Bit 2 - DTS sleep enable
    #[inline(always)]
    pub fn dtslpen(&mut self) -> DTSLPEN_W<'_, APB4HLPENRrs> {
        DTSLPEN_W::new(self, 2)
    }
    ///Bit 4 - BUSPERFM sleep enable
    #[inline(always)]
    pub fn busperfmlpen(&mut self) -> BUSPERFMLPEN_W<'_, APB4HLPENRrs> {
        BUSPERFMLPEN_W::new(self, 4)
    }
}
/**RCC APB4H Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`apb4hlpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4hlpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:APB4HLPENR)*/
pub struct APB4HLPENRrs;
impl crate::RegisterSpec for APB4HLPENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb4hlpenr::R`](R) reader structure
impl crate::Readable for APB4HLPENRrs {}
///`write(|w| ..)` method takes [`apb4hlpenr::W`](W) writer structure
impl crate::Writable for APB4HLPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB4HLPENR to value 0x02
impl crate::Resettable for APB4HLPENRrs {
    const RESET_VALUE: u32 = 0x02;
}
