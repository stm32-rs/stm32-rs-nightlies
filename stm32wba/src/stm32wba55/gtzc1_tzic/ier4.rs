///Register `IER4` reader
pub type R = crate::R<IER4rs>;
///Register `IER4` writer
pub type W = crate::W<IER4rs>;
///Field `GPDMA1IE` reader - illegal access interrupt enable for GPDMA1
pub type GPDMA1IE_R = crate::BitReader;
///Field `GPDMA1IE` writer - illegal access interrupt enable for GPDMA1
pub type GPDMA1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHIE` reader - illegal access interrupt enable for FLASH memory
pub type FLASHIE_R = crate::BitReader;
///Field `FLASHIE` writer - illegal access interrupt enable for FLASH memory
pub type FLASHIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASH_REGIE` reader - illegal access interrupt enable for FLASH interface
pub type FLASH_REGIE_R = crate::BitReader;
///Field `FLASH_REGIE` writer - illegal access interrupt enable for FLASH interface
pub type FLASH_REGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSCFGIE` reader - illegal access interrupt enable for SYSCFG
pub type SYSCFGIE_R = crate::BitReader;
///Field `SYSCFGIE` writer - illegal access interrupt enable for SYSCFG
pub type SYSCFGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCIE` reader - illegal access interrupt enable for RTC
pub type RTCIE_R = crate::BitReader;
///Field `RTCIE` writer - illegal access interrupt enable for RTC
pub type RTCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMPIE` reader - illegal access interrupt enable for TAMP
pub type TAMPIE_R = crate::BitReader;
///Field `TAMPIE` writer - illegal access interrupt enable for TAMP
pub type TAMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRIE` reader - illegal access interrupt enable for PWR
pub type PWRIE_R = crate::BitReader;
///Field `PWRIE` writer - illegal access interrupt enable for PWR
pub type PWRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RCCIE` reader - illegal access interrupt enable for RCC
pub type RCCIE_R = crate::BitReader;
///Field `RCCIE` writer - illegal access interrupt enable for RCC
pub type RCCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTIIE` reader - illegal access interrupt enable for EXTI
pub type EXTIIE_R = crate::BitReader;
///Field `EXTIIE` writer - illegal access interrupt enable for EXTI
pub type EXTIIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZSCIE` reader - illegal access interrupt enable for GTZC1 TZSC
pub type TZSCIE_R = crate::BitReader;
///Field `TZSCIE` writer - illegal access interrupt enable for GTZC1 TZSC
pub type TZSCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZICIE` reader - illegal access interrupt enable for GTZC1 TZIC
pub type TZICIE_R = crate::BitReader;
///Field `TZICIE` writer - illegal access interrupt enable for GTZC1 TZIC
pub type TZICIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM1IE` reader - illegal access interrupt enable for SRAM1 memory
pub type SRAM1IE_R = crate::BitReader;
///Field `SRAM1IE` writer - illegal access interrupt enable for SRAM1 memory
pub type SRAM1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPCBB1IE` reader - illegal access interrupt enable for MPCBB1
pub type MPCBB1IE_R = crate::BitReader;
///Field `MPCBB1IE` writer - illegal access interrupt enable for MPCBB1
pub type MPCBB1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2IE` reader - illegal access interrupt enable for SRAM2 memory
pub type SRAM2IE_R = crate::BitReader;
///Field `SRAM2IE` writer - illegal access interrupt enable for SRAM2 memory
pub type SRAM2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPCBB2IE` reader - illegal access interrupt enable for MPCBB2
pub type MPCBB2IE_R = crate::BitReader;
///Field `MPCBB2IE` writer - illegal access interrupt enable for MPCBB2
pub type MPCBB2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM6IE` reader - illegal access interrupt enable for 2.4GHz RXTXRAM memory
pub type SRAM6IE_R = crate::BitReader;
///Field `SRAM6IE` writer - illegal access interrupt enable for 2.4GHz RXTXRAM memory
pub type SRAM6IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPCBB6IE` reader - illegal access interrupt enable for MPCBB6
pub type MPCBB6IE_R = crate::BitReader;
///Field `MPCBB6IE` writer - illegal access interrupt enable for MPCBB6
pub type MPCBB6IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - illegal access interrupt enable for GPDMA1
    #[inline(always)]
    pub fn gpdma1ie(&self) -> GPDMA1IE_R {
        GPDMA1IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access interrupt enable for FLASH memory
    #[inline(always)]
    pub fn flashie(&self) -> FLASHIE_R {
        FLASHIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access interrupt enable for FLASH interface
    #[inline(always)]
    pub fn flash_regie(&self) -> FLASH_REGIE_R {
        FLASH_REGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 7 - illegal access interrupt enable for SYSCFG
    #[inline(always)]
    pub fn syscfgie(&self) -> SYSCFGIE_R {
        SYSCFGIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access interrupt enable for RTC
    #[inline(always)]
    pub fn rtcie(&self) -> RTCIE_R {
        RTCIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access interrupt enable for TAMP
    #[inline(always)]
    pub fn tampie(&self) -> TAMPIE_R {
        TAMPIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access interrupt enable for PWR
    #[inline(always)]
    pub fn pwrie(&self) -> PWRIE_R {
        PWRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - illegal access interrupt enable for RCC
    #[inline(always)]
    pub fn rccie(&self) -> RCCIE_R {
        RCCIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - illegal access interrupt enable for EXTI
    #[inline(always)]
    pub fn extiie(&self) -> EXTIIE_R {
        EXTIIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access interrupt enable for GTZC1 TZSC
    #[inline(always)]
    pub fn tzscie(&self) -> TZSCIE_R {
        TZSCIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access interrupt enable for GTZC1 TZIC
    #[inline(always)]
    pub fn tzicie(&self) -> TZICIE_R {
        TZICIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 22 - illegal access interrupt enable for SRAM1 memory
    #[inline(always)]
    pub fn sram1ie(&self) -> SRAM1IE_R {
        SRAM1IE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - illegal access interrupt enable for MPCBB1
    #[inline(always)]
    pub fn mpcbb1ie(&self) -> MPCBB1IE_R {
        MPCBB1IE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - illegal access interrupt enable for SRAM2 memory
    #[inline(always)]
    pub fn sram2ie(&self) -> SRAM2IE_R {
        SRAM2IE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - illegal access interrupt enable for MPCBB2
    #[inline(always)]
    pub fn mpcbb2ie(&self) -> MPCBB2IE_R {
        MPCBB2IE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 30 - illegal access interrupt enable for 2.4GHz RXTXRAM memory
    #[inline(always)]
    pub fn sram6ie(&self) -> SRAM6IE_R {
        SRAM6IE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - illegal access interrupt enable for MPCBB6
    #[inline(always)]
    pub fn mpcbb6ie(&self) -> MPCBB6IE_R {
        MPCBB6IE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER4")
            .field("gpdma1ie", &self.gpdma1ie())
            .field("flashie", &self.flashie())
            .field("flash_regie", &self.flash_regie())
            .field("syscfgie", &self.syscfgie())
            .field("rtcie", &self.rtcie())
            .field("tampie", &self.tampie())
            .field("pwrie", &self.pwrie())
            .field("rccie", &self.rccie())
            .field("extiie", &self.extiie())
            .field("tzscie", &self.tzscie())
            .field("tzicie", &self.tzicie())
            .field("sram1ie", &self.sram1ie())
            .field("mpcbb1ie", &self.mpcbb1ie())
            .field("sram2ie", &self.sram2ie())
            .field("mpcbb2ie", &self.mpcbb2ie())
            .field("sram6ie", &self.sram6ie())
            .field("mpcbb6ie", &self.mpcbb6ie())
            .finish()
    }
}
impl W {
    ///Bit 0 - illegal access interrupt enable for GPDMA1
    #[inline(always)]
    pub fn gpdma1ie(&mut self) -> GPDMA1IE_W<'_, IER4rs> {
        GPDMA1IE_W::new(self, 0)
    }
    ///Bit 1 - illegal access interrupt enable for FLASH memory
    #[inline(always)]
    pub fn flashie(&mut self) -> FLASHIE_W<'_, IER4rs> {
        FLASHIE_W::new(self, 1)
    }
    ///Bit 2 - illegal access interrupt enable for FLASH interface
    #[inline(always)]
    pub fn flash_regie(&mut self) -> FLASH_REGIE_W<'_, IER4rs> {
        FLASH_REGIE_W::new(self, 2)
    }
    ///Bit 7 - illegal access interrupt enable for SYSCFG
    #[inline(always)]
    pub fn syscfgie(&mut self) -> SYSCFGIE_W<'_, IER4rs> {
        SYSCFGIE_W::new(self, 7)
    }
    ///Bit 8 - illegal access interrupt enable for RTC
    #[inline(always)]
    pub fn rtcie(&mut self) -> RTCIE_W<'_, IER4rs> {
        RTCIE_W::new(self, 8)
    }
    ///Bit 9 - illegal access interrupt enable for TAMP
    #[inline(always)]
    pub fn tampie(&mut self) -> TAMPIE_W<'_, IER4rs> {
        TAMPIE_W::new(self, 9)
    }
    ///Bit 10 - illegal access interrupt enable for PWR
    #[inline(always)]
    pub fn pwrie(&mut self) -> PWRIE_W<'_, IER4rs> {
        PWRIE_W::new(self, 10)
    }
    ///Bit 11 - illegal access interrupt enable for RCC
    #[inline(always)]
    pub fn rccie(&mut self) -> RCCIE_W<'_, IER4rs> {
        RCCIE_W::new(self, 11)
    }
    ///Bit 13 - illegal access interrupt enable for EXTI
    #[inline(always)]
    pub fn extiie(&mut self) -> EXTIIE_W<'_, IER4rs> {
        EXTIIE_W::new(self, 13)
    }
    ///Bit 14 - illegal access interrupt enable for GTZC1 TZSC
    #[inline(always)]
    pub fn tzscie(&mut self) -> TZSCIE_W<'_, IER4rs> {
        TZSCIE_W::new(self, 14)
    }
    ///Bit 15 - illegal access interrupt enable for GTZC1 TZIC
    #[inline(always)]
    pub fn tzicie(&mut self) -> TZICIE_W<'_, IER4rs> {
        TZICIE_W::new(self, 15)
    }
    ///Bit 22 - illegal access interrupt enable for SRAM1 memory
    #[inline(always)]
    pub fn sram1ie(&mut self) -> SRAM1IE_W<'_, IER4rs> {
        SRAM1IE_W::new(self, 22)
    }
    ///Bit 23 - illegal access interrupt enable for MPCBB1
    #[inline(always)]
    pub fn mpcbb1ie(&mut self) -> MPCBB1IE_W<'_, IER4rs> {
        MPCBB1IE_W::new(self, 23)
    }
    ///Bit 24 - illegal access interrupt enable for SRAM2 memory
    #[inline(always)]
    pub fn sram2ie(&mut self) -> SRAM2IE_W<'_, IER4rs> {
        SRAM2IE_W::new(self, 24)
    }
    ///Bit 25 - illegal access interrupt enable for MPCBB2
    #[inline(always)]
    pub fn mpcbb2ie(&mut self) -> MPCBB2IE_W<'_, IER4rs> {
        MPCBB2IE_W::new(self, 25)
    }
    ///Bit 30 - illegal access interrupt enable for 2.4GHz RXTXRAM memory
    #[inline(always)]
    pub fn sram6ie(&mut self) -> SRAM6IE_W<'_, IER4rs> {
        SRAM6IE_W::new(self, 30)
    }
    ///Bit 31 - illegal access interrupt enable for MPCBB6
    #[inline(always)]
    pub fn mpcbb6ie(&mut self) -> MPCBB6IE_W<'_, IER4rs> {
        MPCBB6IE_W::new(self, 31)
    }
}
/**GTZC1 TZIC interrupt enable register 4

You can [`read`](crate::Reg::read) this register and get [`ier4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#GTZC1_TZIC:IER4)*/
pub struct IER4rs;
impl crate::RegisterSpec for IER4rs {
    type Ux = u32;
}
///`read()` method returns [`ier4::R`](R) reader structure
impl crate::Readable for IER4rs {}
///`write(|w| ..)` method takes [`ier4::W`](W) writer structure
impl crate::Writable for IER4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER4 to value 0
impl crate::Resettable for IER4rs {}
