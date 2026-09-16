///Register `SWIER` reader
pub type R = crate::R<SWIERrs>;
///Register `SWIER` writer
pub type W = crate::W<SWIERrs>;
/**Software Interrupt on line %s

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
///Field `SWIER(0-22)` reader - Software Interrupt on line %s
pub type SWIER_R = crate::BitReader<SOFTWARE_INTERRUPT>;
impl SWIER_R {
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
///Field `SWIER(0-22)` writer - Software Interrupt on line %s
pub type SWIER_W<'a, REG> = crate::BitWriter<'a, REG, SOFTWARE_INTERRUPT>;
impl<'a, REG> SWIER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut crate::W<REG> {
        self.variant(SOFTWARE_INTERRUPT::Pend)
    }
}
impl R {
    ///Software Interrupt on line (0-22)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SWIER0` field.</div>
    #[inline(always)]
    pub fn swier(&self, n: u8) -> SWIER_R {
        #[allow(clippy::no_effect)]
        [(); 23][n as usize];
        SWIER_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Software Interrupt on line (0-22)
    #[inline(always)]
    pub fn swier_iter(&self) -> impl Iterator<Item = SWIER_R> + '_ {
        (0..23).map(move |n| SWIER_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Software Interrupt on line 0
    #[inline(always)]
    pub fn swier0(&self) -> SWIER_R {
        SWIER_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Software Interrupt on line 1
    #[inline(always)]
    pub fn swier1(&self) -> SWIER_R {
        SWIER_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Software Interrupt on line 2
    #[inline(always)]
    pub fn swier2(&self) -> SWIER_R {
        SWIER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Software Interrupt on line 3
    #[inline(always)]
    pub fn swier3(&self) -> SWIER_R {
        SWIER_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Software Interrupt on line 4
    #[inline(always)]
    pub fn swier4(&self) -> SWIER_R {
        SWIER_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Software Interrupt on line 5
    #[inline(always)]
    pub fn swier5(&self) -> SWIER_R {
        SWIER_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Software Interrupt on line 6
    #[inline(always)]
    pub fn swier6(&self) -> SWIER_R {
        SWIER_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Software Interrupt on line 7
    #[inline(always)]
    pub fn swier7(&self) -> SWIER_R {
        SWIER_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Software Interrupt on line 8
    #[inline(always)]
    pub fn swier8(&self) -> SWIER_R {
        SWIER_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Software Interrupt on line 9
    #[inline(always)]
    pub fn swier9(&self) -> SWIER_R {
        SWIER_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Software Interrupt on line 10
    #[inline(always)]
    pub fn swier10(&self) -> SWIER_R {
        SWIER_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Software Interrupt on line 11
    #[inline(always)]
    pub fn swier11(&self) -> SWIER_R {
        SWIER_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Software Interrupt on line 12
    #[inline(always)]
    pub fn swier12(&self) -> SWIER_R {
        SWIER_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Software Interrupt on line 13
    #[inline(always)]
    pub fn swier13(&self) -> SWIER_R {
        SWIER_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Software Interrupt on line 14
    #[inline(always)]
    pub fn swier14(&self) -> SWIER_R {
        SWIER_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Software Interrupt on line 15
    #[inline(always)]
    pub fn swier15(&self) -> SWIER_R {
        SWIER_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Software Interrupt on line 16
    #[inline(always)]
    pub fn swier16(&self) -> SWIER_R {
        SWIER_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Software Interrupt on line 17
    #[inline(always)]
    pub fn swier17(&self) -> SWIER_R {
        SWIER_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Software Interrupt on line 18
    #[inline(always)]
    pub fn swier18(&self) -> SWIER_R {
        SWIER_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Software Interrupt on line 19
    #[inline(always)]
    pub fn swier19(&self) -> SWIER_R {
        SWIER_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Software Interrupt on line 20
    #[inline(always)]
    pub fn swier20(&self) -> SWIER_R {
        SWIER_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Software Interrupt on line 21
    #[inline(always)]
    pub fn swier21(&self) -> SWIER_R {
        SWIER_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Software Interrupt on line 22
    #[inline(always)]
    pub fn swier22(&self) -> SWIER_R {
        SWIER_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWIER")
            .field("swier0", &self.swier0())
            .field("swier1", &self.swier1())
            .field("swier2", &self.swier2())
            .field("swier3", &self.swier3())
            .field("swier4", &self.swier4())
            .field("swier5", &self.swier5())
            .field("swier6", &self.swier6())
            .field("swier7", &self.swier7())
            .field("swier8", &self.swier8())
            .field("swier9", &self.swier9())
            .field("swier10", &self.swier10())
            .field("swier11", &self.swier11())
            .field("swier12", &self.swier12())
            .field("swier13", &self.swier13())
            .field("swier14", &self.swier14())
            .field("swier15", &self.swier15())
            .field("swier16", &self.swier16())
            .field("swier17", &self.swier17())
            .field("swier18", &self.swier18())
            .field("swier19", &self.swier19())
            .field("swier20", &self.swier20())
            .field("swier21", &self.swier21())
            .field("swier22", &self.swier22())
            .finish()
    }
}
impl W {
    ///Software Interrupt on line (0-22)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SWIER0` field.</div>
    #[inline(always)]
    pub fn swier(&mut self, n: u8) -> SWIER_W<'_, SWIERrs> {
        #[allow(clippy::no_effect)]
        [(); 23][n as usize];
        SWIER_W::new(self, n)
    }
    ///Bit 0 - Software Interrupt on line 0
    #[inline(always)]
    pub fn swier0(&mut self) -> SWIER_W<'_, SWIERrs> {
        SWIER_W::new(self, 0)
    }
    ///Bit 1 - Software Interrupt on line 1
    #[inline(always)]
    pub fn swier1(&mut self) -> SWIER_W<'_, SWIERrs> {
        SWIER_W::new(self, 1)
    }
    ///Bit 2 - Software Interrupt on line 2
    #[inline(always)]
    pub fn swier2(&mut self) -> SWIER_W<'_, SWIERrs> {
        SWIER_W::new(self, 2)
    }
    ///Bit 3 - Software Interrupt on line 3
    #[inline(always)]
    pub fn swier3(&mut self) -> SWIER_W<'_, SWIERrs> {
        SWIER_W::new(self, 3)
    }
    ///Bit 4 - Software Interrupt on line 4
    #[inline(always)]
    pub fn swier4(&mut self) -> SWIER_W<'_, SWIERrs> {
        SWIER_W::new(self, 4)
    }
    ///Bit 5 - Software Interrupt on line 5
    #[inline(always)]
    pub fn swier5(&mut self) -> SWIER_W<'_, SWIERrs> {
        SWIER_W::new(self, 5)
    }
    ///Bit 6 - Software Interrupt on line 6
    #[inline(always)]
    pub fn swier6(&mut self) -> SWIER_W<'_, SWIERrs> {
        SWIER_W::new(self, 6)
    }
    ///Bit 7 - Software Interrupt on line 7
    #[inline(always)]
    pub fn swier7(&mut self) -> SWIER_W<'_, SWIERrs> {
        SWIER_W::new(self, 7)
    }
    ///Bit 8 - Software Interrupt on line 8
    #[inline(always)]
    pub fn swier8(&mut self) -> SWIER_W<'_, SWIERrs> {
        SWIER_W::new(self, 8)
    }
    ///Bit 9 - Software Interrupt on line 9
    #[inline(always)]
    pub fn swier9(&mut self) -> SWIER_W<'_, SWIERrs> {
        SWIER_W::new(self, 9)
    }
    ///Bit 10 - Software Interrupt on line 10
    #[inline(always)]
    pub fn swier10(&mut self) -> SWIER_W<'_, SWIERrs> {
        SWIER_W::new(self, 10)
    }
    ///Bit 11 - Software Interrupt on line 11
    #[inline(always)]
    pub fn swier11(&mut self) -> SWIER_W<'_, SWIERrs> {
        SWIER_W::new(self, 11)
    }
    ///Bit 12 - Software Interrupt on line 12
    #[inline(always)]
    pub fn swier12(&mut self) -> SWIER_W<'_, SWIERrs> {
        SWIER_W::new(self, 12)
    }
    ///Bit 13 - Software Interrupt on line 13
    #[inline(always)]
    pub fn swier13(&mut self) -> SWIER_W<'_, SWIERrs> {
        SWIER_W::new(self, 13)
    }
    ///Bit 14 - Software Interrupt on line 14
    #[inline(always)]
    pub fn swier14(&mut self) -> SWIER_W<'_, SWIERrs> {
        SWIER_W::new(self, 14)
    }
    ///Bit 15 - Software Interrupt on line 15
    #[inline(always)]
    pub fn swier15(&mut self) -> SWIER_W<'_, SWIERrs> {
        SWIER_W::new(self, 15)
    }
    ///Bit 16 - Software Interrupt on line 16
    #[inline(always)]
    pub fn swier16(&mut self) -> SWIER_W<'_, SWIERrs> {
        SWIER_W::new(self, 16)
    }
    ///Bit 17 - Software Interrupt on line 17
    #[inline(always)]
    pub fn swier17(&mut self) -> SWIER_W<'_, SWIERrs> {
        SWIER_W::new(self, 17)
    }
    ///Bit 18 - Software Interrupt on line 18
    #[inline(always)]
    pub fn swier18(&mut self) -> SWIER_W<'_, SWIERrs> {
        SWIER_W::new(self, 18)
    }
    ///Bit 19 - Software Interrupt on line 19
    #[inline(always)]
    pub fn swier19(&mut self) -> SWIER_W<'_, SWIERrs> {
        SWIER_W::new(self, 19)
    }
    ///Bit 20 - Software Interrupt on line 20
    #[inline(always)]
    pub fn swier20(&mut self) -> SWIER_W<'_, SWIERrs> {
        SWIER_W::new(self, 20)
    }
    ///Bit 21 - Software Interrupt on line 21
    #[inline(always)]
    pub fn swier21(&mut self) -> SWIER_W<'_, SWIERrs> {
        SWIER_W::new(self, 21)
    }
    ///Bit 22 - Software Interrupt on line 22
    #[inline(always)]
    pub fn swier22(&mut self) -> SWIER_W<'_, SWIERrs> {
        SWIER_W::new(self, 22)
    }
}
/**Software interrupt event register (EXTI_SWIER)

You can [`read`](crate::Reg::read) this register and get [`swier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F413.html#EXTI:SWIER)*/
pub struct SWIERrs;
impl crate::RegisterSpec for SWIERrs {
    type Ux = u32;
}
///`read()` method returns [`swier::R`](R) reader structure
impl crate::Readable for SWIERrs {}
///`write(|w| ..)` method takes [`swier::W`](W) writer structure
impl crate::Writable for SWIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWIER to value 0
impl crate::Resettable for SWIERrs {}
