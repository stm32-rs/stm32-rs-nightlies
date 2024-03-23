#[doc = "Register `SWIER2` reader"]
pub type R = crate::R<SWIER2rs>;
#[doc = "Register `SWIER2` writer"]
pub type W = crate::W<SWIER2rs>;
#[doc = "Software rising edge event trigger on line 34\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI2W {
    #[doc = "1: Generates an interrupt request"]
    Pend = 1,
}
impl From<SWI2W> for bool {
    #[inline(always)]
    fn from(variant: SWI2W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI2` reader - Software rising edge event trigger on line 34"]
pub type SWI2_R = crate::BitReader<SWI2W>;
impl SWI2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SWI2W> {
        match self.bits {
            true => Some(SWI2W::Pend),
            _ => None,
        }
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWI2W::Pend
    }
}
#[doc = "Field `SWI2` writer - Software rising edge event trigger on line 34"]
pub type SWI2_W<'a, REG> = crate::BitWriter<'a, REG, SWI2W>;
impl<'a, REG> SWI2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut crate::W<REG> {
        self.variant(SWI2W::Pend)
    }
}
impl R {
    #[doc = "Bit 2 - Software rising edge event trigger on line 34"]
    #[inline(always)]
    pub fn swi2(&self) -> SWI2_R {
        SWI2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Software rising edge event trigger on line 34"]
    #[inline(always)]
    #[must_use]
    pub fn swi2(&mut self) -> SWI2_W<SWIER2rs> {
        SWI2_W::new(self, 2)
    }
}
#[doc = "EXTI software interrupt event register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swier2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swier2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWIER2rs;
impl crate::RegisterSpec for SWIER2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swier2::R`](R) reader structure"]
impl crate::Readable for SWIER2rs {}
#[doc = "`write(|w| ..)` method takes [`swier2::W`](W) writer structure"]
impl crate::Writable for SWIER2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWIER2 to value 0"]
impl crate::Resettable for SWIER2rs {
    const RESET_VALUE: u32 = 0;
}
