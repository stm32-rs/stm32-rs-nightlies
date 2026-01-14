///Register `DR` reader
pub type R = crate::R<DRrs>;
///Register `DR` writer
pub type W = crate::W<DRrs>;
///Field `DU` reader - Date units in BCD format
pub type DU_R = crate::FieldReader;
///Field `DU` writer - Date units in BCD format
pub type DU_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `DT` reader - Date tens in BCD format
pub type DT_R = crate::FieldReader;
///Field `DT` writer - Date tens in BCD format
pub type DT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
///Field `MU` reader - Month units in BCD format
pub type MU_R = crate::FieldReader;
///Field `MU` writer - Month units in BCD format
pub type MU_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
/**Month tens in BCD format

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MT {
    ///0: Month tens is 0
    Zero = 0,
    ///1: Month tens is 1
    One = 1,
}
impl From<MT> for bool {
    #[inline(always)]
    fn from(variant: MT) -> Self {
        variant as u8 != 0
    }
}
///Field `MT` reader - Month tens in BCD format
pub type MT_R = crate::BitReader<MT>;
impl MT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MT {
        match self.bits {
            false => MT::Zero,
            true => MT::One,
        }
    }
    ///Month tens is 0
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == MT::Zero
    }
    ///Month tens is 1
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == MT::One
    }
}
///Field `MT` writer - Month tens in BCD format
pub type MT_W<'a, REG> = crate::BitWriter<'a, REG, MT>;
impl<'a, REG> MT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Month tens is 0
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(MT::Zero)
    }
    ///Month tens is 1
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(MT::One)
    }
}
///Field `WDU` reader - Week day units
pub type WDU_R = crate::FieldReader;
///Field `WDU` writer - Week day units
pub type WDU_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `YU` reader - Year units in BCD format
pub type YU_R = crate::FieldReader;
///Field `YU` writer - Year units in BCD format
pub type YU_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `YT` reader - Year tens in BCD format
pub type YT_R = crate::FieldReader;
///Field `YT` writer - Year tens in BCD format
pub type YT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
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
    ///Bits 13:15 - Week day units
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
        f.debug_struct("DR")
            .field("yt", &self.yt())
            .field("yu", &self.yu())
            .field("wdu", &self.wdu())
            .field("mt", &self.mt())
            .field("mu", &self.mu())
            .field("dt", &self.dt())
            .field("du", &self.du())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Date units in BCD format
    #[inline(always)]
    pub fn du(&mut self) -> DU_W<'_, DRrs> {
        DU_W::new(self, 0)
    }
    ///Bits 4:5 - Date tens in BCD format
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W<'_, DRrs> {
        DT_W::new(self, 4)
    }
    ///Bits 8:11 - Month units in BCD format
    #[inline(always)]
    pub fn mu(&mut self) -> MU_W<'_, DRrs> {
        MU_W::new(self, 8)
    }
    ///Bit 12 - Month tens in BCD format
    #[inline(always)]
    pub fn mt(&mut self) -> MT_W<'_, DRrs> {
        MT_W::new(self, 12)
    }
    ///Bits 13:15 - Week day units
    #[inline(always)]
    pub fn wdu(&mut self) -> WDU_W<'_, DRrs> {
        WDU_W::new(self, 13)
    }
    ///Bits 16:19 - Year units in BCD format
    #[inline(always)]
    pub fn yu(&mut self) -> YU_W<'_, DRrs> {
        YU_W::new(self, 16)
    }
    ///Bits 20:23 - Year tens in BCD format
    #[inline(always)]
    pub fn yt(&mut self) -> YT_W<'_, DRrs> {
        YT_W::new(self, 20)
    }
}
/**date register

You can [`read`](crate::Reg::read) this register and get [`dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F730.html#RTC:DR)*/
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
