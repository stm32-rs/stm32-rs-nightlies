///Register `UR4` reader
pub type R = crate::R<UR4rs>;
///Register `UR4` writer
pub type W = crate::W<UR4rs>;
///Field `BCM4_ADD1` reader - Mass Erase Protected Area Disabled for bank 1
pub type BCM4_ADD1_R = crate::FieldReader<u16>;
///Field `BCM4_ADD1` writer - Mass Erase Protected Area Disabled for bank 1
pub type BCM4_ADD1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `MEPAD_1` reader - Boot Cortex-M4 Address 1
pub type MEPAD_1_R = crate::BitReader;
impl R {
    ///Bits 0:15 - Mass Erase Protected Area Disabled for bank 1
    #[inline(always)]
    pub fn bcm4_add1(&self) -> BCM4_ADD1_R {
        BCM4_ADD1_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - Boot Cortex-M4 Address 1
    #[inline(always)]
    pub fn mepad_1(&self) -> MEPAD_1_R {
        MEPAD_1_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UR4")
            .field("bcm4_add1", &self.bcm4_add1())
            .field("mepad_1", &self.mepad_1())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Mass Erase Protected Area Disabled for bank 1
    #[inline(always)]
    pub fn bcm4_add1(&mut self) -> BCM4_ADD1_W<UR4rs> {
        BCM4_ADD1_W::new(self, 0)
    }
}
/**SYSCFG user register 4

You can [`read`](crate::Reg::read) this register and get [`ur4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#SYSCFG:UR4)*/
pub struct UR4rs;
impl crate::RegisterSpec for UR4rs {
    type Ux = u32;
}
///`read()` method returns [`ur4::R`](R) reader structure
impl crate::Readable for UR4rs {}
///`write(|w| ..)` method takes [`ur4::W`](W) writer structure
impl crate::Writable for UR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets UR4 to value 0
impl crate::Resettable for UR4rs {}
