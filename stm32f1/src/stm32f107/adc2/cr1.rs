///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `AWDCH` reader - Analog watchdog channel select bits
pub type AWDCH_R = crate::FieldReader;
///Field `AWDCH` writer - Analog watchdog channel select bits
pub type AWDCH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Analog watchdog interrupt enable
pub use crate::stm32f107::adc1::cr1::AWDIE;
///Field `AWDIE` reader - Analog watchdog interrupt enable
pub use crate::stm32f107::adc1::cr1::AWDIE_R;
///Field `AWDIE` writer - Analog watchdog interrupt enable
pub use crate::stm32f107::adc1::cr1::AWDIE_W;
///Enable the watchdog on a single channel in scan mode
pub use crate::stm32f107::adc1::cr1::AWDSGL;
///Field `AWDSGL` reader - Enable the watchdog on a single channel in scan mode
pub use crate::stm32f107::adc1::cr1::AWDSGL_R;
///Field `AWDSGL` writer - Enable the watchdog on a single channel in scan mode
pub use crate::stm32f107::adc1::cr1::AWDSGL_W;
///Discontinuous mode on regular channels
pub use crate::stm32f107::adc1::cr1::DISCEN;
///Field `DISCEN` reader - Discontinuous mode on regular channels
pub use crate::stm32f107::adc1::cr1::DISCEN_R;
///Field `DISCEN` writer - Discontinuous mode on regular channels
pub use crate::stm32f107::adc1::cr1::DISCEN_W;
///Interrupt enable for EOC
pub use crate::stm32f107::adc1::cr1::EOCIE;
///Field `EOCIE` reader - Interrupt enable for EOC
pub use crate::stm32f107::adc1::cr1::EOCIE_R;
///Field `EOCIE` writer - Interrupt enable for EOC
pub use crate::stm32f107::adc1::cr1::EOCIE_W;
///Automatic injected group conversion
pub use crate::stm32f107::adc1::cr1::JAUTO;
///Field `JAUTO` reader - Automatic injected group conversion
pub use crate::stm32f107::adc1::cr1::JAUTO_R;
///Field `JAUTO` writer - Automatic injected group conversion
pub use crate::stm32f107::adc1::cr1::JAUTO_W;
///Discontinuous mode on injected channels
pub use crate::stm32f107::adc1::cr1::JDISCEN;
///Field `JDISCEN` reader - Discontinuous mode on injected channels
pub use crate::stm32f107::adc1::cr1::JDISCEN_R;
///Field `JDISCEN` writer - Discontinuous mode on injected channels
pub use crate::stm32f107::adc1::cr1::JDISCEN_W;
///Interrupt enable for injected channels
pub use crate::stm32f107::adc1::cr1::JEOCIE;
///Field `JEOCIE` reader - Interrupt enable for injected channels
pub use crate::stm32f107::adc1::cr1::JEOCIE_R;
///Field `JEOCIE` writer - Interrupt enable for injected channels
pub use crate::stm32f107::adc1::cr1::JEOCIE_W;
///Scan mode
pub use crate::stm32f107::adc1::cr1::SCAN;
///Field `SCAN` reader - Scan mode
pub use crate::stm32f107::adc1::cr1::SCAN_R;
///Field `SCAN` writer - Scan mode
pub use crate::stm32f107::adc1::cr1::SCAN_W;
///Field `DISCNUM` reader - Discontinuous mode channel count
pub type DISCNUM_R = crate::FieldReader;
///Field `DISCNUM` writer - Discontinuous mode channel count
pub type DISCNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
///Analog watchdog enable on regular channels
pub use crate::stm32f107::adc1::cr1::AWDEN;
///Field `AWDEN` reader - Analog watchdog enable on regular channels
pub use crate::stm32f107::adc1::cr1::AWDEN_R;
///Field `AWDEN` writer - Analog watchdog enable on regular channels
pub use crate::stm32f107::adc1::cr1::AWDEN_W;
///Analog watchdog enable on injected channels
pub use crate::stm32f107::adc1::cr1::JAWDEN;
///Field `JAWDEN` reader - Analog watchdog enable on injected channels
pub use crate::stm32f107::adc1::cr1::JAWDEN_R;
///Field `JAWDEN` writer - Analog watchdog enable on injected channels
pub use crate::stm32f107::adc1::cr1::JAWDEN_W;
impl R {
    ///Bits 0:4 - Analog watchdog channel select bits
    #[inline(always)]
    pub fn awdch(&self) -> AWDCH_R {
        AWDCH_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 5 - Interrupt enable for EOC
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Analog watchdog interrupt enable
    #[inline(always)]
    pub fn awdie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Interrupt enable for injected channels
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Scan mode
    #[inline(always)]
    pub fn scan(&self) -> SCAN_R {
        SCAN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Enable the watchdog on a single channel in scan mode
    #[inline(always)]
    pub fn awdsgl(&self) -> AWDSGL_R {
        AWDSGL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Automatic injected group conversion
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Discontinuous mode on regular channels
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Discontinuous mode on injected channels
    #[inline(always)]
    pub fn jdiscen(&self) -> JDISCEN_R {
        JDISCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:15 - Discontinuous mode channel count
    #[inline(always)]
    pub fn discnum(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bit 22 - Analog watchdog enable on injected channels
    #[inline(always)]
    pub fn jawden(&self) -> JAWDEN_R {
        JAWDEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Analog watchdog enable on regular channels
    #[inline(always)]
    pub fn awden(&self) -> AWDEN_R {
        AWDEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("awden", &self.awden())
            .field("jawden", &self.jawden())
            .field("discnum", &self.discnum())
            .field("jdiscen", &self.jdiscen())
            .field("discen", &self.discen())
            .field("jauto", &self.jauto())
            .field("awdsgl", &self.awdsgl())
            .field("scan", &self.scan())
            .field("jeocie", &self.jeocie())
            .field("awdie", &self.awdie())
            .field("eocie", &self.eocie())
            .field("awdch", &self.awdch())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Analog watchdog channel select bits
    #[inline(always)]
    pub fn awdch(&mut self) -> AWDCH_W<'_, CR1rs> {
        AWDCH_W::new(self, 0)
    }
    ///Bit 5 - Interrupt enable for EOC
    #[inline(always)]
    pub fn eocie(&mut self) -> EOCIE_W<'_, CR1rs> {
        EOCIE_W::new(self, 5)
    }
    ///Bit 6 - Analog watchdog interrupt enable
    #[inline(always)]
    pub fn awdie(&mut self) -> AWDIE_W<'_, CR1rs> {
        AWDIE_W::new(self, 6)
    }
    ///Bit 7 - Interrupt enable for injected channels
    #[inline(always)]
    pub fn jeocie(&mut self) -> JEOCIE_W<'_, CR1rs> {
        JEOCIE_W::new(self, 7)
    }
    ///Bit 8 - Scan mode
    #[inline(always)]
    pub fn scan(&mut self) -> SCAN_W<'_, CR1rs> {
        SCAN_W::new(self, 8)
    }
    ///Bit 9 - Enable the watchdog on a single channel in scan mode
    #[inline(always)]
    pub fn awdsgl(&mut self) -> AWDSGL_W<'_, CR1rs> {
        AWDSGL_W::new(self, 9)
    }
    ///Bit 10 - Automatic injected group conversion
    #[inline(always)]
    pub fn jauto(&mut self) -> JAUTO_W<'_, CR1rs> {
        JAUTO_W::new(self, 10)
    }
    ///Bit 11 - Discontinuous mode on regular channels
    #[inline(always)]
    pub fn discen(&mut self) -> DISCEN_W<'_, CR1rs> {
        DISCEN_W::new(self, 11)
    }
    ///Bit 12 - Discontinuous mode on injected channels
    #[inline(always)]
    pub fn jdiscen(&mut self) -> JDISCEN_W<'_, CR1rs> {
        JDISCEN_W::new(self, 12)
    }
    ///Bits 13:15 - Discontinuous mode channel count
    #[inline(always)]
    pub fn discnum(&mut self) -> DISCNUM_W<'_, CR1rs> {
        DISCNUM_W::new(self, 13)
    }
    ///Bit 22 - Analog watchdog enable on injected channels
    #[inline(always)]
    pub fn jawden(&mut self) -> JAWDEN_W<'_, CR1rs> {
        JAWDEN_W::new(self, 22)
    }
    ///Bit 23 - Analog watchdog enable on regular channels
    #[inline(always)]
    pub fn awden(&mut self) -> AWDEN_W<'_, CR1rs> {
        AWDEN_W::new(self, 23)
    }
}
/**control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#ADC2:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1rs {}
