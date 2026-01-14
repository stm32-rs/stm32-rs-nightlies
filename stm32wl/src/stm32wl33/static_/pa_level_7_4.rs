///Register `PA_LEVEL_7_4` reader
pub type R = crate::R<PA_LEVEL_7_4rs>;
///Register `PA_LEVEL_7_4` writer
pub type W = crate::W<PA_LEVEL_7_4rs>;
///Field `PA_LEVEL4` reader - Output power level for fifth step
pub type PA_LEVEL4_R = crate::FieldReader;
///Field `PA_LEVEL4` writer - Output power level for fifth step
pub type PA_LEVEL4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PA_LEVEL5` reader - Output power level for sixth step
pub type PA_LEVEL5_R = crate::FieldReader;
///Field `PA_LEVEL5` writer - Output power level for sixth step
pub type PA_LEVEL5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PA_LEVEL6` reader - Output power level for seventh step
pub type PA_LEVEL6_R = crate::FieldReader;
///Field `PA_LEVEL6` writer - Output power level for seventh step
pub type PA_LEVEL6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PA_LEVEL7` reader - Output power level for eighth step
pub type PA_LEVEL7_R = crate::FieldReader;
///Field `PA_LEVEL7` writer - Output power level for eighth step
pub type PA_LEVEL7_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Output power level for fifth step
    #[inline(always)]
    pub fn pa_level4(&self) -> PA_LEVEL4_R {
        PA_LEVEL4_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Output power level for sixth step
    #[inline(always)]
    pub fn pa_level5(&self) -> PA_LEVEL5_R {
        PA_LEVEL5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Output power level for seventh step
    #[inline(always)]
    pub fn pa_level6(&self) -> PA_LEVEL6_R {
        PA_LEVEL6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Output power level for eighth step
    #[inline(always)]
    pub fn pa_level7(&self) -> PA_LEVEL7_R {
        PA_LEVEL7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PA_LEVEL_7_4")
            .field("pa_level4", &self.pa_level4())
            .field("pa_level5", &self.pa_level5())
            .field("pa_level6", &self.pa_level6())
            .field("pa_level7", &self.pa_level7())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Output power level for fifth step
    #[inline(always)]
    pub fn pa_level4(&mut self) -> PA_LEVEL4_W<'_, PA_LEVEL_7_4rs> {
        PA_LEVEL4_W::new(self, 0)
    }
    ///Bits 8:15 - Output power level for sixth step
    #[inline(always)]
    pub fn pa_level5(&mut self) -> PA_LEVEL5_W<'_, PA_LEVEL_7_4rs> {
        PA_LEVEL5_W::new(self, 8)
    }
    ///Bits 16:23 - Output power level for seventh step
    #[inline(always)]
    pub fn pa_level6(&mut self) -> PA_LEVEL6_W<'_, PA_LEVEL_7_4rs> {
        PA_LEVEL6_W::new(self, 16)
    }
    ///Bits 24:31 - Output power level for eighth step
    #[inline(always)]
    pub fn pa_level7(&mut self) -> PA_LEVEL7_W<'_, PA_LEVEL_7_4rs> {
        PA_LEVEL7_W::new(self, 24)
    }
}
/**PA_LEVEL_7_4 register

You can [`read`](crate::Reg::read) this register and get [`pa_level_7_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_level_7_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:PA_LEVEL_7_4)*/
pub struct PA_LEVEL_7_4rs;
impl crate::RegisterSpec for PA_LEVEL_7_4rs {
    type Ux = u32;
}
///`read()` method returns [`pa_level_7_4::R`](R) reader structure
impl crate::Readable for PA_LEVEL_7_4rs {}
///`write(|w| ..)` method takes [`pa_level_7_4::W`](W) writer structure
impl crate::Writable for PA_LEVEL_7_4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PA_LEVEL_7_4 to value 0x5147_3b2f
impl crate::Resettable for PA_LEVEL_7_4rs {
    const RESET_VALUE: u32 = 0x5147_3b2f;
}
