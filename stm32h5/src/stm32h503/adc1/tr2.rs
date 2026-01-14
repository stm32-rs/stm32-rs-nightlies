///Register `TR2` reader
pub type R = crate::R<TR2rs>;
///Register `TR2` writer
pub type W = crate::W<TR2rs>;
///Field `LT2` reader - Analog watchdog 2 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 2. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type LT2_R = crate::FieldReader;
///Field `LT2` writer - Analog watchdog 2 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 2. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type LT2_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `HT2` reader - Analog watchdog 2 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 2. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type HT2_R = crate::FieldReader;
///Field `HT2` writer - Analog watchdog 2 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 2. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type HT2_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bits 0:7 - Analog watchdog 2 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 2. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn lt2(&self) -> LT2_R {
        LT2_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - Analog watchdog 2 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 2. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn ht2(&self) -> HT2_R {
        HT2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TR2")
            .field("lt2", &self.lt2())
            .field("ht2", &self.ht2())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Analog watchdog 2 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 2. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn lt2(&mut self) -> LT2_W<'_, TR2rs> {
        LT2_W::new(self, 0)
    }
    ///Bits 16:23 - Analog watchdog 2 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 2. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn ht2(&mut self) -> HT2_W<'_, TR2rs> {
        HT2_W::new(self, 16)
    }
}
/**ADC watchdog threshold register 2

You can [`read`](crate::Reg::read) this register and get [`tr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#ADC1:TR2)*/
pub struct TR2rs;
impl crate::RegisterSpec for TR2rs {
    type Ux = u32;
}
///`read()` method returns [`tr2::R`](R) reader structure
impl crate::Readable for TR2rs {}
///`write(|w| ..)` method takes [`tr2::W`](W) writer structure
impl crate::Writable for TR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TR2 to value 0x00ff_0000
impl crate::Resettable for TR2rs {
    const RESET_VALUE: u32 = 0x00ff_0000;
}
