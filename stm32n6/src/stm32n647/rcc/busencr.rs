///Register `BUSENCR` writer
pub type W = crate::W<BUSENCRrs>;
///Field `ACLKNENC` writer - ACLKN enable
pub type ACLKNENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACLKNCENC` writer - ACLKNC enable
pub type ACLKNCENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBMENC` writer - AHBM enable
pub type AHBMENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB1ENC` writer - AHB1 enable
pub type AHB1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB2ENC` writer - AHB2 enable
pub type AHB2ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB3ENC` writer - AHB3 enable
pub type AHB3ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB4ENC` writer - AHB4 enable
pub type AHB4ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB5ENC` writer - AHB5 enable
pub type AHB5ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB1ENC` writer - APB1 enable
pub type APB1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB2ENC` writer - APB2 enable
pub type APB2ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB3ENC` writer - APB3 enable
pub type APB3ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB4ENC` writer - APB4 enable
pub type APB4ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB5ENC` writer - APB5 enable
pub type APB5ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<BUSENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - ACLKN enable
    #[inline(always)]
    pub fn aclknenc(&mut self) -> ACLKNENC_W<'_, BUSENCRrs> {
        ACLKNENC_W::new(self, 0)
    }
    ///Bit 1 - ACLKNC enable
    #[inline(always)]
    pub fn aclkncenc(&mut self) -> ACLKNCENC_W<'_, BUSENCRrs> {
        ACLKNCENC_W::new(self, 1)
    }
    ///Bit 2 - AHBM enable
    #[inline(always)]
    pub fn ahbmenc(&mut self) -> AHBMENC_W<'_, BUSENCRrs> {
        AHBMENC_W::new(self, 2)
    }
    ///Bit 3 - AHB1 enable
    #[inline(always)]
    pub fn ahb1enc(&mut self) -> AHB1ENC_W<'_, BUSENCRrs> {
        AHB1ENC_W::new(self, 3)
    }
    ///Bit 4 - AHB2 enable
    #[inline(always)]
    pub fn ahb2enc(&mut self) -> AHB2ENC_W<'_, BUSENCRrs> {
        AHB2ENC_W::new(self, 4)
    }
    ///Bit 5 - AHB3 enable
    #[inline(always)]
    pub fn ahb3enc(&mut self) -> AHB3ENC_W<'_, BUSENCRrs> {
        AHB3ENC_W::new(self, 5)
    }
    ///Bit 6 - AHB4 enable
    #[inline(always)]
    pub fn ahb4enc(&mut self) -> AHB4ENC_W<'_, BUSENCRrs> {
        AHB4ENC_W::new(self, 6)
    }
    ///Bit 7 - AHB5 enable
    #[inline(always)]
    pub fn ahb5enc(&mut self) -> AHB5ENC_W<'_, BUSENCRrs> {
        AHB5ENC_W::new(self, 7)
    }
    ///Bit 8 - APB1 enable
    #[inline(always)]
    pub fn apb1enc(&mut self) -> APB1ENC_W<'_, BUSENCRrs> {
        APB1ENC_W::new(self, 8)
    }
    ///Bit 9 - APB2 enable
    #[inline(always)]
    pub fn apb2enc(&mut self) -> APB2ENC_W<'_, BUSENCRrs> {
        APB2ENC_W::new(self, 9)
    }
    ///Bit 10 - APB3 enable
    #[inline(always)]
    pub fn apb3enc(&mut self) -> APB3ENC_W<'_, BUSENCRrs> {
        APB3ENC_W::new(self, 10)
    }
    ///Bit 11 - APB4 enable
    #[inline(always)]
    pub fn apb4enc(&mut self) -> APB4ENC_W<'_, BUSENCRrs> {
        APB4ENC_W::new(self, 11)
    }
    ///Bit 12 - APB5 enable
    #[inline(always)]
    pub fn apb5enc(&mut self) -> APB5ENC_W<'_, BUSENCRrs> {
        APB5ENC_W::new(self, 12)
    }
}
/**RCC bus enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busencr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:BUSENCR)*/
pub struct BUSENCRrs;
impl crate::RegisterSpec for BUSENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`busencr::W`](W) writer structure
impl crate::Writable for BUSENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BUSENCR to value 0
impl crate::Resettable for BUSENCRrs {}
