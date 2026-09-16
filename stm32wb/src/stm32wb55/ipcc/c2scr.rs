///Register `C2SCR` writer
pub type W = crate::W<C2SCRrs>;
///Field `CH1C` writer - processor 2 Receive channel 1 status clear
pub type CH1C_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH2C` writer - processor 2 Receive channel 2 status clear
pub type CH2C_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH3C` writer - processor 2 Receive channel 3 status clear
pub type CH3C_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH4C` writer - processor 2 Receive channel 4 status clear
pub type CH4C_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH5C` writer - processor 2 Receive channel 5 status clear
pub type CH5C_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH6C` writer - processor 2 Receive channel 6 status clear
pub type CH6C_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1S` writer - processor 2 Transmit channel 1 status set
pub type CH1S_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH2S` writer - processor 2 Transmit channel 2 status set
pub type CH2S_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH3S` writer - processor 2 Transmit channel 3 status set
pub type CH3S_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH4S` writer - processor 2 Transmit channel 4 status set
pub type CH4S_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH5S` writer - processor 2 Transmit channel 5 status set
pub type CH5S_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH6S` writer - processor 2 Transmit channel 6 status set
pub type CH6S_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<C2SCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - processor 2 Receive channel 1 status clear
    #[inline(always)]
    pub fn ch1c(&mut self) -> CH1C_W<'_, C2SCRrs> {
        CH1C_W::new(self, 0)
    }
    ///Bit 1 - processor 2 Receive channel 2 status clear
    #[inline(always)]
    pub fn ch2c(&mut self) -> CH2C_W<'_, C2SCRrs> {
        CH2C_W::new(self, 1)
    }
    ///Bit 2 - processor 2 Receive channel 3 status clear
    #[inline(always)]
    pub fn ch3c(&mut self) -> CH3C_W<'_, C2SCRrs> {
        CH3C_W::new(self, 2)
    }
    ///Bit 3 - processor 2 Receive channel 4 status clear
    #[inline(always)]
    pub fn ch4c(&mut self) -> CH4C_W<'_, C2SCRrs> {
        CH4C_W::new(self, 3)
    }
    ///Bit 4 - processor 2 Receive channel 5 status clear
    #[inline(always)]
    pub fn ch5c(&mut self) -> CH5C_W<'_, C2SCRrs> {
        CH5C_W::new(self, 4)
    }
    ///Bit 5 - processor 2 Receive channel 6 status clear
    #[inline(always)]
    pub fn ch6c(&mut self) -> CH6C_W<'_, C2SCRrs> {
        CH6C_W::new(self, 5)
    }
    ///Bit 16 - processor 2 Transmit channel 1 status set
    #[inline(always)]
    pub fn ch1s(&mut self) -> CH1S_W<'_, C2SCRrs> {
        CH1S_W::new(self, 16)
    }
    ///Bit 17 - processor 2 Transmit channel 2 status set
    #[inline(always)]
    pub fn ch2s(&mut self) -> CH2S_W<'_, C2SCRrs> {
        CH2S_W::new(self, 17)
    }
    ///Bit 18 - processor 2 Transmit channel 3 status set
    #[inline(always)]
    pub fn ch3s(&mut self) -> CH3S_W<'_, C2SCRrs> {
        CH3S_W::new(self, 18)
    }
    ///Bit 19 - processor 2 Transmit channel 4 status set
    #[inline(always)]
    pub fn ch4s(&mut self) -> CH4S_W<'_, C2SCRrs> {
        CH4S_W::new(self, 19)
    }
    ///Bit 20 - processor 2 Transmit channel 5 status set
    #[inline(always)]
    pub fn ch5s(&mut self) -> CH5S_W<'_, C2SCRrs> {
        CH5S_W::new(self, 20)
    }
    ///Bit 21 - processor 2 Transmit channel 6 status set
    #[inline(always)]
    pub fn ch6s(&mut self) -> CH6S_W<'_, C2SCRrs> {
        CH6S_W::new(self, 21)
    }
}
/**Status Set or Clear register CPU2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#IPCC:C2SCR)*/
pub struct C2SCRrs;
impl crate::RegisterSpec for C2SCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`c2scr::W`](W) writer structure
impl crate::Writable for C2SCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2SCR to value 0
impl crate::Resettable for C2SCRrs {}
