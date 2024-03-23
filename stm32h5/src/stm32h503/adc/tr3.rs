#[doc = "Register `TR3` reader"]
pub type R = crate::R<TR3rs>;
#[doc = "Register `TR3` writer"]
pub type W = crate::W<TR3rs>;
#[doc = "Field `LT3` reader - Analog watchdog 3 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 3. This watchdog compares the 8-bit of LT3 with the 8 MSB of the converted data. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type LT3_R = crate::FieldReader;
#[doc = "Field `LT3` writer - Analog watchdog 3 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 3. This watchdog compares the 8-bit of LT3 with the 8 MSB of the converted data. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type LT3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HT3` reader - Analog watchdog 3 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 3. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type HT3_R = crate::FieldReader;
#[doc = "Field `HT3` writer - Analog watchdog 3 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 3. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type HT3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Analog watchdog 3 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 3. This watchdog compares the 8-bit of LT3 with the 8 MSB of the converted data. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn lt3(&self) -> LT3_R {
        LT3_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Analog watchdog 3 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 3. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ht3(&self) -> HT3_R {
        HT3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Analog watchdog 3 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 3. This watchdog compares the 8-bit of LT3 with the 8 MSB of the converted data. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn lt3(&mut self) -> LT3_W<TR3rs> {
        LT3_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Analog watchdog 3 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 3. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn ht3(&mut self) -> HT3_W<TR3rs> {
        HT3_W::new(self, 16)
    }
}
#[doc = "ADC watchdog threshold register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TR3rs;
impl crate::RegisterSpec for TR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr3::R`](R) reader structure"]
impl crate::Readable for TR3rs {}
#[doc = "`write(|w| ..)` method takes [`tr3::W`](W) writer structure"]
impl crate::Writable for TR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR3 to value 0x00ff_0000"]
impl crate::Resettable for TR3rs {
    const RESET_VALUE: u32 = 0x00ff_0000;
}
