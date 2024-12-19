///Register `SWIER2` reader
pub type R = crate::R<SWIER2rs>;
///Register `SWIER2` writer
pub type W = crate::W<SWIER2rs>;
/**Software interrupt on event

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI34W {
    ///1: Generates an interrupt request
    Pend = 1,
}
impl From<SWI34W> for bool {
    #[inline(always)]
    fn from(variant: SWI34W) -> Self {
        variant as u8 != 0
    }
}
///Field `SWI34` reader - Software interrupt on event
pub type SWI34_R = crate::BitReader<SWI34W>;
impl SWI34_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SWI34W> {
        match self.bits {
            true => Some(SWI34W::Pend),
            _ => None,
        }
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWI34W::Pend
    }
}
///Field `SWI34` writer - Software interrupt on event
pub type SWI34_W<'a, REG> = crate::BitWriter<'a, REG, SWI34W>;
impl<'a, REG> SWI34_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut crate::W<REG> {
        self.variant(SWI34W::Pend)
    }
}
///Field `SWI40` reader - Software interrupt on event
pub use SWI34_R as SWI40_R;
///Field `SWI41` reader - Software interrupt on event
pub use SWI34_R as SWI41_R;
///Field `SWI45` reader - Software interrupt on event 45
pub use SWI34_R as SWI45_R;
///Field `SWI40` writer - Software interrupt on event
pub use SWI34_W as SWI40_W;
///Field `SWI41` writer - Software interrupt on event
pub use SWI34_W as SWI41_W;
///Field `SWI45` writer - Software interrupt on event 45
pub use SWI34_W as SWI45_W;
impl R {
    ///Bit 2 - Software interrupt on event
    #[inline(always)]
    pub fn swi34(&self) -> SWI34_R {
        SWI34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - Software interrupt on event
    #[inline(always)]
    pub fn swi40(&self) -> SWI40_R {
        SWI40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Software interrupt on event
    #[inline(always)]
    pub fn swi41(&self) -> SWI41_R {
        SWI41_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 13 - Software interrupt on event 45
    #[inline(always)]
    pub fn swi45(&self) -> SWI45_R {
        SWI45_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWIER2")
            .field("swi34", &self.swi34())
            .field("swi40", &self.swi40())
            .field("swi41", &self.swi41())
            .field("swi45", &self.swi45())
            .finish()
    }
}
impl W {
    ///Bit 2 - Software interrupt on event
    #[inline(always)]
    pub fn swi34(&mut self) -> SWI34_W<SWIER2rs> {
        SWI34_W::new(self, 2)
    }
    ///Bit 8 - Software interrupt on event
    #[inline(always)]
    pub fn swi40(&mut self) -> SWI40_W<SWIER2rs> {
        SWI40_W::new(self, 8)
    }
    ///Bit 9 - Software interrupt on event
    #[inline(always)]
    pub fn swi41(&mut self) -> SWI41_W<SWIER2rs> {
        SWI41_W::new(self, 9)
    }
    ///Bit 13 - Software interrupt on event 45
    #[inline(always)]
    pub fn swi45(&mut self) -> SWI45_W<SWIER2rs> {
        SWI45_W::new(self, 13)
    }
}
/**software interrupt event register

You can [`read`](crate::Reg::read) this register and get [`swier2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#EXTI:SWIER2)*/
pub struct SWIER2rs;
impl crate::RegisterSpec for SWIER2rs {
    type Ux = u32;
}
///`read()` method returns [`swier2::R`](R) reader structure
impl crate::Readable for SWIER2rs {}
///`write(|w| ..)` method takes [`swier2::W`](W) writer structure
impl crate::Writable for SWIER2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SWIER2 to value 0
impl crate::Resettable for SWIER2rs {
    const RESET_VALUE: u32 = 0;
}
