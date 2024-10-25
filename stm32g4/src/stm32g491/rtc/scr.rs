///Register `SCR` writer
pub type W = crate::W<SCRrs>;
///Field `CALRAF` writer - CALRAF
pub type CALRAF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALRBF` writer - CALRBF
pub type CALRBF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUTF` writer - CWUTF
pub type CWUTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSF` writer - CTSF
pub type CTSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSOVF` writer - CTSOVF
pub type CTSOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITSF` writer - CITSF
pub type CITSF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<SCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - CALRAF
    #[inline(always)]
    #[must_use]
    pub fn calraf(&mut self) -> CALRAF_W<SCRrs> {
        CALRAF_W::new(self, 0)
    }
    ///Bit 1 - CALRBF
    #[inline(always)]
    #[must_use]
    pub fn calrbf(&mut self) -> CALRBF_W<SCRrs> {
        CALRBF_W::new(self, 1)
    }
    ///Bit 2 - CWUTF
    #[inline(always)]
    #[must_use]
    pub fn cwutf(&mut self) -> CWUTF_W<SCRrs> {
        CWUTF_W::new(self, 2)
    }
    ///Bit 3 - CTSF
    #[inline(always)]
    #[must_use]
    pub fn ctsf(&mut self) -> CTSF_W<SCRrs> {
        CTSF_W::new(self, 3)
    }
    ///Bit 4 - CTSOVF
    #[inline(always)]
    #[must_use]
    pub fn ctsovf(&mut self) -> CTSOVF_W<SCRrs> {
        CTSOVF_W::new(self, 4)
    }
    ///Bit 5 - CITSF
    #[inline(always)]
    #[must_use]
    pub fn citsf(&mut self) -> CITSF_W<SCRrs> {
        CITSF_W::new(self, 5)
    }
}
/**status register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G491xx.html#RTC:SCR)*/
pub struct SCRrs;
impl crate::RegisterSpec for SCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`scr::W`](W) writer structure
impl crate::Writable for SCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SCR to value 0
impl crate::Resettable for SCRrs {
    const RESET_VALUE: u32 = 0;
}
