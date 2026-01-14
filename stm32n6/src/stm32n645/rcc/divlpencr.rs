///Register `DIVLPENCR` writer
pub type W = crate::W<DIVLPENCRrs>;
///Field `IC1LPENC` writer - IC1 sleep enable
pub type IC1LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC2LPENC` writer - IC2 sleep enable
pub type IC2LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC3LPENC` writer - IC3 sleep enable
pub type IC3LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC4LPENC` writer - IC4 sleep enable
pub type IC4LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC5LPENC` writer - IC5 sleep enable
pub type IC5LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC6LPENC` writer - IC6 sleep enable
pub type IC6LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC7LPENC` writer - IC7 sleep enable
pub type IC7LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC8LPENC` writer - IC8 sleep enable
pub type IC8LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC9LPENC` writer - IC9 sleep enable
pub type IC9LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC10LPENC` writer - IC10 sleep enable
pub type IC10LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC11LPENC` writer - IC11 sleep enable
pub type IC11LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC12LPENC` writer - IC12 sleep enable
pub type IC12LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC13LPENC` writer - IC13 sleep enable
pub type IC13LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC14LPENC` writer - IC14 sleep enable
pub type IC14LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC15LPENC` writer - IC15 sleep enable
pub type IC15LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC16LPENC` writer - IC16 sleep enable
pub type IC16LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC17LPENC` writer - IC17 sleep enable
pub type IC17LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC18LPENC` writer - IC18 sleep enable
pub type IC18LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC19LPENC` writer - IC19 sleep enable
pub type IC19LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC20LPENC` writer - IC20 sleep enable
pub type IC20LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<DIVLPENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - IC1 sleep enable
    #[inline(always)]
    pub fn ic1lpenc(&mut self) -> IC1LPENC_W<'_, DIVLPENCRrs> {
        IC1LPENC_W::new(self, 0)
    }
    ///Bit 1 - IC2 sleep enable
    #[inline(always)]
    pub fn ic2lpenc(&mut self) -> IC2LPENC_W<'_, DIVLPENCRrs> {
        IC2LPENC_W::new(self, 1)
    }
    ///Bit 2 - IC3 sleep enable
    #[inline(always)]
    pub fn ic3lpenc(&mut self) -> IC3LPENC_W<'_, DIVLPENCRrs> {
        IC3LPENC_W::new(self, 2)
    }
    ///Bit 3 - IC4 sleep enable
    #[inline(always)]
    pub fn ic4lpenc(&mut self) -> IC4LPENC_W<'_, DIVLPENCRrs> {
        IC4LPENC_W::new(self, 3)
    }
    ///Bit 4 - IC5 sleep enable
    #[inline(always)]
    pub fn ic5lpenc(&mut self) -> IC5LPENC_W<'_, DIVLPENCRrs> {
        IC5LPENC_W::new(self, 4)
    }
    ///Bit 5 - IC6 sleep enable
    #[inline(always)]
    pub fn ic6lpenc(&mut self) -> IC6LPENC_W<'_, DIVLPENCRrs> {
        IC6LPENC_W::new(self, 5)
    }
    ///Bit 6 - IC7 sleep enable
    #[inline(always)]
    pub fn ic7lpenc(&mut self) -> IC7LPENC_W<'_, DIVLPENCRrs> {
        IC7LPENC_W::new(self, 6)
    }
    ///Bit 7 - IC8 sleep enable
    #[inline(always)]
    pub fn ic8lpenc(&mut self) -> IC8LPENC_W<'_, DIVLPENCRrs> {
        IC8LPENC_W::new(self, 7)
    }
    ///Bit 8 - IC9 sleep enable
    #[inline(always)]
    pub fn ic9lpenc(&mut self) -> IC9LPENC_W<'_, DIVLPENCRrs> {
        IC9LPENC_W::new(self, 8)
    }
    ///Bit 9 - IC10 sleep enable
    #[inline(always)]
    pub fn ic10lpenc(&mut self) -> IC10LPENC_W<'_, DIVLPENCRrs> {
        IC10LPENC_W::new(self, 9)
    }
    ///Bit 10 - IC11 sleep enable
    #[inline(always)]
    pub fn ic11lpenc(&mut self) -> IC11LPENC_W<'_, DIVLPENCRrs> {
        IC11LPENC_W::new(self, 10)
    }
    ///Bit 11 - IC12 sleep enable
    #[inline(always)]
    pub fn ic12lpenc(&mut self) -> IC12LPENC_W<'_, DIVLPENCRrs> {
        IC12LPENC_W::new(self, 11)
    }
    ///Bit 12 - IC13 sleep enable
    #[inline(always)]
    pub fn ic13lpenc(&mut self) -> IC13LPENC_W<'_, DIVLPENCRrs> {
        IC13LPENC_W::new(self, 12)
    }
    ///Bit 13 - IC14 sleep enable
    #[inline(always)]
    pub fn ic14lpenc(&mut self) -> IC14LPENC_W<'_, DIVLPENCRrs> {
        IC14LPENC_W::new(self, 13)
    }
    ///Bit 14 - IC15 sleep enable
    #[inline(always)]
    pub fn ic15lpenc(&mut self) -> IC15LPENC_W<'_, DIVLPENCRrs> {
        IC15LPENC_W::new(self, 14)
    }
    ///Bit 15 - IC16 sleep enable
    #[inline(always)]
    pub fn ic16lpenc(&mut self) -> IC16LPENC_W<'_, DIVLPENCRrs> {
        IC16LPENC_W::new(self, 15)
    }
    ///Bit 16 - IC17 sleep enable
    #[inline(always)]
    pub fn ic17lpenc(&mut self) -> IC17LPENC_W<'_, DIVLPENCRrs> {
        IC17LPENC_W::new(self, 16)
    }
    ///Bit 17 - IC18 sleep enable
    #[inline(always)]
    pub fn ic18lpenc(&mut self) -> IC18LPENC_W<'_, DIVLPENCRrs> {
        IC18LPENC_W::new(self, 17)
    }
    ///Bit 18 - IC19 sleep enable
    #[inline(always)]
    pub fn ic19lpenc(&mut self) -> IC19LPENC_W<'_, DIVLPENCRrs> {
        IC19LPENC_W::new(self, 18)
    }
    ///Bit 19 - IC20 sleep enable
    #[inline(always)]
    pub fn ic20lpenc(&mut self) -> IC20LPENC_W<'_, DIVLPENCRrs> {
        IC20LPENC_W::new(self, 19)
    }
}
/**RCC divider Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divlpencr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:DIVLPENCR)*/
pub struct DIVLPENCRrs;
impl crate::RegisterSpec for DIVLPENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`divlpencr::W`](W) writer structure
impl crate::Writable for DIVLPENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIVLPENCR to value 0
impl crate::Resettable for DIVLPENCRrs {}
