///Register `LDO` reader
pub type R = crate::R<LDOrs>;
///Register `LDO` writer
pub type W = crate::W<LDOrs>;
///Field `LDO_USED` reader - Indicates the presence of the LDO in the chip
pub type LDO_USED_R = crate::BitReader;
///Field `LDO_STATUS` reader - Monitors the status of the PHY's LDO
pub type LDO_STATUS_R = crate::BitReader;
///Field `LDO_DISABLE` reader - Controls disable of the High Speed PHY's LDO
pub type LDO_DISABLE_R = crate::BitReader;
///Field `LDO_DISABLE` writer - Controls disable of the High Speed PHY's LDO
pub type LDO_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Indicates the presence of the LDO in the chip
    #[inline(always)]
    pub fn ldo_used(&self) -> LDO_USED_R {
        LDO_USED_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Monitors the status of the PHY's LDO
    #[inline(always)]
    pub fn ldo_status(&self) -> LDO_STATUS_R {
        LDO_STATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Controls disable of the High Speed PHY's LDO
    #[inline(always)]
    pub fn ldo_disable(&self) -> LDO_DISABLE_R {
        LDO_DISABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LDO")
            .field("ldo_used", &self.ldo_used())
            .field("ldo_status", &self.ldo_status())
            .field("ldo_disable", &self.ldo_disable())
            .finish()
    }
}
impl W {
    ///Bit 2 - Controls disable of the High Speed PHY's LDO
    #[inline(always)]
    pub fn ldo_disable(&mut self) -> LDO_DISABLE_W<'_, LDOrs> {
        LDO_DISABLE_W::new(self, 2)
    }
}
/**USBPHYC LDO control and status register

You can [`read`](crate::Reg::read) this register and get [`ldo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F730.html#USBPHYC:LDO)*/
pub struct LDOrs;
impl crate::RegisterSpec for LDOrs {
    type Ux = u32;
}
///`read()` method returns [`ldo::R`](R) reader structure
impl crate::Readable for LDOrs {}
///`write(|w| ..)` method takes [`ldo::W`](W) writer structure
impl crate::Writable for LDOrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LDO to value 0x01
impl crate::Resettable for LDOrs {
    const RESET_VALUE: u32 = 0x01;
}
