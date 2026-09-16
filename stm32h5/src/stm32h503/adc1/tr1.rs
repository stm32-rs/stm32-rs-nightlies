///Register `TR1` reader
pub type R = crate::R<TR1rs>;
///Register `TR1` writer
pub type W = crate::W<TR1rs>;
///Field `LT1` reader - Analog watchdog 1 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type LT1_R = crate::FieldReader<u16>;
///Field `LT1` writer - Analog watchdog 1 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type LT1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
///Field `AWDFILT` reader - Analog watchdog filtering parameter This bit is set and cleared by software. ... Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWDFILT_R = crate::FieldReader;
///Field `AWDFILT` writer - Analog watchdog filtering parameter This bit is set and cleared by software. ... Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type AWDFILT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `HT1` reader - Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type HT1_R = crate::FieldReader<u16>;
///Field `HT1` writer - Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type HT1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
impl R {
    ///Bits 0:11 - Analog watchdog 1 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn lt1(&self) -> LT1_R {
        LT1_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 12:14 - Analog watchdog filtering parameter This bit is set and cleared by software. ... Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awdfilt(&self) -> AWDFILT_R {
        AWDFILT_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:27 - Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn ht1(&self) -> HT1_R {
        HT1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TR1")
            .field("lt1", &self.lt1())
            .field("awdfilt", &self.awdfilt())
            .field("ht1", &self.ht1())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Analog watchdog 1 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn lt1(&mut self) -> LT1_W<'_, TR1rs> {
        LT1_W::new(self, 0)
    }
    ///Bits 12:14 - Analog watchdog filtering parameter This bit is set and cleared by software. ... Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn awdfilt(&mut self) -> AWDFILT_W<'_, TR1rs> {
        AWDFILT_W::new(self, 12)
    }
    ///Bits 16:27 - Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn ht1(&mut self) -> HT1_W<'_, TR1rs> {
        HT1_W::new(self, 16)
    }
}
/**ADC watchdog threshold register 1

You can [`read`](crate::Reg::read) this register and get [`tr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#ADC1:TR1)*/
pub struct TR1rs;
impl crate::RegisterSpec for TR1rs {
    type Ux = u32;
}
///`read()` method returns [`tr1::R`](R) reader structure
impl crate::Readable for TR1rs {}
///`write(|w| ..)` method takes [`tr1::W`](W) writer structure
impl crate::Writable for TR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TR1 to value 0x0fff_0000
impl crate::Resettable for TR1rs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
