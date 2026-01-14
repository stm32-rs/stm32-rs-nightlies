///Register `WD_TH` reader
pub type R = crate::R<WD_THrs>;
///Register `WD_TH` writer
pub type W = crate::W<WD_THrs>;
///Field `WD_LT` reader - WD_LT\[11:0\]: analog watchdog low level threshold.
pub type WD_LT_R = crate::FieldReader<u16>;
///Field `WD_LT` writer - WD_LT\[11:0\]: analog watchdog low level threshold.
pub type WD_LT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `WD_HT` reader - WD_HT\[11:0\]: analog watchdog high level threshold.
pub type WD_HT_R = crate::FieldReader<u16>;
///Field `WD_HT` writer - WD_HT\[11:0\]: analog watchdog high level threshold.
pub type WD_HT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - WD_LT\[11:0\]: analog watchdog low level threshold.
    #[inline(always)]
    pub fn wd_lt(&self) -> WD_LT_R {
        WD_LT_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - WD_HT\[11:0\]: analog watchdog high level threshold.
    #[inline(always)]
    pub fn wd_ht(&self) -> WD_HT_R {
        WD_HT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WD_TH")
            .field("wd_lt", &self.wd_lt())
            .field("wd_ht", &self.wd_ht())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - WD_LT\[11:0\]: analog watchdog low level threshold.
    #[inline(always)]
    pub fn wd_lt(&mut self) -> WD_LT_W<'_, WD_THrs> {
        WD_LT_W::new(self, 0)
    }
    ///Bits 16:27 - WD_HT\[11:0\]: analog watchdog high level threshold.
    #[inline(always)]
    pub fn wd_ht(&mut self) -> WD_HT_W<'_, WD_THrs> {
        WD_HT_W::new(self, 16)
    }
}
/**WD_TH register

You can [`read`](crate::Reg::read) this register and get [`wd_th::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd_th::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#ADC:WD_TH)*/
pub struct WD_THrs;
impl crate::RegisterSpec for WD_THrs {
    type Ux = u32;
}
///`read()` method returns [`wd_th::R`](R) reader structure
impl crate::Readable for WD_THrs {}
///`write(|w| ..)` method takes [`wd_th::W`](W) writer structure
impl crate::Writable for WD_THrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WD_TH to value 0x0fff_0000
impl crate::Resettable for WD_THrs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
