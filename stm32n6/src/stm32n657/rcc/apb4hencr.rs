///Register `APB4HENCR` writer
pub type W = crate::W<APB4HENCRrs>;
///Field `SYSCFGENC` writer - SYSCFG enable
pub type SYSCFGENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSECENC` writer - BSEC enable
pub type BSECENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTSENC` writer - DTS enable
pub type DTSENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSPERFMENC` writer - BUSPERFM enable
pub type BUSPERFMENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB4HENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - SYSCFG enable
    #[inline(always)]
    pub fn syscfgenc(&mut self) -> SYSCFGENC_W<'_, APB4HENCRrs> {
        SYSCFGENC_W::new(self, 0)
    }
    ///Bit 1 - BSEC enable
    #[inline(always)]
    pub fn bsecenc(&mut self) -> BSECENC_W<'_, APB4HENCRrs> {
        BSECENC_W::new(self, 1)
    }
    ///Bit 2 - DTS enable
    #[inline(always)]
    pub fn dtsenc(&mut self) -> DTSENC_W<'_, APB4HENCRrs> {
        DTSENC_W::new(self, 2)
    }
    ///Bit 4 - BUSPERFM enable
    #[inline(always)]
    pub fn busperfmenc(&mut self) -> BUSPERFMENC_W<'_, APB4HENCRrs> {
        BUSPERFMENC_W::new(self, 4)
    }
}
/**RCC APB4H enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4hencr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:APB4HENCR)*/
pub struct APB4HENCRrs;
impl crate::RegisterSpec for APB4HENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb4hencr::W`](W) writer structure
impl crate::Writable for APB4HENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB4HENCR to value 0
impl crate::Resettable for APB4HENCRrs {}
