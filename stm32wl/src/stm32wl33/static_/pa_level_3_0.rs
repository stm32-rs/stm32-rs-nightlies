///Register `PA_LEVEL_3_0` reader
pub type R = crate::R<PA_LEVEL_3_0rs>;
///Register `PA_LEVEL_3_0` writer
pub type W = crate::W<PA_LEVEL_3_0rs>;
///Field `PA_LEVEL0` reader - Output power level for first step
pub type PA_LEVEL0_R = crate::FieldReader;
///Field `PA_LEVEL0` writer - Output power level for first step
pub type PA_LEVEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PA_LEVEL1` reader - Output power level for second step
pub type PA_LEVEL1_R = crate::FieldReader;
///Field `PA_LEVEL1` writer - Output power level for second step
pub type PA_LEVEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PA_LEVEL2` reader - Output power level for third step
pub type PA_LEVEL2_R = crate::FieldReader;
///Field `PA_LEVEL2` writer - Output power level for third step
pub type PA_LEVEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PA_LEVEL3` reader - Output power level for fourth step
pub type PA_LEVEL3_R = crate::FieldReader;
///Field `PA_LEVEL3` writer - Output power level for fourth step
pub type PA_LEVEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Output power level for first step
    #[inline(always)]
    pub fn pa_level0(&self) -> PA_LEVEL0_R {
        PA_LEVEL0_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Output power level for second step
    #[inline(always)]
    pub fn pa_level1(&self) -> PA_LEVEL1_R {
        PA_LEVEL1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Output power level for third step
    #[inline(always)]
    pub fn pa_level2(&self) -> PA_LEVEL2_R {
        PA_LEVEL2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Output power level for fourth step
    #[inline(always)]
    pub fn pa_level3(&self) -> PA_LEVEL3_R {
        PA_LEVEL3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PA_LEVEL_3_0")
            .field("pa_level0", &self.pa_level0())
            .field("pa_level1", &self.pa_level1())
            .field("pa_level2", &self.pa_level2())
            .field("pa_level3", &self.pa_level3())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Output power level for first step
    #[inline(always)]
    pub fn pa_level0(&mut self) -> PA_LEVEL0_W<'_, PA_LEVEL_3_0rs> {
        PA_LEVEL0_W::new(self, 0)
    }
    ///Bits 8:15 - Output power level for second step
    #[inline(always)]
    pub fn pa_level1(&mut self) -> PA_LEVEL1_W<'_, PA_LEVEL_3_0rs> {
        PA_LEVEL1_W::new(self, 8)
    }
    ///Bits 16:23 - Output power level for third step
    #[inline(always)]
    pub fn pa_level2(&mut self) -> PA_LEVEL2_W<'_, PA_LEVEL_3_0rs> {
        PA_LEVEL2_W::new(self, 16)
    }
    ///Bits 24:31 - Output power level for fourth step
    #[inline(always)]
    pub fn pa_level3(&mut self) -> PA_LEVEL3_W<'_, PA_LEVEL_3_0rs> {
        PA_LEVEL3_W::new(self, 24)
    }
}
/**PA_LEVEL_3_0 register

You can [`read`](crate::Reg::read) this register and get [`pa_level_3_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_level_3_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:PA_LEVEL_3_0)*/
pub struct PA_LEVEL_3_0rs;
impl crate::RegisterSpec for PA_LEVEL_3_0rs {
    type Ux = u32;
}
///`read()` method returns [`pa_level_3_0::R`](R) reader structure
impl crate::Readable for PA_LEVEL_3_0rs {}
///`write(|w| ..)` method takes [`pa_level_3_0::W`](W) writer structure
impl crate::Writable for PA_LEVEL_3_0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PA_LEVEL_3_0 to value 0x230b_0100
impl crate::Resettable for PA_LEVEL_3_0rs {
    const RESET_VALUE: u32 = 0x230b_0100;
}
