///Register `TSDR` reader
pub type R = crate::R<TSDRrs>;
///Register `TSDR` writer
pub type W = crate::W<TSDRrs>;
///Field `DU` reader - Date units in BCD format.
pub type DU_R = crate::FieldReader;
///Field `DU` writer - Date units in BCD format.
pub type DU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DT` reader - Date tens in BCD format.
pub type DT_R = crate::FieldReader;
///Field `DT` writer - Date tens in BCD format.
pub type DT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MU` reader - Month units in BCD format.
pub type MU_R = crate::FieldReader;
///Field `MU` writer - Month units in BCD format.
pub type MU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MT` reader - Month tens in BCD format.
pub type MT_R = crate::BitReader;
///Field `MT` writer - Month tens in BCD format.
pub type MT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDU` reader - Week day units
pub type WDU_R = crate::FieldReader;
///Field `WDU` writer - Week day units
pub type WDU_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:3 - Date units in BCD format.
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5 - Date tens in BCD format.
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:11 - Month units in BCD format.
    #[inline(always)]
    pub fn mu(&self) -> MU_R {
        MU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - Month tens in BCD format.
    #[inline(always)]
    pub fn mt(&self) -> MT_R {
        MT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:15 - Week day units
    #[inline(always)]
    pub fn wdu(&self) -> WDU_R {
        WDU_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSDR")
            .field("du", &self.du())
            .field("dt", &self.dt())
            .field("mu", &self.mu())
            .field("mt", &self.mt())
            .field("wdu", &self.wdu())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Date units in BCD format.
    #[inline(always)]
    pub fn du(&mut self) -> DU_W<'_, TSDRrs> {
        DU_W::new(self, 0)
    }
    ///Bits 4:5 - Date tens in BCD format.
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W<'_, TSDRrs> {
        DT_W::new(self, 4)
    }
    ///Bits 8:11 - Month units in BCD format.
    #[inline(always)]
    pub fn mu(&mut self) -> MU_W<'_, TSDRrs> {
        MU_W::new(self, 8)
    }
    ///Bit 12 - Month tens in BCD format.
    #[inline(always)]
    pub fn mt(&mut self) -> MT_W<'_, TSDRrs> {
        MT_W::new(self, 12)
    }
    ///Bits 13:15 - Week day units
    #[inline(always)]
    pub fn wdu(&mut self) -> WDU_W<'_, TSDRrs> {
        WDU_W::new(self, 13)
    }
}
/**RTC_TSDR register

You can [`read`](crate::Reg::read) this register and get [`tsdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RTC:TSDR)*/
pub struct TSDRrs;
impl crate::RegisterSpec for TSDRrs {
    type Ux = u32;
}
///`read()` method returns [`tsdr::R`](R) reader structure
impl crate::Readable for TSDRrs {}
///`write(|w| ..)` method takes [`tsdr::W`](W) writer structure
impl crate::Writable for TSDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TSDR to value 0
impl crate::Resettable for TSDRrs {}
