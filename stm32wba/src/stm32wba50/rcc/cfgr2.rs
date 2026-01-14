///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
///Field `HPRE` reader - AHB1, AHB2 and AHB4 prescaler Set and cleared by software to control the division factor of the AHB1, AHB2 and AHB4 clock (hclk1). The software must limit the incremental frequency step by setting these bits correctly to ensure that the hclk1 maximum incremental frequency step does not exceed the maximum allowed incremental frequency step (for more details, refer to Table99: SYSCLK and bus maximum frequency). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xx: hclk1 = SYSCLK not divided
pub type HPRE_R = crate::FieldReader;
///Field `HPRE` writer - AHB1, AHB2 and AHB4 prescaler Set and cleared by software to control the division factor of the AHB1, AHB2 and AHB4 clock (hclk1). The software must limit the incremental frequency step by setting these bits correctly to ensure that the hclk1 maximum incremental frequency step does not exceed the maximum allowed incremental frequency step (for more details, refer to Table99: SYSCLK and bus maximum frequency). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xx: hclk1 = SYSCLK not divided
pub type HPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PPRE1` reader - APB1 prescaler Set and cleared by software to control the division factor of the APB1 clock (pclk1). 0xx: pclk1 = hclk1 not divided
pub type PPRE1_R = crate::FieldReader;
///Field `PPRE1` writer - APB1 prescaler Set and cleared by software to control the division factor of the APB1 clock (pclk1). 0xx: pclk1 = hclk1 not divided
pub type PPRE1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PPRE2` reader - APB2 prescaler Set and cleared by software to control the division factor of the APB2 clock (pclk2). 0xx: pclk2 = hclk1 not divided
pub type PPRE2_R = crate::FieldReader;
///Field `PPRE2` writer - APB2 prescaler Set and cleared by software to control the division factor of the APB2 clock (pclk2). 0xx: pclk2 = hclk1 not divided
pub type PPRE2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - AHB1, AHB2 and AHB4 prescaler Set and cleared by software to control the division factor of the AHB1, AHB2 and AHB4 clock (hclk1). The software must limit the incremental frequency step by setting these bits correctly to ensure that the hclk1 maximum incremental frequency step does not exceed the maximum allowed incremental frequency step (for more details, refer to Table99: SYSCLK and bus maximum frequency). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xx: hclk1 = SYSCLK not divided
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - APB1 prescaler Set and cleared by software to control the division factor of the APB1 clock (pclk1). 0xx: pclk1 = hclk1 not divided
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - APB2 prescaler Set and cleared by software to control the division factor of the APB2 clock (pclk2). 0xx: pclk2 = hclk1 not divided
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("hpre", &self.hpre())
            .field("ppre1", &self.ppre1())
            .field("ppre2", &self.ppre2())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - AHB1, AHB2 and AHB4 prescaler Set and cleared by software to control the division factor of the AHB1, AHB2 and AHB4 clock (hclk1). The software must limit the incremental frequency step by setting these bits correctly to ensure that the hclk1 maximum incremental frequency step does not exceed the maximum allowed incremental frequency step (for more details, refer to Table99: SYSCLK and bus maximum frequency). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xx: hclk1 = SYSCLK not divided
    #[inline(always)]
    pub fn hpre(&mut self) -> HPRE_W<'_, CFGR2rs> {
        HPRE_W::new(self, 0)
    }
    ///Bits 4:6 - APB1 prescaler Set and cleared by software to control the division factor of the APB1 clock (pclk1). 0xx: pclk1 = hclk1 not divided
    #[inline(always)]
    pub fn ppre1(&mut self) -> PPRE1_W<'_, CFGR2rs> {
        PPRE1_W::new(self, 4)
    }
    ///Bits 8:10 - APB2 prescaler Set and cleared by software to control the division factor of the APB2 clock (pclk2). 0xx: pclk2 = hclk1 not divided
    #[inline(always)]
    pub fn ppre2(&mut self) -> PPRE2_W<'_, CFGR2rs> {
        PPRE2_W::new(self, 8)
    }
}
/**RCC clock configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#RCC:CFGR2)*/
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr2::R`](R) reader structure
impl crate::Readable for CFGR2rs {}
///`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2rs {}
