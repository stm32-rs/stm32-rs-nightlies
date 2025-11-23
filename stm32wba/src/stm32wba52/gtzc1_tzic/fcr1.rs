///Register `FCR1` writer
pub type W = crate::W<FCR1rs>;
///Field `CTIM2F` writer - clear the illegal access flag for TIM2
pub type CTIM2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM3F` writer - clear the illegal access flag for TIM3
pub type CTIM3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWWDGF` writer - clear the illegal access flag for WWDG
pub type CWWDGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CIWDGF` writer - clear the illegal access flag for IWDG
pub type CIWDGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CUSART2F` writer - clear the illegal access flag for USART2
pub type CUSART2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CI2C1F` writer - clear the illegal access flag for I2C1
pub type CI2C1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLPTIM2F` writer - clear the illegal access flag for LPTIM2
pub type CLPTIM2F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FCR1rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - clear the illegal access flag for TIM2
    #[inline(always)]
    pub fn ctim2f(&mut self) -> CTIM2F_W<'_, FCR1rs> {
        CTIM2F_W::new(self, 0)
    }
    ///Bit 1 - clear the illegal access flag for TIM3
    #[inline(always)]
    pub fn ctim3f(&mut self) -> CTIM3F_W<'_, FCR1rs> {
        CTIM3F_W::new(self, 1)
    }
    ///Bit 6 - clear the illegal access flag for WWDG
    #[inline(always)]
    pub fn cwwdgf(&mut self) -> CWWDGF_W<'_, FCR1rs> {
        CWWDGF_W::new(self, 6)
    }
    ///Bit 7 - clear the illegal access flag for IWDG
    #[inline(always)]
    pub fn ciwdgf(&mut self) -> CIWDGF_W<'_, FCR1rs> {
        CIWDGF_W::new(self, 7)
    }
    ///Bit 9 - clear the illegal access flag for USART2
    #[inline(always)]
    pub fn cusart2f(&mut self) -> CUSART2F_W<'_, FCR1rs> {
        CUSART2F_W::new(self, 9)
    }
    ///Bit 13 - clear the illegal access flag for I2C1
    #[inline(always)]
    pub fn ci2c1f(&mut self) -> CI2C1F_W<'_, FCR1rs> {
        CI2C1F_W::new(self, 13)
    }
    ///Bit 17 - clear the illegal access flag for LPTIM2
    #[inline(always)]
    pub fn clptim2f(&mut self) -> CLPTIM2F_W<'_, FCR1rs> {
        CLPTIM2F_W::new(self, 17)
    }
}
/**TZIC flag clear register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GTZC1_TZIC:FCR1)*/
pub struct FCR1rs;
impl crate::RegisterSpec for FCR1rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fcr1::W`](W) writer structure
impl crate::Writable for FCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCR1 to value 0
impl crate::Resettable for FCR1rs {}
