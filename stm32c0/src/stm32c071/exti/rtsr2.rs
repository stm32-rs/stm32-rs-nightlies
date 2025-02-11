///Register `RTSR2` reader
pub type R = crate::R<RTSR2rs>;
///Register `RTSR2` writer
pub type W = crate::W<RTSR2rs>;
///Field `RT34` reader - Rising trigger event configuration bit of configurable line 34 Each bit enables/disables the rising edge trigger for the event and interrupt on the line 34. This configurable line is edge triggered; no glitch must be generated on this inputs. Note: If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT34_R = crate::BitReader;
///Field `RT34` writer - Rising trigger event configuration bit of configurable line 34 Each bit enables/disables the rising edge trigger for the event and interrupt on the line 34. This configurable line is edge triggered; no glitch must be generated on this inputs. Note: If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
pub type RT34_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - Rising trigger event configuration bit of configurable line 34 Each bit enables/disables the rising edge trigger for the event and interrupt on the line 34. This configurable line is edge triggered; no glitch must be generated on this inputs. Note: If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt34(&self) -> RT34_R {
        RT34_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTSR2").field("rt34", &self.rt34()).finish()
    }
}
impl W {
    ///Bit 2 - Rising trigger event configuration bit of configurable line 34 Each bit enables/disables the rising edge trigger for the event and interrupt on the line 34. This configurable line is edge triggered; no glitch must be generated on this inputs. Note: If a rising edge on the configurable line occurs during writing of the register, the associated pending bit is not set. Rising edge trigger can be set for a line with falling edge trigger enabled. In this case, both edges generate a trigger.
    #[inline(always)]
    pub fn rt34(&mut self) -> RT34_W<RTSR2rs> {
        RT34_W::new(self, 2)
    }
}
/**EXTI rising trigger selection register 2

You can [`read`](crate::Reg::read) this register and get [`rtsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#EXTI:RTSR2)*/
pub struct RTSR2rs;
impl crate::RegisterSpec for RTSR2rs {
    type Ux = u32;
}
///`read()` method returns [`rtsr2::R`](R) reader structure
impl crate::Readable for RTSR2rs {}
///`write(|w| ..)` method takes [`rtsr2::W`](W) writer structure
impl crate::Writable for RTSR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RTSR2 to value 0
impl crate::Resettable for RTSR2rs {
    const RESET_VALUE: u32 = 0;
}
