///Register `SWIER2` reader
pub type R = crate::R<SWIER2rs>;
///Register `SWIER2` writer
pub type W = crate::W<SWIER2rs>;
/**Software rising edge event trigger on line 34

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI2W {
    ///1: Generates an interrupt request
    Pend = 1,
}
impl From<SWI2W> for bool {
    #[inline(always)]
    fn from(variant: SWI2W) -> Self {
        variant as u8 != 0
    }
}
///Field `SWI2` reader - Software rising edge event trigger on line 34
pub type SWI2_R = crate::BitReader<SWI2W>;
impl SWI2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SWI2W> {
        match self.bits {
            true => Some(SWI2W::Pend),
            _ => None,
        }
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWI2W::Pend
    }
}
///Field `SWI2` writer - Software rising edge event trigger on line 34
pub type SWI2_W<'a, REG> = crate::BitWriter<'a, REG, SWI2W>;
impl<'a, REG> SWI2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut crate::W<REG> {
        self.variant(SWI2W::Pend)
    }
}
impl R {
    ///Bit 2 - Software rising edge event trigger on line 34
    #[inline(always)]
    pub fn swi2(&self) -> SWI2_R {
        SWI2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWIER2")
            .field("swi2", &self.swi2())
            .finish()
    }
}
impl W {
    ///Bit 2 - Software rising edge event trigger on line 34
    #[inline(always)]
    pub fn swi2(&mut self) -> SWI2_W<SWIER2rs> {
        SWI2_W::new(self, 2)
    }
}
/**EXTI software interrupt event register 2

You can [`read`](crate::Reg::read) this register and get [`swier2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0C1.html#EXTI:SWIER2)*/
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
