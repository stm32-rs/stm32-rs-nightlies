///Register `CFGR3` reader
pub type R = crate::R<CFGR3rs>;
///Register `CFGR3` writer
pub type W = crate::W<CFGR3rs>;
///Field `PINMUX0` reader - Pin GPIO multiplexer 0 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved Pin F2 of WLCSP14 package GPIO assignment 1x: Reserved
pub type PINMUX0_R = crate::FieldReader;
///Field `PINMUX0` writer - Pin GPIO multiplexer 0 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved Pin F2 of WLCSP14 package GPIO assignment 1x: Reserved
pub type PINMUX0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PINMUX1` reader - Pin GPIO multiplexer 1 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved
pub type PINMUX1_R = crate::FieldReader;
///Field `PINMUX1` writer - Pin GPIO multiplexer 1 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved
pub type PINMUX1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PINMUX2` reader - Pin GPIO multiplexer 2 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved 1x: Reserved
pub type PINMUX2_R = crate::FieldReader;
///Field `PINMUX2` writer - Pin GPIO multiplexer 2 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved 1x: Reserved
pub type PINMUX2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PINMUX3` reader - Pin GPIO multiplexer 3 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved
pub type PINMUX3_R = crate::FieldReader;
///Field `PINMUX3` writer - Pin GPIO multiplexer 3 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved
pub type PINMUX3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PINMUX4` reader - Pin GPIO multiplexer 4 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved 1x: Reserved
pub type PINMUX4_R = crate::FieldReader;
///Field `PINMUX4` writer - Pin GPIO multiplexer 4 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved 1x: Reserved
pub type PINMUX4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PINMUX5` reader - Pin GPIO multiplexer 5 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved
pub type PINMUX5_R = crate::FieldReader;
///Field `PINMUX5` writer - Pin GPIO multiplexer 5 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved
pub type PINMUX5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Pin GPIO multiplexer 0 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved Pin F2 of WLCSP14 package GPIO assignment 1x: Reserved
    #[inline(always)]
    pub fn pinmux0(&self) -> PINMUX0_R {
        PINMUX0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Pin GPIO multiplexer 1 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved
    #[inline(always)]
    pub fn pinmux1(&self) -> PINMUX1_R {
        PINMUX1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Pin GPIO multiplexer 2 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved 1x: Reserved
    #[inline(always)]
    pub fn pinmux2(&self) -> PINMUX2_R {
        PINMUX2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Pin GPIO multiplexer 3 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved
    #[inline(always)]
    pub fn pinmux3(&self) -> PINMUX3_R {
        PINMUX3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Pin GPIO multiplexer 4 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved 1x: Reserved
    #[inline(always)]
    pub fn pinmux4(&self) -> PINMUX4_R {
        PINMUX4_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Pin GPIO multiplexer 5 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved
    #[inline(always)]
    pub fn pinmux5(&self) -> PINMUX5_R {
        PINMUX5_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR3")
            .field("pinmux0", &self.pinmux0())
            .field("pinmux1", &self.pinmux1())
            .field("pinmux2", &self.pinmux2())
            .field("pinmux3", &self.pinmux3())
            .field("pinmux4", &self.pinmux4())
            .field("pinmux5", &self.pinmux5())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Pin GPIO multiplexer 0 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved Pin F2 of WLCSP14 package GPIO assignment 1x: Reserved
    #[inline(always)]
    pub fn pinmux0(&mut self) -> PINMUX0_W<CFGR3rs> {
        PINMUX0_W::new(self, 0)
    }
    ///Bits 2:3 - Pin GPIO multiplexer 1 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved
    #[inline(always)]
    pub fn pinmux1(&mut self) -> PINMUX1_W<CFGR3rs> {
        PINMUX1_W::new(self, 2)
    }
    ///Bits 4:5 - Pin GPIO multiplexer 2 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved 1x: Reserved
    #[inline(always)]
    pub fn pinmux2(&mut self) -> PINMUX2_W<CFGR3rs> {
        PINMUX2_W::new(self, 4)
    }
    ///Bits 6:7 - Pin GPIO multiplexer 3 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved
    #[inline(always)]
    pub fn pinmux3(&mut self) -> PINMUX3_W<CFGR3rs> {
        PINMUX3_W::new(self, 6)
    }
    ///Bits 8:9 - Pin GPIO multiplexer 4 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved 1x: Reserved
    #[inline(always)]
    pub fn pinmux4(&mut self) -> PINMUX4_W<CFGR3rs> {
        PINMUX4_W::new(self, 8)
    }
    ///Bits 10:11 - Pin GPIO multiplexer 5 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved
    #[inline(always)]
    pub fn pinmux5(&mut self) -> PINMUX5_W<CFGR3rs> {
        PINMUX5_W::new(self, 10)
    }
}
/**SYSCFG configuration register 3

You can [`read`](crate::Reg::read) this register and get [`cfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#SYSCFG:CFGR3)*/
pub struct CFGR3rs;
impl crate::RegisterSpec for CFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr3::R`](R) reader structure
impl crate::Readable for CFGR3rs {}
///`write(|w| ..)` method takes [`cfgr3::W`](W) writer structure
impl crate::Writable for CFGR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CFGR3 to value 0
impl crate::Resettable for CFGR3rs {
    const RESET_VALUE: u32 = 0;
}
