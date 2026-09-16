///Register `CFGR3` reader
pub type R = crate::R<CFGR3rs>;
///Register `CFGR3` writer
pub type W = crate::W<CFGR3rs>;
///Field `PPRE7` reader - APB7 prescaler Set and cleared by software to control the division factor of the APB7 clock (pclk7). 0xx: hclk1 not divided
pub type PPRE7_R = crate::FieldReader;
///Field `PPRE7` writer - APB7 prescaler Set and cleared by software to control the division factor of the APB7 clock (pclk7). 0xx: hclk1 not divided
pub type PPRE7_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 4:6 - APB7 prescaler Set and cleared by software to control the division factor of the APB7 clock (pclk7). 0xx: hclk1 not divided
    #[inline(always)]
    pub fn ppre7(&self) -> PPRE7_R {
        PPRE7_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR3")
            .field("ppre7", &self.ppre7())
            .finish()
    }
}
impl W {
    ///Bits 4:6 - APB7 prescaler Set and cleared by software to control the division factor of the APB7 clock (pclk7). 0xx: hclk1 not divided
    #[inline(always)]
    pub fn ppre7(&mut self) -> PPRE7_W<'_, CFGR3rs> {
        PPRE7_W::new(self, 4)
    }
}
/**RCC clock configuration register 3

You can [`read`](crate::Reg::read) this register and get [`cfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#RCC:CFGR3)*/
pub struct CFGR3rs;
impl crate::RegisterSpec for CFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr3::R`](R) reader structure
impl crate::Readable for CFGR3rs {}
///`write(|w| ..)` method takes [`cfgr3::W`](W) writer structure
impl crate::Writable for CFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR3 to value 0
impl crate::Resettable for CFGR3rs {}
