///Register `UR2` reader
pub type R = crate::R<UR2rs>;
///Register `UR2` writer
pub type W = crate::W<UR2rs>;
///Field `BORH` reader - BOR_LVL Brownout Reset Threshold Level
pub type BORH_R = crate::FieldReader;
///Field `BCM7_ADD0` reader - Cortex-M7 Boot Address 0
pub type BCM7_ADD0_R = crate::FieldReader<u16>;
///Field `BCM7_ADD0` writer - Cortex-M7 Boot Address 0
pub type BCM7_ADD0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:1 - BOR_LVL Brownout Reset Threshold Level
    #[inline(always)]
    pub fn borh(&self) -> BORH_R {
        BORH_R::new((self.bits & 3) as u8)
    }
    ///Bits 16:31 - Cortex-M7 Boot Address 0
    #[inline(always)]
    pub fn bcm7_add0(&self) -> BCM7_ADD0_R {
        BCM7_ADD0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UR2")
            .field("borh", &self.borh())
            .field("bcm7_add0", &self.bcm7_add0())
            .finish()
    }
}
impl W {
    ///Bits 16:31 - Cortex-M7 Boot Address 0
    #[inline(always)]
    pub fn bcm7_add0(&mut self) -> BCM7_ADD0_W<'_, UR2rs> {
        BCM7_ADD0_W::new(self, 16)
    }
}
/**SYSCFG user register 2

You can [`read`](crate::Reg::read) this register and get [`ur2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM7.html#SYSCFG:UR2)*/
pub struct UR2rs;
impl crate::RegisterSpec for UR2rs {
    type Ux = u32;
}
///`read()` method returns [`ur2::R`](R) reader structure
impl crate::Readable for UR2rs {}
///`write(|w| ..)` method takes [`ur2::W`](W) writer structure
impl crate::Writable for UR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets UR2 to value 0
impl crate::Resettable for UR2rs {}
