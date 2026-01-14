///Register `SWIER2` reader
pub type R = crate::R<SWIER2rs>;
///Register `SWIER2` writer
pub type W = crate::W<SWIER2rs>;
/**Software interrupt on line 35

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFTWARE_INTERRUPT {
    ///1: Generates an interrupt request
    Pend = 1,
}
impl From<SOFTWARE_INTERRUPT> for bool {
    #[inline(always)]
    fn from(variant: SOFTWARE_INTERRUPT) -> Self {
        variant as u8 != 0
    }
}
///Field `SWI35` reader - Software interrupt on line 35
pub type SWI35_R = crate::BitReader<SOFTWARE_INTERRUPT>;
impl SWI35_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SOFTWARE_INTERRUPT> {
        match self.bits {
            true => Some(SOFTWARE_INTERRUPT::Pend),
            _ => None,
        }
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SOFTWARE_INTERRUPT::Pend
    }
}
///Field `SWI35` writer - Software interrupt on line 35
pub type SWI35_W<'a, REG> = crate::BitWriter<'a, REG, SOFTWARE_INTERRUPT>;
impl<'a, REG> SWI35_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut crate::W<REG> {
        self.variant(SOFTWARE_INTERRUPT::Pend)
    }
}
///Field `SWI36` reader - Software interrupt on line 36
pub use SWI35_R as SWI36_R;
///Field `SWI37` reader - Software interrupt on line 37
pub use SWI35_R as SWI37_R;
///Field `SWI38` reader - Software interrupt on line 38
pub use SWI35_R as SWI38_R;
///Field `SWI36` writer - Software interrupt on line 36
pub use SWI35_W as SWI36_W;
///Field `SWI37` writer - Software interrupt on line 37
pub use SWI35_W as SWI37_W;
///Field `SWI38` writer - Software interrupt on line 38
pub use SWI35_W as SWI38_W;
impl R {
    ///Bit 3 - Software interrupt on line 35
    #[inline(always)]
    pub fn swi35(&self) -> SWI35_R {
        SWI35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Software interrupt on line 36
    #[inline(always)]
    pub fn swi36(&self) -> SWI36_R {
        SWI36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Software interrupt on line 37
    #[inline(always)]
    pub fn swi37(&self) -> SWI37_R {
        SWI37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Software interrupt on line 38
    #[inline(always)]
    pub fn swi38(&self) -> SWI38_R {
        SWI38_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWIER2")
            .field("swi35", &self.swi35())
            .field("swi36", &self.swi36())
            .field("swi37", &self.swi37())
            .field("swi38", &self.swi38())
            .finish()
    }
}
impl W {
    ///Bit 3 - Software interrupt on line 35
    #[inline(always)]
    pub fn swi35(&mut self) -> SWI35_W<'_, SWIER2rs> {
        SWI35_W::new(self, 3)
    }
    ///Bit 4 - Software interrupt on line 36
    #[inline(always)]
    pub fn swi36(&mut self) -> SWI36_W<'_, SWIER2rs> {
        SWI36_W::new(self, 4)
    }
    ///Bit 5 - Software interrupt on line 37
    #[inline(always)]
    pub fn swi37(&mut self) -> SWI37_W<'_, SWIER2rs> {
        SWI37_W::new(self, 5)
    }
    ///Bit 6 - Software interrupt on line 38
    #[inline(always)]
    pub fn swi38(&mut self) -> SWI38_W<'_, SWIER2rs> {
        SWI38_W::new(self, 6)
    }
}
/**Software interrupt event register

You can [`read`](crate::Reg::read) this register and get [`swier2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#EXTI:SWIER2)*/
pub struct SWIER2rs;
impl crate::RegisterSpec for SWIER2rs {
    type Ux = u32;
}
///`read()` method returns [`swier2::R`](R) reader structure
impl crate::Readable for SWIER2rs {}
///`write(|w| ..)` method takes [`swier2::W`](W) writer structure
impl crate::Writable for SWIER2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWIER2 to value 0
impl crate::Resettable for SWIER2rs {}
