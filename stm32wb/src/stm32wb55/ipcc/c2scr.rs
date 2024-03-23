#[doc = "Register `C2SCR` writer"]
pub type W = crate::W<C2SCRrs>;
#[doc = "Field `CH1C` writer - processor 2 Receive channel 1 status clear"]
pub type CH1C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2C` writer - processor 2 Receive channel 2 status clear"]
pub type CH2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3C` writer - processor 2 Receive channel 3 status clear"]
pub type CH3C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4C` writer - processor 2 Receive channel 4 status clear"]
pub type CH4C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5C` writer - processor 2 Receive channel 5 status clear"]
pub type CH5C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6C` writer - processor 2 Receive channel 6 status clear"]
pub type CH6C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1S` writer - processor 2 Transmit channel 1 status set"]
pub type CH1S_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2S` writer - processor 2 Transmit channel 2 status set"]
pub type CH2S_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3S` writer - processor 2 Transmit channel 3 status set"]
pub type CH3S_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4S` writer - processor 2 Transmit channel 4 status set"]
pub type CH4S_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5S` writer - processor 2 Transmit channel 5 status set"]
pub type CH5S_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6S` writer - processor 2 Transmit channel 6 status set"]
pub type CH6S_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - processor 2 Receive channel 1 status clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch1c(&mut self) -> CH1C_W<C2SCRrs> {
        CH1C_W::new(self, 0)
    }
    #[doc = "Bit 1 - processor 2 Receive channel 2 status clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch2c(&mut self) -> CH2C_W<C2SCRrs> {
        CH2C_W::new(self, 1)
    }
    #[doc = "Bit 2 - processor 2 Receive channel 3 status clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch3c(&mut self) -> CH3C_W<C2SCRrs> {
        CH3C_W::new(self, 2)
    }
    #[doc = "Bit 3 - processor 2 Receive channel 4 status clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch4c(&mut self) -> CH4C_W<C2SCRrs> {
        CH4C_W::new(self, 3)
    }
    #[doc = "Bit 4 - processor 2 Receive channel 5 status clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch5c(&mut self) -> CH5C_W<C2SCRrs> {
        CH5C_W::new(self, 4)
    }
    #[doc = "Bit 5 - processor 2 Receive channel 6 status clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch6c(&mut self) -> CH6C_W<C2SCRrs> {
        CH6C_W::new(self, 5)
    }
    #[doc = "Bit 16 - processor 2 Transmit channel 1 status set"]
    #[inline(always)]
    #[must_use]
    pub fn ch1s(&mut self) -> CH1S_W<C2SCRrs> {
        CH1S_W::new(self, 16)
    }
    #[doc = "Bit 17 - processor 2 Transmit channel 2 status set"]
    #[inline(always)]
    #[must_use]
    pub fn ch2s(&mut self) -> CH2S_W<C2SCRrs> {
        CH2S_W::new(self, 17)
    }
    #[doc = "Bit 18 - processor 2 Transmit channel 3 status set"]
    #[inline(always)]
    #[must_use]
    pub fn ch3s(&mut self) -> CH3S_W<C2SCRrs> {
        CH3S_W::new(self, 18)
    }
    #[doc = "Bit 19 - processor 2 Transmit channel 4 status set"]
    #[inline(always)]
    #[must_use]
    pub fn ch4s(&mut self) -> CH4S_W<C2SCRrs> {
        CH4S_W::new(self, 19)
    }
    #[doc = "Bit 20 - processor 2 Transmit channel 5 status set"]
    #[inline(always)]
    #[must_use]
    pub fn ch5s(&mut self) -> CH5S_W<C2SCRrs> {
        CH5S_W::new(self, 20)
    }
    #[doc = "Bit 21 - processor 2 Transmit channel 6 status set"]
    #[inline(always)]
    #[must_use]
    pub fn ch6s(&mut self) -> CH6S_W<C2SCRrs> {
        CH6S_W::new(self, 21)
    }
}
#[doc = "Status Set or Clear register CPU2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2SCRrs;
impl crate::RegisterSpec for C2SCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`c2scr::W`](W) writer structure"]
impl crate::Writable for C2SCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2SCR to value 0"]
impl crate::Resettable for C2SCRrs {
    const RESET_VALUE: u32 = 0;
}
