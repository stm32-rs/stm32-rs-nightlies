///Register `RTSR` reader
pub type R = crate::R<RTSRrs>;
///Register `RTSR` writer
pub type W = crate::W<RTSRrs>;
/**Rising trigger event configuration of line %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TR0 {
    ///0: Rising edge trigger is disabled
    Disabled = 0,
    ///1: Rising edge trigger is enabled
    Enabled = 1,
}
impl From<TR0> for bool {
    #[inline(always)]
    fn from(variant: TR0) -> Self {
        variant as u8 != 0
    }
}
///Field `TR(0-23)` reader - Rising trigger event configuration of line %s
pub type TR_R = crate::BitReader<TR0>;
impl TR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TR0 {
        match self.bits {
            false => TR0::Disabled,
            true => TR0::Enabled,
        }
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TR0::Disabled
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TR0::Enabled
    }
}
///Field `TR(0-23)` writer - Rising trigger event configuration of line %s
pub type TR_W<'a, REG> = crate::BitWriter<'a, REG, TR0>;
impl<'a, REG> TR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TR0::Disabled)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TR0::Enabled)
    }
}
impl R {
    ///Rising trigger event configuration of line (0-23)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TR0` field.</div>
    #[inline(always)]
    pub fn tr(&self, n: u8) -> TR_R {
        #[allow(clippy::no_effect)]
        [(); 24][n as usize];
        TR_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Rising trigger event configuration of line (0-23)
    #[inline(always)]
    pub fn tr_iter(&self) -> impl Iterator<Item = TR_R> + '_ {
        (0..24).map(move |n| TR_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Rising trigger event configuration of line 0
    #[inline(always)]
    pub fn tr0(&self) -> TR_R {
        TR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Rising trigger event configuration of line 1
    #[inline(always)]
    pub fn tr1(&self) -> TR_R {
        TR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Rising trigger event configuration of line 2
    #[inline(always)]
    pub fn tr2(&self) -> TR_R {
        TR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Rising trigger event configuration of line 3
    #[inline(always)]
    pub fn tr3(&self) -> TR_R {
        TR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rising trigger event configuration of line 4
    #[inline(always)]
    pub fn tr4(&self) -> TR_R {
        TR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rising trigger event configuration of line 5
    #[inline(always)]
    pub fn tr5(&self) -> TR_R {
        TR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Rising trigger event configuration of line 6
    #[inline(always)]
    pub fn tr6(&self) -> TR_R {
        TR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Rising trigger event configuration of line 7
    #[inline(always)]
    pub fn tr7(&self) -> TR_R {
        TR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Rising trigger event configuration of line 8
    #[inline(always)]
    pub fn tr8(&self) -> TR_R {
        TR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Rising trigger event configuration of line 9
    #[inline(always)]
    pub fn tr9(&self) -> TR_R {
        TR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Rising trigger event configuration of line 10
    #[inline(always)]
    pub fn tr10(&self) -> TR_R {
        TR_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Rising trigger event configuration of line 11
    #[inline(always)]
    pub fn tr11(&self) -> TR_R {
        TR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Rising trigger event configuration of line 12
    #[inline(always)]
    pub fn tr12(&self) -> TR_R {
        TR_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Rising trigger event configuration of line 13
    #[inline(always)]
    pub fn tr13(&self) -> TR_R {
        TR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Rising trigger event configuration of line 14
    #[inline(always)]
    pub fn tr14(&self) -> TR_R {
        TR_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Rising trigger event configuration of line 15
    #[inline(always)]
    pub fn tr15(&self) -> TR_R {
        TR_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Rising trigger event configuration of line 16
    #[inline(always)]
    pub fn tr16(&self) -> TR_R {
        TR_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Rising trigger event configuration of line 17
    #[inline(always)]
    pub fn tr17(&self) -> TR_R {
        TR_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Rising trigger event configuration of line 18
    #[inline(always)]
    pub fn tr18(&self) -> TR_R {
        TR_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Rising trigger event configuration of line 19
    #[inline(always)]
    pub fn tr19(&self) -> TR_R {
        TR_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Rising trigger event configuration of line 20
    #[inline(always)]
    pub fn tr20(&self) -> TR_R {
        TR_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Rising trigger event configuration of line 21
    #[inline(always)]
    pub fn tr21(&self) -> TR_R {
        TR_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Rising trigger event configuration of line 22
    #[inline(always)]
    pub fn tr22(&self) -> TR_R {
        TR_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Rising trigger event configuration of line 23
    #[inline(always)]
    pub fn tr23(&self) -> TR_R {
        TR_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTSR")
            .field("tr0", &self.tr0())
            .field("tr1", &self.tr1())
            .field("tr2", &self.tr2())
            .field("tr3", &self.tr3())
            .field("tr4", &self.tr4())
            .field("tr5", &self.tr5())
            .field("tr6", &self.tr6())
            .field("tr7", &self.tr7())
            .field("tr8", &self.tr8())
            .field("tr9", &self.tr9())
            .field("tr10", &self.tr10())
            .field("tr11", &self.tr11())
            .field("tr12", &self.tr12())
            .field("tr13", &self.tr13())
            .field("tr14", &self.tr14())
            .field("tr15", &self.tr15())
            .field("tr16", &self.tr16())
            .field("tr17", &self.tr17())
            .field("tr18", &self.tr18())
            .field("tr19", &self.tr19())
            .field("tr20", &self.tr20())
            .field("tr21", &self.tr21())
            .field("tr22", &self.tr22())
            .field("tr23", &self.tr23())
            .finish()
    }
}
impl W {
    ///Rising trigger event configuration of line (0-23)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TR0` field.</div>
    #[inline(always)]
    pub fn tr(&mut self, n: u8) -> TR_W<RTSRrs> {
        #[allow(clippy::no_effect)]
        [(); 24][n as usize];
        TR_W::new(self, n)
    }
    ///Bit 0 - Rising trigger event configuration of line 0
    #[inline(always)]
    pub fn tr0(&mut self) -> TR_W<RTSRrs> {
        TR_W::new(self, 0)
    }
    ///Bit 1 - Rising trigger event configuration of line 1
    #[inline(always)]
    pub fn tr1(&mut self) -> TR_W<RTSRrs> {
        TR_W::new(self, 1)
    }
    ///Bit 2 - Rising trigger event configuration of line 2
    #[inline(always)]
    pub fn tr2(&mut self) -> TR_W<RTSRrs> {
        TR_W::new(self, 2)
    }
    ///Bit 3 - Rising trigger event configuration of line 3
    #[inline(always)]
    pub fn tr3(&mut self) -> TR_W<RTSRrs> {
        TR_W::new(self, 3)
    }
    ///Bit 4 - Rising trigger event configuration of line 4
    #[inline(always)]
    pub fn tr4(&mut self) -> TR_W<RTSRrs> {
        TR_W::new(self, 4)
    }
    ///Bit 5 - Rising trigger event configuration of line 5
    #[inline(always)]
    pub fn tr5(&mut self) -> TR_W<RTSRrs> {
        TR_W::new(self, 5)
    }
    ///Bit 6 - Rising trigger event configuration of line 6
    #[inline(always)]
    pub fn tr6(&mut self) -> TR_W<RTSRrs> {
        TR_W::new(self, 6)
    }
    ///Bit 7 - Rising trigger event configuration of line 7
    #[inline(always)]
    pub fn tr7(&mut self) -> TR_W<RTSRrs> {
        TR_W::new(self, 7)
    }
    ///Bit 8 - Rising trigger event configuration of line 8
    #[inline(always)]
    pub fn tr8(&mut self) -> TR_W<RTSRrs> {
        TR_W::new(self, 8)
    }
    ///Bit 9 - Rising trigger event configuration of line 9
    #[inline(always)]
    pub fn tr9(&mut self) -> TR_W<RTSRrs> {
        TR_W::new(self, 9)
    }
    ///Bit 10 - Rising trigger event configuration of line 10
    #[inline(always)]
    pub fn tr10(&mut self) -> TR_W<RTSRrs> {
        TR_W::new(self, 10)
    }
    ///Bit 11 - Rising trigger event configuration of line 11
    #[inline(always)]
    pub fn tr11(&mut self) -> TR_W<RTSRrs> {
        TR_W::new(self, 11)
    }
    ///Bit 12 - Rising trigger event configuration of line 12
    #[inline(always)]
    pub fn tr12(&mut self) -> TR_W<RTSRrs> {
        TR_W::new(self, 12)
    }
    ///Bit 13 - Rising trigger event configuration of line 13
    #[inline(always)]
    pub fn tr13(&mut self) -> TR_W<RTSRrs> {
        TR_W::new(self, 13)
    }
    ///Bit 14 - Rising trigger event configuration of line 14
    #[inline(always)]
    pub fn tr14(&mut self) -> TR_W<RTSRrs> {
        TR_W::new(self, 14)
    }
    ///Bit 15 - Rising trigger event configuration of line 15
    #[inline(always)]
    pub fn tr15(&mut self) -> TR_W<RTSRrs> {
        TR_W::new(self, 15)
    }
    ///Bit 16 - Rising trigger event configuration of line 16
    #[inline(always)]
    pub fn tr16(&mut self) -> TR_W<RTSRrs> {
        TR_W::new(self, 16)
    }
    ///Bit 17 - Rising trigger event configuration of line 17
    #[inline(always)]
    pub fn tr17(&mut self) -> TR_W<RTSRrs> {
        TR_W::new(self, 17)
    }
    ///Bit 18 - Rising trigger event configuration of line 18
    #[inline(always)]
    pub fn tr18(&mut self) -> TR_W<RTSRrs> {
        TR_W::new(self, 18)
    }
    ///Bit 19 - Rising trigger event configuration of line 19
    #[inline(always)]
    pub fn tr19(&mut self) -> TR_W<RTSRrs> {
        TR_W::new(self, 19)
    }
    ///Bit 20 - Rising trigger event configuration of line 20
    #[inline(always)]
    pub fn tr20(&mut self) -> TR_W<RTSRrs> {
        TR_W::new(self, 20)
    }
    ///Bit 21 - Rising trigger event configuration of line 21
    #[inline(always)]
    pub fn tr21(&mut self) -> TR_W<RTSRrs> {
        TR_W::new(self, 21)
    }
    ///Bit 22 - Rising trigger event configuration of line 22
    #[inline(always)]
    pub fn tr22(&mut self) -> TR_W<RTSRrs> {
        TR_W::new(self, 22)
    }
    ///Bit 23 - Rising trigger event configuration of line 23
    #[inline(always)]
    pub fn tr23(&mut self) -> TR_W<RTSRrs> {
        TR_W::new(self, 23)
    }
}
/**Rising Trigger selection register (EXTI_RTSR)

You can [`read`](crate::Reg::read) this register and get [`rtsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F723.html#EXTI:RTSR)*/
pub struct RTSRrs;
impl crate::RegisterSpec for RTSRrs {
    type Ux = u32;
}
///`read()` method returns [`rtsr::R`](R) reader structure
impl crate::Readable for RTSRrs {}
///`write(|w| ..)` method takes [`rtsr::W`](W) writer structure
impl crate::Writable for RTSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RTSR to value 0
impl crate::Resettable for RTSRrs {
    const RESET_VALUE: u32 = 0;
}