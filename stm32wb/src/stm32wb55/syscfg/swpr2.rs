#[doc = "Register `SWPR2` writer"]
pub type W = crate::W<SWPR2rs>;
#[doc = "Field `P32WP` writer - P32WP"]
pub type P32WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P33WP` writer - P33WP"]
pub type P33WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P34WP` writer - P34WP"]
pub type P34WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P35WP` writer - P35WP"]
pub type P35WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P36WP` writer - P36WP"]
pub type P36WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P37WP` writer - P37WP"]
pub type P37WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P38WP` writer - P38WP"]
pub type P38WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P39WP` writer - P39WP"]
pub type P39WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P40WP` writer - P40WP"]
pub type P40WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P41WP` writer - P41WP"]
pub type P41WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P42WP` writer - P42WP"]
pub type P42WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P43WP` writer - P43WP"]
pub type P43WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P44WP` writer - P44WP"]
pub type P44WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P45WP` writer - P45WP"]
pub type P45WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P46WP` writer - P46WP"]
pub type P46WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P47WP` writer - P47WP"]
pub type P47WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P48WP` writer - P48WP"]
pub type P48WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P49WP` writer - P49WP"]
pub type P49WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P50WP` writer - P50WP"]
pub type P50WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P51WP` writer - P51WP"]
pub type P51WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P52WP` writer - P52WP"]
pub type P52WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P53WP` writer - P53WP"]
pub type P53WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P54WP` writer - P54WP"]
pub type P54WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P55WP` writer - P55WP"]
pub type P55WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P56WP` writer - P56WP"]
pub type P56WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P57WP` writer - P57WP"]
pub type P57WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P58WP` writer - P58WP"]
pub type P58WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P59WP` writer - P59WP"]
pub type P59WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P60WP` writer - P60WP"]
pub type P60WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P61WP` writer - P61WP"]
pub type P61WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P62WP` writer - P62WP"]
pub type P62WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P63WP` writer - SRAM2 page 63 write protection"]
pub type P63WP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - P32WP"]
    #[inline(always)]
    #[must_use]
    pub fn p32wp(&mut self) -> P32WP_W<SWPR2rs> {
        P32WP_W::new(self, 0)
    }
    #[doc = "Bit 1 - P33WP"]
    #[inline(always)]
    #[must_use]
    pub fn p33wp(&mut self) -> P33WP_W<SWPR2rs> {
        P33WP_W::new(self, 1)
    }
    #[doc = "Bit 2 - P34WP"]
    #[inline(always)]
    #[must_use]
    pub fn p34wp(&mut self) -> P34WP_W<SWPR2rs> {
        P34WP_W::new(self, 2)
    }
    #[doc = "Bit 3 - P35WP"]
    #[inline(always)]
    #[must_use]
    pub fn p35wp(&mut self) -> P35WP_W<SWPR2rs> {
        P35WP_W::new(self, 3)
    }
    #[doc = "Bit 4 - P36WP"]
    #[inline(always)]
    #[must_use]
    pub fn p36wp(&mut self) -> P36WP_W<SWPR2rs> {
        P36WP_W::new(self, 4)
    }
    #[doc = "Bit 5 - P37WP"]
    #[inline(always)]
    #[must_use]
    pub fn p37wp(&mut self) -> P37WP_W<SWPR2rs> {
        P37WP_W::new(self, 5)
    }
    #[doc = "Bit 6 - P38WP"]
    #[inline(always)]
    #[must_use]
    pub fn p38wp(&mut self) -> P38WP_W<SWPR2rs> {
        P38WP_W::new(self, 6)
    }
    #[doc = "Bit 7 - P39WP"]
    #[inline(always)]
    #[must_use]
    pub fn p39wp(&mut self) -> P39WP_W<SWPR2rs> {
        P39WP_W::new(self, 7)
    }
    #[doc = "Bit 8 - P40WP"]
    #[inline(always)]
    #[must_use]
    pub fn p40wp(&mut self) -> P40WP_W<SWPR2rs> {
        P40WP_W::new(self, 8)
    }
    #[doc = "Bit 9 - P41WP"]
    #[inline(always)]
    #[must_use]
    pub fn p41wp(&mut self) -> P41WP_W<SWPR2rs> {
        P41WP_W::new(self, 9)
    }
    #[doc = "Bit 10 - P42WP"]
    #[inline(always)]
    #[must_use]
    pub fn p42wp(&mut self) -> P42WP_W<SWPR2rs> {
        P42WP_W::new(self, 10)
    }
    #[doc = "Bit 11 - P43WP"]
    #[inline(always)]
    #[must_use]
    pub fn p43wp(&mut self) -> P43WP_W<SWPR2rs> {
        P43WP_W::new(self, 11)
    }
    #[doc = "Bit 12 - P44WP"]
    #[inline(always)]
    #[must_use]
    pub fn p44wp(&mut self) -> P44WP_W<SWPR2rs> {
        P44WP_W::new(self, 12)
    }
    #[doc = "Bit 13 - P45WP"]
    #[inline(always)]
    #[must_use]
    pub fn p45wp(&mut self) -> P45WP_W<SWPR2rs> {
        P45WP_W::new(self, 13)
    }
    #[doc = "Bit 14 - P46WP"]
    #[inline(always)]
    #[must_use]
    pub fn p46wp(&mut self) -> P46WP_W<SWPR2rs> {
        P46WP_W::new(self, 14)
    }
    #[doc = "Bit 15 - P47WP"]
    #[inline(always)]
    #[must_use]
    pub fn p47wp(&mut self) -> P47WP_W<SWPR2rs> {
        P47WP_W::new(self, 15)
    }
    #[doc = "Bit 16 - P48WP"]
    #[inline(always)]
    #[must_use]
    pub fn p48wp(&mut self) -> P48WP_W<SWPR2rs> {
        P48WP_W::new(self, 16)
    }
    #[doc = "Bit 17 - P49WP"]
    #[inline(always)]
    #[must_use]
    pub fn p49wp(&mut self) -> P49WP_W<SWPR2rs> {
        P49WP_W::new(self, 17)
    }
    #[doc = "Bit 18 - P50WP"]
    #[inline(always)]
    #[must_use]
    pub fn p50wp(&mut self) -> P50WP_W<SWPR2rs> {
        P50WP_W::new(self, 18)
    }
    #[doc = "Bit 19 - P51WP"]
    #[inline(always)]
    #[must_use]
    pub fn p51wp(&mut self) -> P51WP_W<SWPR2rs> {
        P51WP_W::new(self, 19)
    }
    #[doc = "Bit 20 - P52WP"]
    #[inline(always)]
    #[must_use]
    pub fn p52wp(&mut self) -> P52WP_W<SWPR2rs> {
        P52WP_W::new(self, 20)
    }
    #[doc = "Bit 21 - P53WP"]
    #[inline(always)]
    #[must_use]
    pub fn p53wp(&mut self) -> P53WP_W<SWPR2rs> {
        P53WP_W::new(self, 21)
    }
    #[doc = "Bit 22 - P54WP"]
    #[inline(always)]
    #[must_use]
    pub fn p54wp(&mut self) -> P54WP_W<SWPR2rs> {
        P54WP_W::new(self, 22)
    }
    #[doc = "Bit 23 - P55WP"]
    #[inline(always)]
    #[must_use]
    pub fn p55wp(&mut self) -> P55WP_W<SWPR2rs> {
        P55WP_W::new(self, 23)
    }
    #[doc = "Bit 24 - P56WP"]
    #[inline(always)]
    #[must_use]
    pub fn p56wp(&mut self) -> P56WP_W<SWPR2rs> {
        P56WP_W::new(self, 24)
    }
    #[doc = "Bit 25 - P57WP"]
    #[inline(always)]
    #[must_use]
    pub fn p57wp(&mut self) -> P57WP_W<SWPR2rs> {
        P57WP_W::new(self, 25)
    }
    #[doc = "Bit 26 - P58WP"]
    #[inline(always)]
    #[must_use]
    pub fn p58wp(&mut self) -> P58WP_W<SWPR2rs> {
        P58WP_W::new(self, 26)
    }
    #[doc = "Bit 27 - P59WP"]
    #[inline(always)]
    #[must_use]
    pub fn p59wp(&mut self) -> P59WP_W<SWPR2rs> {
        P59WP_W::new(self, 27)
    }
    #[doc = "Bit 28 - P60WP"]
    #[inline(always)]
    #[must_use]
    pub fn p60wp(&mut self) -> P60WP_W<SWPR2rs> {
        P60WP_W::new(self, 28)
    }
    #[doc = "Bit 29 - P61WP"]
    #[inline(always)]
    #[must_use]
    pub fn p61wp(&mut self) -> P61WP_W<SWPR2rs> {
        P61WP_W::new(self, 29)
    }
    #[doc = "Bit 30 - P62WP"]
    #[inline(always)]
    #[must_use]
    pub fn p62wp(&mut self) -> P62WP_W<SWPR2rs> {
        P62WP_W::new(self, 30)
    }
    #[doc = "Bit 31 - SRAM2 page 63 write protection"]
    #[inline(always)]
    #[must_use]
    pub fn p63wp(&mut self) -> P63WP_W<SWPR2rs> {
        P63WP_W::new(self, 31)
    }
}
#[doc = "SRAM2 write protection register 2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swpr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWPR2rs;
impl crate::RegisterSpec for SWPR2rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swpr2::W`](W) writer structure"]
impl crate::Writable for SWPR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWPR2 to value 0"]
impl crate::Resettable for SWPR2rs {
    const RESET_VALUE: u32 = 0;
}
