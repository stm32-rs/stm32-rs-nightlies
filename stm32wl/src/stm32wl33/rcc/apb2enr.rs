///Register `APB2ENR` reader
pub type R = crate::R<APB2ENRrs>;
///Register `APB2ENR` writer
pub type W = crate::W<APB2ENRrs>;
///Field `MRSUBGEN` reader - MRSUBG clock enable. Note: when this bit is '1', it must prevent clk_sys different from 16, 32, 64. If the configured clock is lower than 16MHz (1, 2, 4 or 8 MHz) or equal to 24MHz, clk_sys must be 16MHz 0: clock disable 1: clock enable
pub type MRSUBGEN_R = crate::BitReader;
///Field `MRSUBGEN` writer - MRSUBG clock enable. Note: when this bit is '1', it must prevent clk_sys different from 16, 32, 64. If the configured clock is lower than 16MHz (1, 2, 4 or 8 MHz) or equal to 24MHz, clk_sys must be 16MHz 0: clock disable 1: clock enable
pub type MRSUBGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPAWUREN` reader - Bubble clock enable Set and enable by software. 0: clock disable 1: clock enable
pub type LPAWUREN_R = crate::BitReader;
///Field `LPAWUREN` writer - Bubble clock enable Set and enable by software. 0: clock disable 1: clock enable
pub type LPAWUREN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - MRSUBG clock enable. Note: when this bit is '1', it must prevent clk_sys different from 16, 32, 64. If the configured clock is lower than 16MHz (1, 2, 4 or 8 MHz) or equal to 24MHz, clk_sys must be 16MHz 0: clock disable 1: clock enable
    #[inline(always)]
    pub fn mrsubgen(&self) -> MRSUBGEN_R {
        MRSUBGEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - Bubble clock enable Set and enable by software. 0: clock disable 1: clock enable
    #[inline(always)]
    pub fn lpawuren(&self) -> LPAWUREN_R {
        LPAWUREN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2ENR")
            .field("mrsubgen", &self.mrsubgen())
            .field("lpawuren", &self.lpawuren())
            .finish()
    }
}
impl W {
    ///Bit 0 - MRSUBG clock enable. Note: when this bit is '1', it must prevent clk_sys different from 16, 32, 64. If the configured clock is lower than 16MHz (1, 2, 4 or 8 MHz) or equal to 24MHz, clk_sys must be 16MHz 0: clock disable 1: clock enable
    #[inline(always)]
    pub fn mrsubgen(&mut self) -> MRSUBGEN_W<'_, APB2ENRrs> {
        MRSUBGEN_W::new(self, 0)
    }
    ///Bit 3 - Bubble clock enable Set and enable by software. 0: clock disable 1: clock enable
    #[inline(always)]
    pub fn lpawuren(&mut self) -> LPAWUREN_W<'_, APB2ENRrs> {
        LPAWUREN_W::new(self, 3)
    }
}
/**APB2ENR register

You can [`read`](crate::Reg::read) this register and get [`apb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RCC:APB2ENR)*/
pub struct APB2ENRrs;
impl crate::RegisterSpec for APB2ENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb2enr::R`](R) reader structure
impl crate::Readable for APB2ENRrs {}
///`write(|w| ..)` method takes [`apb2enr::W`](W) writer structure
impl crate::Writable for APB2ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2ENR to value 0
impl crate::Resettable for APB2ENRrs {}
