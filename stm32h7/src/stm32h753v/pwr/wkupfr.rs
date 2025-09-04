///Register `WKUPFR` reader
pub type R = crate::R<WKUPFRrs>;
///Register `WKUPFR` writer
pub type W = crate::W<WKUPFRrs>;
///Field `WKUPF1` reader - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR).
pub type WKUPF1_R = crate::BitReader;
///Field `WKUPF1` writer - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR).
pub type WKUPF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPF2` reader - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR).
pub type WKUPF2_R = crate::BitReader;
///Field `WKUPF2` writer - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR).
pub type WKUPF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPF3` reader - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR).
pub type WKUPF3_R = crate::BitReader;
///Field `WKUPF3` writer - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR).
pub type WKUPF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPF4` reader - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR).
pub type WKUPF4_R = crate::BitReader;
///Field `WKUPF4` writer - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR).
pub type WKUPF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPF5` reader - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR).
pub type WKUPF5_R = crate::BitReader;
///Field `WKUPF5` writer - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR).
pub type WKUPF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPF6` reader - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR).
pub type WKUPF6_R = crate::BitReader;
///Field `WKUPF6` writer - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR).
pub type WKUPF6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR).
    #[inline(always)]
    pub fn wkupf1(&self) -> WKUPF1_R {
        WKUPF1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR).
    #[inline(always)]
    pub fn wkupf2(&self) -> WKUPF2_R {
        WKUPF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR).
    #[inline(always)]
    pub fn wkupf3(&self) -> WKUPF3_R {
        WKUPF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR).
    #[inline(always)]
    pub fn wkupf4(&self) -> WKUPF4_R {
        WKUPF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR).
    #[inline(always)]
    pub fn wkupf5(&self) -> WKUPF5_R {
        WKUPF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR).
    #[inline(always)]
    pub fn wkupf6(&self) -> WKUPF6_R {
        WKUPF6_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WKUPFR")
            .field("wkupf1", &self.wkupf1())
            .field("wkupf2", &self.wkupf2())
            .field("wkupf3", &self.wkupf3())
            .field("wkupf4", &self.wkupf4())
            .field("wkupf5", &self.wkupf5())
            .field("wkupf6", &self.wkupf6())
            .finish()
    }
}
impl W {
    ///Bit 0 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR).
    #[inline(always)]
    pub fn wkupf1(&mut self) -> WKUPF1_W<WKUPFRrs> {
        WKUPF1_W::new(self, 0)
    }
    ///Bit 1 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR).
    #[inline(always)]
    pub fn wkupf2(&mut self) -> WKUPF2_W<WKUPFRrs> {
        WKUPF2_W::new(self, 1)
    }
    ///Bit 2 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR).
    #[inline(always)]
    pub fn wkupf3(&mut self) -> WKUPF3_W<WKUPFRrs> {
        WKUPF3_W::new(self, 2)
    }
    ///Bit 3 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR).
    #[inline(always)]
    pub fn wkupf4(&mut self) -> WKUPF4_W<WKUPFRrs> {
        WKUPF4_W::new(self, 3)
    }
    ///Bit 4 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR).
    #[inline(always)]
    pub fn wkupf5(&mut self) -> WKUPF5_W<WKUPFRrs> {
        WKUPF5_W::new(self, 4)
    }
    ///Bit 5 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR).
    #[inline(always)]
    pub fn wkupf6(&mut self) -> WKUPF6_W<WKUPFRrs> {
        WKUPF6_W::new(self, 5)
    }
}
/**reset only by system reset, not reset by wakeup from Standby mode

You can [`read`](crate::Reg::read) this register and get [`wkupfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkupfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#PWR:WKUPFR)*/
pub struct WKUPFRrs;
impl crate::RegisterSpec for WKUPFRrs {
    type Ux = u32;
}
///`read()` method returns [`wkupfr::R`](R) reader structure
impl crate::Readable for WKUPFRrs {}
///`write(|w| ..)` method takes [`wkupfr::W`](W) writer structure
impl crate::Writable for WKUPFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WKUPFR to value 0
impl crate::Resettable for WKUPFRrs {}
