///Register `FAST_RX_TIMER` reader
pub type R = crate::R<FAST_RX_TIMERrs>;
///Register `FAST_RX_TIMER` writer
pub type W = crate::W<FAST_RX_TIMERrs>;
///Field `FAST_RX_TIMEOUT` reader - Fast RX termination timer value (corresponding to the delay to measure the RSSI and to let the HW check CS flag information)
pub type FAST_RX_TIMEOUT_R = crate::FieldReader;
///Field `FAST_RX_TIMEOUT` writer - Fast RX termination timer value (corresponding to the delay to measure the RSSI and to let the HW check CS flag information)
pub type FAST_RX_TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `FAST_CS_TERM_EN` reader - Enable the Fast RX Termination feature
pub type FAST_CS_TERM_EN_R = crate::BitReader;
///Field `FAST_CS_TERM_EN` writer - Enable the Fast RX Termination feature
pub type FAST_CS_TERM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Fast RX termination timer value (corresponding to the delay to measure the RSSI and to let the HW check CS flag information)
    #[inline(always)]
    pub fn fast_rx_timeout(&self) -> FAST_RX_TIMEOUT_R {
        FAST_RX_TIMEOUT_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - Enable the Fast RX Termination feature
    #[inline(always)]
    pub fn fast_cs_term_en(&self) -> FAST_CS_TERM_EN_R {
        FAST_CS_TERM_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FAST_RX_TIMER")
            .field("fast_rx_timeout", &self.fast_rx_timeout())
            .field("fast_cs_term_en", &self.fast_cs_term_en())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Fast RX termination timer value (corresponding to the delay to measure the RSSI and to let the HW check CS flag information)
    #[inline(always)]
    pub fn fast_rx_timeout(&mut self) -> FAST_RX_TIMEOUT_W<'_, FAST_RX_TIMERrs> {
        FAST_RX_TIMEOUT_W::new(self, 0)
    }
    ///Bit 8 - Enable the Fast RX Termination feature
    #[inline(always)]
    pub fn fast_cs_term_en(&mut self) -> FAST_CS_TERM_EN_W<'_, FAST_RX_TIMERrs> {
        FAST_CS_TERM_EN_W::new(self, 8)
    }
}
/**FAST_RX_TIMER register

You can [`read`](crate::Reg::read) this register and get [`fast_rx_timer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fast_rx_timer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DYNAMIC_REG:FAST_RX_TIMER)*/
pub struct FAST_RX_TIMERrs;
impl crate::RegisterSpec for FAST_RX_TIMERrs {
    type Ux = u32;
}
///`read()` method returns [`fast_rx_timer::R`](R) reader structure
impl crate::Readable for FAST_RX_TIMERrs {}
///`write(|w| ..)` method takes [`fast_rx_timer::W`](W) writer structure
impl crate::Writable for FAST_RX_TIMERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FAST_RX_TIMER to value 0
impl crate::Resettable for FAST_RX_TIMERrs {}
