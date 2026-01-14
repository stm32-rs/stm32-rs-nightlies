///Register `CFGLOCK` reader
pub type R = crate::R<CFGLOCKrs>;
///Register `CFGLOCK` writer
pub type W = crate::W<CFGLOCKrs>;
///Field `SPLCK(0-3)` reader - Security/privilege configuration lock super-block; This bit is set by software and can be cleared only by system reset.; note that bit \[3:2\] are reserved on sales type STM32WBA5xEx for MPCBB1.
pub type SPLCK_R = crate::BitReader;
///Field `SPLCK(0-3)` writer - Security/privilege configuration lock super-block; This bit is set by software and can be cleared only by system reset.; note that bit \[3:2\] are reserved on sales type STM32WBA5xEx for MPCBB1.
pub type SPLCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Security/privilege configuration lock super-block; This bit is set by software and can be cleared only by system reset.; note that bit \[3:2\] are reserved on sales type STM32WBA5xEx for MPCBB1.
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SPLCK0` field.</div>
    #[inline(always)]
    pub fn splck(&self, n: u8) -> SPLCK_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        SPLCK_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Security/privilege configuration lock super-block; This bit is set by software and can be cleared only by system reset.; note that bit \[3:2\] are reserved on sales type STM32WBA5xEx for MPCBB1.
    #[inline(always)]
    pub fn splck_iter(&self) -> impl Iterator<Item = SPLCK_R> + '_ {
        (0..4).map(move |n| SPLCK_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Security/privilege configuration lock super-block; This bit is set by software and can be cleared only by system reset.; note that bit \[3:2\] are reserved on sales type STM32WBA5xEx for MPCBB1.
    #[inline(always)]
    pub fn splck0(&self) -> SPLCK_R {
        SPLCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Security/privilege configuration lock super-block; This bit is set by software and can be cleared only by system reset.; note that bit \[3:2\] are reserved on sales type STM32WBA5xEx for MPCBB1.
    #[inline(always)]
    pub fn splck1(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Security/privilege configuration lock super-block; This bit is set by software and can be cleared only by system reset.; note that bit \[3:2\] are reserved on sales type STM32WBA5xEx for MPCBB1.
    #[inline(always)]
    pub fn splck2(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Security/privilege configuration lock super-block; This bit is set by software and can be cleared only by system reset.; note that bit \[3:2\] are reserved on sales type STM32WBA5xEx for MPCBB1.
    #[inline(always)]
    pub fn splck3(&self) -> SPLCK_R {
        SPLCK_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGLOCK")
            .field("splck0", &self.splck0())
            .field("splck1", &self.splck1())
            .field("splck2", &self.splck2())
            .field("splck3", &self.splck3())
            .finish()
    }
}
impl W {
    ///Security/privilege configuration lock super-block; This bit is set by software and can be cleared only by system reset.; note that bit \[3:2\] are reserved on sales type STM32WBA5xEx for MPCBB1.
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SPLCK0` field.</div>
    #[inline(always)]
    pub fn splck(&mut self, n: u8) -> SPLCK_W<'_, CFGLOCKrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        SPLCK_W::new(self, n)
    }
    ///Bit 0 - Security/privilege configuration lock super-block; This bit is set by software and can be cleared only by system reset.; note that bit \[3:2\] are reserved on sales type STM32WBA5xEx for MPCBB1.
    #[inline(always)]
    pub fn splck0(&mut self) -> SPLCK_W<'_, CFGLOCKrs> {
        SPLCK_W::new(self, 0)
    }
    ///Bit 1 - Security/privilege configuration lock super-block; This bit is set by software and can be cleared only by system reset.; note that bit \[3:2\] are reserved on sales type STM32WBA5xEx for MPCBB1.
    #[inline(always)]
    pub fn splck1(&mut self) -> SPLCK_W<'_, CFGLOCKrs> {
        SPLCK_W::new(self, 1)
    }
    ///Bit 2 - Security/privilege configuration lock super-block; This bit is set by software and can be cleared only by system reset.; note that bit \[3:2\] are reserved on sales type STM32WBA5xEx for MPCBB1.
    #[inline(always)]
    pub fn splck2(&mut self) -> SPLCK_W<'_, CFGLOCKrs> {
        SPLCK_W::new(self, 2)
    }
    ///Bit 3 - Security/privilege configuration lock super-block; This bit is set by software and can be cleared only by system reset.; note that bit \[3:2\] are reserved on sales type STM32WBA5xEx for MPCBB1.
    #[inline(always)]
    pub fn splck3(&mut self) -> SPLCK_W<'_, CFGLOCKrs> {
        SPLCK_W::new(self, 3)
    }
}
/**GTZC1 SRAMz MPCBB configuration lock register

You can [`read`](crate::Reg::read) this register and get [`cfglock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfglock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#GTZC1_MPCBB1:CFGLOCK)*/
pub struct CFGLOCKrs;
impl crate::RegisterSpec for CFGLOCKrs {
    type Ux = u32;
}
///`read()` method returns [`cfglock::R`](R) reader structure
impl crate::Readable for CFGLOCKrs {}
///`write(|w| ..)` method takes [`cfglock::W`](W) writer structure
impl crate::Writable for CFGLOCKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGLOCK to value 0
impl crate::Resettable for CFGLOCKrs {}
