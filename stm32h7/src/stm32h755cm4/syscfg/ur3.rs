///Register `UR3` reader
pub type R = crate::R<UR3rs>;
///Register `UR3` writer
pub type W = crate::W<UR3rs>;
///Field `BCM4_ADD1` reader - Cortex-M4 Boot Address 0
pub type BCM4_ADD1_R = crate::FieldReader<u16>;
///Field `BCM4_ADD1` writer - Cortex-M4 Boot Address 0
pub type BCM4_ADD1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `BCM7_ADD1` reader - Cortex-M7 Boot Address 1
pub type BCM7_ADD1_R = crate::FieldReader<u16>;
///Field `BCM7_ADD1` writer - Cortex-M7 Boot Address 1
pub type BCM7_ADD1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Cortex-M4 Boot Address 0
    #[inline(always)]
    pub fn bcm4_add1(&self) -> BCM4_ADD1_R {
        BCM4_ADD1_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Cortex-M7 Boot Address 1
    #[inline(always)]
    pub fn bcm7_add1(&self) -> BCM7_ADD1_R {
        BCM7_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UR3")
            .field("bcm4_add1", &self.bcm4_add1())
            .field("bcm7_add1", &self.bcm7_add1())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Cortex-M4 Boot Address 0
    #[inline(always)]
    pub fn bcm4_add1(&mut self) -> BCM4_ADD1_W<'_, UR3rs> {
        BCM4_ADD1_W::new(self, 0)
    }
    ///Bits 16:31 - Cortex-M7 Boot Address 1
    #[inline(always)]
    pub fn bcm7_add1(&mut self) -> BCM7_ADD1_W<'_, UR3rs> {
        BCM7_ADD1_W::new(self, 16)
    }
}
/**SYSCFG user register 3

You can [`read`](crate::Reg::read) this register and get [`ur3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#SYSCFG:UR3)*/
pub struct UR3rs;
impl crate::RegisterSpec for UR3rs {
    type Ux = u32;
}
///`read()` method returns [`ur3::R`](R) reader structure
impl crate::Readable for UR3rs {}
///`write(|w| ..)` method takes [`ur3::W`](W) writer structure
impl crate::Writable for UR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets UR3 to value 0
impl crate::Resettable for UR3rs {}
