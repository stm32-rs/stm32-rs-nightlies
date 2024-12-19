///Register `MPCBB4_CFGLOCK` reader
pub type R = crate::R<MPCBB4_CFGLOCKrs>;
///Register `MPCBB4_CFGLOCK` writer
pub type W = crate::W<MPCBB4_CFGLOCKrs>;
///Field `SPLCK0` reader - Security/privilege configuration lock for super-block 0
pub type SPLCK0_R = crate::BitReader;
///Field `SPLCK0` writer - Security/privilege configuration lock for super-block 0
pub type SPLCK0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Security/privilege configuration lock for super-block 0
    #[inline(always)]
    pub fn splck0(&self) -> SPLCK0_R {
        SPLCK0_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MPCBB4_CFGLOCK")
            .field("splck0", &self.splck0())
            .finish()
    }
}
impl W {
    ///Bit 0 - Security/privilege configuration lock for super-block 0
    #[inline(always)]
    pub fn splck0(&mut self) -> SPLCK0_W<MPCBB4_CFGLOCKrs> {
        SPLCK0_W::new(self, 0)
    }
}
/**GTZC2 SRAM4 MPCBB configuration lock register

You can [`read`](crate::Reg::read) this register and get [`mpcbb4_cfglock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb4_cfglock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#GTZC2_MPCBB4:MPCBB4_CFGLOCK)*/
pub struct MPCBB4_CFGLOCKrs;
impl crate::RegisterSpec for MPCBB4_CFGLOCKrs {
    type Ux = u32;
}
///`read()` method returns [`mpcbb4_cfglock::R`](R) reader structure
impl crate::Readable for MPCBB4_CFGLOCKrs {}
///`write(|w| ..)` method takes [`mpcbb4_cfglock::W`](W) writer structure
impl crate::Writable for MPCBB4_CFGLOCKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MPCBB4_CFGLOCK to value 0
impl crate::Resettable for MPCBB4_CFGLOCKrs {
    const RESET_VALUE: u32 = 0;
}
