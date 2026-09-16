///Register `RADIOENR` reader
pub type R = crate::R<RADIOENRrs>;
///Register `RADIOENR` writer
pub type W = crate::W<RADIOENRrs>;
///Field `BBCLKEN` reader - 2.4 GHz RADIO baseband kernel clock (aclk) enable Set and cleared by software. Note: The HSE32 oscillator needs to be enabled by either HSEON or STRADIOCLKON.
pub type BBCLKEN_R = crate::BitReader;
///Field `BBCLKEN` writer - 2.4 GHz RADIO baseband kernel clock (aclk) enable Set and cleared by software. Note: The HSE32 oscillator needs to be enabled by either HSEON or STRADIOCLKON.
pub type BBCLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STRADIOCLKON` reader - 2.4 GHz RADIO bus clock enable and HSE32 oscillator enable by 2.4 GHz RADIO sleep timer wakeup event Set by hardware on a 2.4 GHz RADIO sleep timer wakeup event. Cleared by software writing zero to this bit. Note: Before accessing the 2.4 GHz RADIO registers the RADIOCLKRDY bit must be checked.
pub type STRADIOCLKON_R = crate::BitReader;
///Field `STRADIOCLKON` writer - 2.4 GHz RADIO bus clock enable and HSE32 oscillator enable by 2.4 GHz RADIO sleep timer wakeup event Set by hardware on a 2.4 GHz RADIO sleep timer wakeup event. Cleared by software writing zero to this bit. Note: Before accessing the 2.4 GHz RADIO registers the RADIOCLKRDY bit must be checked.
pub type STRADIOCLKON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RADIOCLKRDY` reader - 2.4 GHz RADIO bus clock ready. Set and cleared by hardware to indicate that the 2.4 GHz RADIO bus clock is ready and the 2.4 GHz RADIO registers can be accessed. Note: Once both RADIOEN and STRADIOCLKON are cleared, RADIOCLKRDY goes low after three hclk5 clock cycles.
pub type RADIOCLKRDY_R = crate::BitReader;
impl R {
    ///Bit 1 - 2.4 GHz RADIO baseband kernel clock (aclk) enable Set and cleared by software. Note: The HSE32 oscillator needs to be enabled by either HSEON or STRADIOCLKON.
    #[inline(always)]
    pub fn bbclken(&self) -> BBCLKEN_R {
        BBCLKEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 16 - 2.4 GHz RADIO bus clock enable and HSE32 oscillator enable by 2.4 GHz RADIO sleep timer wakeup event Set by hardware on a 2.4 GHz RADIO sleep timer wakeup event. Cleared by software writing zero to this bit. Note: Before accessing the 2.4 GHz RADIO registers the RADIOCLKRDY bit must be checked.
    #[inline(always)]
    pub fn stradioclkon(&self) -> STRADIOCLKON_R {
        STRADIOCLKON_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - 2.4 GHz RADIO bus clock ready. Set and cleared by hardware to indicate that the 2.4 GHz RADIO bus clock is ready and the 2.4 GHz RADIO registers can be accessed. Note: Once both RADIOEN and STRADIOCLKON are cleared, RADIOCLKRDY goes low after three hclk5 clock cycles.
    #[inline(always)]
    pub fn radioclkrdy(&self) -> RADIOCLKRDY_R {
        RADIOCLKRDY_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RADIOENR")
            .field("bbclken", &self.bbclken())
            .field("stradioclkon", &self.stradioclkon())
            .field("radioclkrdy", &self.radioclkrdy())
            .finish()
    }
}
impl W {
    ///Bit 1 - 2.4 GHz RADIO baseband kernel clock (aclk) enable Set and cleared by software. Note: The HSE32 oscillator needs to be enabled by either HSEON or STRADIOCLKON.
    #[inline(always)]
    pub fn bbclken(&mut self) -> BBCLKEN_W<'_, RADIOENRrs> {
        BBCLKEN_W::new(self, 1)
    }
    ///Bit 16 - 2.4 GHz RADIO bus clock enable and HSE32 oscillator enable by 2.4 GHz RADIO sleep timer wakeup event Set by hardware on a 2.4 GHz RADIO sleep timer wakeup event. Cleared by software writing zero to this bit. Note: Before accessing the 2.4 GHz RADIO registers the RADIOCLKRDY bit must be checked.
    #[inline(always)]
    pub fn stradioclkon(&mut self) -> STRADIOCLKON_W<'_, RADIOENRrs> {
        STRADIOCLKON_W::new(self, 16)
    }
}
/**RCC RADIO peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`radioenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`radioenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#RCC:RADIOENR)*/
pub struct RADIOENRrs;
impl crate::RegisterSpec for RADIOENRrs {
    type Ux = u32;
}
///`read()` method returns [`radioenr::R`](R) reader structure
impl crate::Readable for RADIOENRrs {}
///`write(|w| ..)` method takes [`radioenr::W`](W) writer structure
impl crate::Writable for RADIOENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RADIOENR to value 0
impl crate::Resettable for RADIOENRrs {}
