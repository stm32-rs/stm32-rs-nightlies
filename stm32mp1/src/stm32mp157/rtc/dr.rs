///Register `DR` reader
pub type R = crate::R<DRrs>;
///Register `DR` writer
pub type W = crate::W<DRrs>;
///Field `DU` reader - DU
pub type DU_R = crate::FieldReader;
///Field `DU` writer - DU
pub type DU_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `DT` reader - DT
pub type DT_R = crate::FieldReader;
///Field `DT` writer - DT
pub type DT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
///Field `MU` reader - MU
pub type MU_R = crate::FieldReader;
///Field `MU` writer - MU
pub type MU_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `MT` reader - MT
pub type MT_R = crate::BitReader;
///Field `MT` writer - MT
pub type MT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDU` reader - WDU
pub type WDU_R = crate::FieldReader;
///Field `WDU` writer - WDU
pub type WDU_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `YU` reader - YU
pub type YU_R = crate::FieldReader;
///Field `YU` writer - YU
pub type YU_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `YT` reader - YT
pub type YT_R = crate::FieldReader;
///Field `YT` writer - YT
pub type YT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl R {
    ///Bits 0:3 - DU
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5 - DT
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:11 - MU
    #[inline(always)]
    pub fn mu(&self) -> MU_R {
        MU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - MT
    #[inline(always)]
    pub fn mt(&self) -> MT_R {
        MT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:15 - WDU
    #[inline(always)]
    pub fn wdu(&self) -> WDU_R {
        WDU_R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 16:19 - YU
    #[inline(always)]
    pub fn yu(&self) -> YU_R {
        YU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - YT
    #[inline(always)]
    pub fn yt(&self) -> YT_R {
        YT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR")
            .field("du", &self.du())
            .field("dt", &self.dt())
            .field("mu", &self.mu())
            .field("mt", &self.mt())
            .field("wdu", &self.wdu())
            .field("yu", &self.yu())
            .field("yt", &self.yt())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - DU
    #[inline(always)]
    pub fn du(&mut self) -> DU_W<'_, DRrs> {
        DU_W::new(self, 0)
    }
    ///Bits 4:5 - DT
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W<'_, DRrs> {
        DT_W::new(self, 4)
    }
    ///Bits 8:11 - MU
    #[inline(always)]
    pub fn mu(&mut self) -> MU_W<'_, DRrs> {
        MU_W::new(self, 8)
    }
    ///Bit 12 - MT
    #[inline(always)]
    pub fn mt(&mut self) -> MT_W<'_, DRrs> {
        MT_W::new(self, 12)
    }
    ///Bits 13:15 - WDU
    #[inline(always)]
    pub fn wdu(&mut self) -> WDU_W<'_, DRrs> {
        WDU_W::new(self, 13)
    }
    ///Bits 16:19 - YU
    #[inline(always)]
    pub fn yu(&mut self) -> YU_W<'_, DRrs> {
        YU_W::new(self, 16)
    }
    ///Bits 20:23 - YT
    #[inline(always)]
    pub fn yt(&mut self) -> YT_W<'_, DRrs> {
        YT_W::new(self, 20)
    }
}
/**The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.

You can [`read`](crate::Reg::read) this register and get [`dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RTC:DR)*/
pub struct DRrs;
impl crate::RegisterSpec for DRrs {
    type Ux = u32;
}
///`read()` method returns [`dr::R`](R) reader structure
impl crate::Readable for DRrs {}
///`write(|w| ..)` method takes [`dr::W`](W) writer structure
impl crate::Writable for DRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DR to value 0x2101
impl crate::Resettable for DRrs {
    const RESET_VALUE: u32 = 0x2101;
}
