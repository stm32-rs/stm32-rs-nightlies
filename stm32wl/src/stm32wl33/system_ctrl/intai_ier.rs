///Register `INTAI_IER` reader
pub type R = crate::R<INTAI_IERrs>;
///Register `INTAI_IER` writer
pub type W = crate::W<INTAI_IERrs>;
///Field `TX_IE` reader - TX_IE: interrupt enable on TX_SEQUENCE signal: 0: TX_SEQUENCE interrupt is disabled (default). 1: TX_SEQUENCE interrupt is enabled
pub type TX_IE_R = crate::BitReader;
///Field `TX_IE` writer - TX_IE: interrupt enable on TX_SEQUENCE signal: 0: TX_SEQUENCE interrupt is disabled (default). 1: TX_SEQUENCE interrupt is enabled
pub type TX_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_IE` reader - RX_IE: interrupt enable on RX_SEQUENCE signal: 0: RX_SEQUENCE interrupt is disabled (default). 1: RX_SEQUENCE interrupt is enabled
pub type RX_IE_R = crate::BitReader;
///Field `RX_IE` writer - RX_IE: interrupt enable on RX_SEQUENCE signal: 0: RX_SEQUENCE interrupt is disabled (default). 1: RX_SEQUENCE interrupt is enabled
pub type RX_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP_IE` reader - COMP_IE: interrupt enable on COMP_OUT signal: 0: COMP_OUT interrupt is disabled (default). 1: COMP_OUT interrupt is enabled
pub type COMP_IE_R = crate::BitReader;
///Field `COMP_IE` writer - COMP_IE: interrupt enable on COMP_OUT signal: 0: COMP_OUT interrupt is disabled (default). 1: COMP_OUT interrupt is enabled
pub type COMP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFIP_BUSY_STATUS_IE` reader - RFIP_BUSY_STATUS_IE: interrupt enable on RFIP_BUSY_STATUS signal: 0: RFIP_BUSY_STATUS interrupt is disabled (default). 1: RFIP_BUSY_STATUS interrupt is enabled
pub type RFIP_BUSY_STATUS_IE_R = crate::BitReader;
///Field `RFIP_BUSY_STATUS_IE` writer - RFIP_BUSY_STATUS_IE: interrupt enable on RFIP_BUSY_STATUS signal: 0: RFIP_BUSY_STATUS interrupt is disabled (default). 1: RFIP_BUSY_STATUS interrupt is enabled
pub type RFIP_BUSY_STATUS_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TX_IE: interrupt enable on TX_SEQUENCE signal: 0: TX_SEQUENCE interrupt is disabled (default). 1: TX_SEQUENCE interrupt is enabled
    #[inline(always)]
    pub fn tx_ie(&self) -> TX_IE_R {
        TX_IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RX_IE: interrupt enable on RX_SEQUENCE signal: 0: RX_SEQUENCE interrupt is disabled (default). 1: RX_SEQUENCE interrupt is enabled
    #[inline(always)]
    pub fn rx_ie(&self) -> RX_IE_R {
        RX_IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - COMP_IE: interrupt enable on COMP_OUT signal: 0: COMP_OUT interrupt is disabled (default). 1: COMP_OUT interrupt is enabled
    #[inline(always)]
    pub fn comp_ie(&self) -> COMP_IE_R {
        COMP_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RFIP_BUSY_STATUS_IE: interrupt enable on RFIP_BUSY_STATUS signal: 0: RFIP_BUSY_STATUS interrupt is disabled (default). 1: RFIP_BUSY_STATUS interrupt is enabled
    #[inline(always)]
    pub fn rfip_busy_status_ie(&self) -> RFIP_BUSY_STATUS_IE_R {
        RFIP_BUSY_STATUS_IE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTAI_IER")
            .field("tx_ie", &self.tx_ie())
            .field("rx_ie", &self.rx_ie())
            .field("comp_ie", &self.comp_ie())
            .field("rfip_busy_status_ie", &self.rfip_busy_status_ie())
            .finish()
    }
}
impl W {
    ///Bit 0 - TX_IE: interrupt enable on TX_SEQUENCE signal: 0: TX_SEQUENCE interrupt is disabled (default). 1: TX_SEQUENCE interrupt is enabled
    #[inline(always)]
    pub fn tx_ie(&mut self) -> TX_IE_W<'_, INTAI_IERrs> {
        TX_IE_W::new(self, 0)
    }
    ///Bit 1 - RX_IE: interrupt enable on RX_SEQUENCE signal: 0: RX_SEQUENCE interrupt is disabled (default). 1: RX_SEQUENCE interrupt is enabled
    #[inline(always)]
    pub fn rx_ie(&mut self) -> RX_IE_W<'_, INTAI_IERrs> {
        RX_IE_W::new(self, 1)
    }
    ///Bit 4 - COMP_IE: interrupt enable on COMP_OUT signal: 0: COMP_OUT interrupt is disabled (default). 1: COMP_OUT interrupt is enabled
    #[inline(always)]
    pub fn comp_ie(&mut self) -> COMP_IE_W<'_, INTAI_IERrs> {
        COMP_IE_W::new(self, 4)
    }
    ///Bit 5 - RFIP_BUSY_STATUS_IE: interrupt enable on RFIP_BUSY_STATUS signal: 0: RFIP_BUSY_STATUS interrupt is disabled (default). 1: RFIP_BUSY_STATUS interrupt is enabled
    #[inline(always)]
    pub fn rfip_busy_status_ie(&mut self) -> RFIP_BUSY_STATUS_IE_W<'_, INTAI_IERrs> {
        RFIP_BUSY_STATUS_IE_W::new(self, 5)
    }
}
/**INTAI_IER register

You can [`read`](crate::Reg::read) this register and get [`intai_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intai_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SYSTEM_CTRL:INTAI_IER)*/
pub struct INTAI_IERrs;
impl crate::RegisterSpec for INTAI_IERrs {
    type Ux = u32;
}
///`read()` method returns [`intai_ier::R`](R) reader structure
impl crate::Readable for INTAI_IERrs {}
///`write(|w| ..)` method takes [`intai_ier::W`](W) writer structure
impl crate::Writable for INTAI_IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INTAI_IER to value 0
impl crate::Resettable for INTAI_IERrs {}
