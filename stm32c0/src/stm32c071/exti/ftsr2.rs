///Register `FTSR2` reader
pub type R = crate::R<FTSR2rs>;
///Register `FTSR2` writer
pub type W = crate::W<FTSR2rs>;
///Field `FT34` reader - Falling trigger event configuration bit of configurable line 34. Each bit enables/disables the falling edge trigger for the event and interrupt on the line 34. The configurable lines are edge triggered; no glitch must be generated on these inputs. Note: If a falling edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Falling edge trigger can be set for a line with rising edge trigger enabled. In this case, both edges generate a trigger.
pub type FT34_R = crate::BitReader;
///Field `FT34` writer - Falling trigger event configuration bit of configurable line 34. Each bit enables/disables the falling edge trigger for the event and interrupt on the line 34. The configurable lines are edge triggered; no glitch must be generated on these inputs. Note: If a falling edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Falling edge trigger can be set for a line with rising edge trigger enabled. In this case, both edges generate a trigger.
pub type FT34_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - Falling trigger event configuration bit of configurable line 34. Each bit enables/disables the falling edge trigger for the event and interrupt on the line 34. The configurable lines are edge triggered; no glitch must be generated on these inputs. Note: If a falling edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Falling edge trigger can be set for a line with rising edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn ft34(&self) -> FT34_R {
        FT34_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FTSR2").field("ft34", &self.ft34()).finish()
    }
}
impl W {
    ///Bit 2 - Falling trigger event configuration bit of configurable line 34. Each bit enables/disables the falling edge trigger for the event and interrupt on the line 34. The configurable lines are edge triggered; no glitch must be generated on these inputs. Note: If a falling edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Falling edge trigger can be set for a line with rising edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn ft34(&mut self) -> FT34_W<FTSR2rs> {
        FT34_W::new(self, 2)
    }
}
/**EXTI falling trigger selection register 2

You can [`read`](crate::Reg::read) this register and get [`ftsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#EXTI:FTSR2)*/
pub struct FTSR2rs;
impl crate::RegisterSpec for FTSR2rs {
    type Ux = u32;
}
///`read()` method returns [`ftsr2::R`](R) reader structure
impl crate::Readable for FTSR2rs {}
///`write(|w| ..)` method takes [`ftsr2::W`](W) writer structure
impl crate::Writable for FTSR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FTSR2 to value 0
impl crate::Resettable for FTSR2rs {
    const RESET_VALUE: u32 = 0;
}
