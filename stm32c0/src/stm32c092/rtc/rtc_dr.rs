///Register `RTC_DR` reader
pub type R = crate::R<RTC_DRrs>;
///Register `RTC_DR` writer
pub type W = crate::W<RTC_DRrs>;
///Field `DU` reader - Date units in BCD format
pub type DU_R = crate::FieldReader;
///Field `DU` writer - Date units in BCD format
pub type DU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DT` reader - Date tens in BCD format
pub type DT_R = crate::FieldReader;
///Field `DT` writer - Date tens in BCD format
pub type DT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MU` reader - Month units in BCD format
pub type MU_R = crate::FieldReader;
///Field `MU` writer - Month units in BCD format
pub type MU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MT` reader - Month tens in BCD format
pub type MT_R = crate::BitReader;
///Field `MT` writer - Month tens in BCD format
pub type MT_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Week day units ...

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDU {
    ///0: forbidden
    B0x0 = 0,
    ///1: Monday
    B0x1 = 1,
    ///7: Sunday
    B0x7 = 7,
}
impl From<WDU> for u8 {
    #[inline(always)]
    fn from(variant: WDU) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDU {
    type Ux = u8;
}
impl crate::IsEnum for WDU {}
///Field `WDU` reader - Week day units ...
pub type WDU_R = crate::FieldReader<WDU>;
impl WDU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<WDU> {
        match self.bits {
            0 => Some(WDU::B0x0),
            1 => Some(WDU::B0x1),
            7 => Some(WDU::B0x7),
            _ => None,
        }
    }
    ///forbidden
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WDU::B0x0
    }
    ///Monday
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WDU::B0x1
    }
    ///Sunday
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == WDU::B0x7
    }
}
///Field `WDU` writer - Week day units ...
pub type WDU_W<'a, REG> = crate::FieldWriter<'a, REG, 3, WDU>;
impl<'a, REG> WDU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///forbidden
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WDU::B0x0)
    }
    ///Monday
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WDU::B0x1)
    }
    ///Sunday
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(WDU::B0x7)
    }
}
///Field `YU` reader - Year units in BCD format
pub type YU_R = crate::FieldReader;
///Field `YU` writer - Year units in BCD format
pub type YU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `YT` reader - Year tens in BCD format
pub type YT_R = crate::FieldReader;
///Field `YT` writer - Year tens in BCD format
pub type YT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Date units in BCD format
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5 - Date tens in BCD format
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:11 - Month units in BCD format
    #[inline(always)]
    pub fn mu(&self) -> MU_R {
        MU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - Month tens in BCD format
    #[inline(always)]
    pub fn mt(&self) -> MT_R {
        MT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:15 - Week day units ...
    #[inline(always)]
    pub fn wdu(&self) -> WDU_R {
        WDU_R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 16:19 - Year units in BCD format
    #[inline(always)]
    pub fn yu(&self) -> YU_R {
        YU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Year tens in BCD format
    #[inline(always)]
    pub fn yt(&self) -> YT_R {
        YT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_DR")
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
    ///Bits 0:3 - Date units in BCD format
    #[inline(always)]
    pub fn du(&mut self) -> DU_W<'_, RTC_DRrs> {
        DU_W::new(self, 0)
    }
    ///Bits 4:5 - Date tens in BCD format
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W<'_, RTC_DRrs> {
        DT_W::new(self, 4)
    }
    ///Bits 8:11 - Month units in BCD format
    #[inline(always)]
    pub fn mu(&mut self) -> MU_W<'_, RTC_DRrs> {
        MU_W::new(self, 8)
    }
    ///Bit 12 - Month tens in BCD format
    #[inline(always)]
    pub fn mt(&mut self) -> MT_W<'_, RTC_DRrs> {
        MT_W::new(self, 12)
    }
    ///Bits 13:15 - Week day units ...
    #[inline(always)]
    pub fn wdu(&mut self) -> WDU_W<'_, RTC_DRrs> {
        WDU_W::new(self, 13)
    }
    ///Bits 16:19 - Year units in BCD format
    #[inline(always)]
    pub fn yu(&mut self) -> YU_W<'_, RTC_DRrs> {
        YU_W::new(self, 16)
    }
    ///Bits 20:23 - Year tens in BCD format
    #[inline(always)]
    pub fn yt(&mut self) -> YT_W<'_, RTC_DRrs> {
        YT_W::new(self, 20)
    }
}
/**RTC date register

You can [`read`](crate::Reg::read) this register and get [`rtc_dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#RTC:RTC_DR)*/
pub struct RTC_DRrs;
impl crate::RegisterSpec for RTC_DRrs {
    type Ux = u32;
}
///`read()` method returns [`rtc_dr::R`](R) reader structure
impl crate::Readable for RTC_DRrs {}
///`write(|w| ..)` method takes [`rtc_dr::W`](W) writer structure
impl crate::Writable for RTC_DRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RTC_DR to value 0x2101
impl crate::Resettable for RTC_DRrs {
    const RESET_VALUE: u32 = 0x2101;
}
