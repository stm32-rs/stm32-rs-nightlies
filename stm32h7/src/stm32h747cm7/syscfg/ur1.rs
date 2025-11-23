///Register `UR1` reader
pub type R = crate::R<UR1rs>;
///Register `UR1` writer
pub type W = crate::W<UR1rs>;
///Field `BCM4` reader - Boot Cortex-M4
pub type BCM4_R = crate::BitReader;
///Field `BCM4` writer - Boot Cortex-M4
pub type BCM4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BCM7` reader - Boot Cortex-M7
pub type BCM7_R = crate::BitReader;
///Field `BCM7` writer - Boot Cortex-M7
pub type BCM7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Boot Cortex-M4
    #[inline(always)]
    pub fn bcm4(&self) -> BCM4_R {
        BCM4_R::new((self.bits & 1) != 0)
    }
    ///Bit 16 - Boot Cortex-M7
    #[inline(always)]
    pub fn bcm7(&self) -> BCM7_R {
        BCM7_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UR1")
            .field("bcm4", &self.bcm4())
            .field("bcm7", &self.bcm7())
            .finish()
    }
}
impl W {
    ///Bit 0 - Boot Cortex-M4
    #[inline(always)]
    pub fn bcm4(&mut self) -> BCM4_W<'_, UR1rs> {
        BCM4_W::new(self, 0)
    }
    ///Bit 16 - Boot Cortex-M7
    #[inline(always)]
    pub fn bcm7(&mut self) -> BCM7_W<'_, UR1rs> {
        BCM7_W::new(self, 16)
    }
}
/**SYSCFG user register 1

You can [`read`](crate::Reg::read) this register and get [`ur1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#SYSCFG:UR1)*/
pub struct UR1rs;
impl crate::RegisterSpec for UR1rs {
    type Ux = u32;
}
///`read()` method returns [`ur1::R`](R) reader structure
impl crate::Readable for UR1rs {}
///`write(|w| ..)` method takes [`ur1::W`](W) writer structure
impl crate::Writable for UR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets UR1 to value 0
impl crate::Resettable for UR1rs {}
