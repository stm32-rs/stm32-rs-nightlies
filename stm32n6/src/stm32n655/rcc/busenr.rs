///Register `BUSENR` reader
pub type R = crate::R<BUSENRrs>;
///Register `BUSENR` writer
pub type W = crate::W<BUSENRrs>;
///Field `ACLKNEN` reader - ACLKN enable
pub type ACLKNEN_R = crate::BitReader;
///Field `ACLKNEN` writer - ACLKN enable
pub type ACLKNEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACLKNCEN` reader - ACLKNC enable
pub type ACLKNCEN_R = crate::BitReader;
///Field `ACLKNCEN` writer - ACLKNC enable
pub type ACLKNCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBMEN` reader - AHBM enable
pub type AHBMEN_R = crate::BitReader;
///Field `AHBMEN` writer - AHBM enable
pub type AHBMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB1EN` reader - AHB1 enable
pub type AHB1EN_R = crate::BitReader;
///Field `AHB1EN` writer - AHB1 enable
pub type AHB1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB2EN` reader - AHB2 enable
pub type AHB2EN_R = crate::BitReader;
///Field `AHB2EN` writer - AHB2 enable
pub type AHB2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB3EN` reader - AHB3 enable
pub type AHB3EN_R = crate::BitReader;
///Field `AHB3EN` writer - AHB3 enable
pub type AHB3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB4EN` reader - AHB4 enable
pub type AHB4EN_R = crate::BitReader;
///Field `AHB4EN` writer - AHB4 enable
pub type AHB4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB5EN` reader - AHB5 enable
pub type AHB5EN_R = crate::BitReader;
///Field `AHB5EN` writer - AHB5 enable
pub type AHB5EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB1EN` reader - APB1 enable
pub type APB1EN_R = crate::BitReader;
///Field `APB1EN` writer - APB1 enable
pub type APB1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB2EN` reader - APB2 enable
pub type APB2EN_R = crate::BitReader;
///Field `APB2EN` writer - APB2 enable
pub type APB2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB3EN` reader - APB3 enable
pub type APB3EN_R = crate::BitReader;
///Field `APB3EN` writer - APB3 enable
pub type APB3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB4EN` reader - APB4 enable
pub type APB4EN_R = crate::BitReader;
///Field `APB4EN` writer - APB4 enable
pub type APB4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB5EN` reader - APB5 enable
pub type APB5EN_R = crate::BitReader;
///Field `APB5EN` writer - APB5 enable
pub type APB5EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ACLKN enable
    #[inline(always)]
    pub fn aclknen(&self) -> ACLKNEN_R {
        ACLKNEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ACLKNC enable
    #[inline(always)]
    pub fn aclkncen(&self) -> ACLKNCEN_R {
        ACLKNCEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AHBM enable
    #[inline(always)]
    pub fn ahbmen(&self) -> AHBMEN_R {
        AHBMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AHB1 enable
    #[inline(always)]
    pub fn ahb1en(&self) -> AHB1EN_R {
        AHB1EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AHB2 enable
    #[inline(always)]
    pub fn ahb2en(&self) -> AHB2EN_R {
        AHB2EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AHB3 enable
    #[inline(always)]
    pub fn ahb3en(&self) -> AHB3EN_R {
        AHB3EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AHB4 enable
    #[inline(always)]
    pub fn ahb4en(&self) -> AHB4EN_R {
        AHB4EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - AHB5 enable
    #[inline(always)]
    pub fn ahb5en(&self) -> AHB5EN_R {
        AHB5EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - APB1 enable
    #[inline(always)]
    pub fn apb1en(&self) -> APB1EN_R {
        APB1EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - APB2 enable
    #[inline(always)]
    pub fn apb2en(&self) -> APB2EN_R {
        APB2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - APB3 enable
    #[inline(always)]
    pub fn apb3en(&self) -> APB3EN_R {
        APB3EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - APB4 enable
    #[inline(always)]
    pub fn apb4en(&self) -> APB4EN_R {
        APB4EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - APB5 enable
    #[inline(always)]
    pub fn apb5en(&self) -> APB5EN_R {
        APB5EN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUSENR")
            .field("aclknen", &self.aclknen())
            .field("aclkncen", &self.aclkncen())
            .field("ahbmen", &self.ahbmen())
            .field("ahb1en", &self.ahb1en())
            .field("ahb2en", &self.ahb2en())
            .field("ahb3en", &self.ahb3en())
            .field("ahb4en", &self.ahb4en())
            .field("ahb5en", &self.ahb5en())
            .field("apb1en", &self.apb1en())
            .field("apb2en", &self.apb2en())
            .field("apb3en", &self.apb3en())
            .field("apb4en", &self.apb4en())
            .field("apb5en", &self.apb5en())
            .finish()
    }
}
impl W {
    ///Bit 0 - ACLKN enable
    #[inline(always)]
    pub fn aclknen(&mut self) -> ACLKNEN_W<'_, BUSENRrs> {
        ACLKNEN_W::new(self, 0)
    }
    ///Bit 1 - ACLKNC enable
    #[inline(always)]
    pub fn aclkncen(&mut self) -> ACLKNCEN_W<'_, BUSENRrs> {
        ACLKNCEN_W::new(self, 1)
    }
    ///Bit 2 - AHBM enable
    #[inline(always)]
    pub fn ahbmen(&mut self) -> AHBMEN_W<'_, BUSENRrs> {
        AHBMEN_W::new(self, 2)
    }
    ///Bit 3 - AHB1 enable
    #[inline(always)]
    pub fn ahb1en(&mut self) -> AHB1EN_W<'_, BUSENRrs> {
        AHB1EN_W::new(self, 3)
    }
    ///Bit 4 - AHB2 enable
    #[inline(always)]
    pub fn ahb2en(&mut self) -> AHB2EN_W<'_, BUSENRrs> {
        AHB2EN_W::new(self, 4)
    }
    ///Bit 5 - AHB3 enable
    #[inline(always)]
    pub fn ahb3en(&mut self) -> AHB3EN_W<'_, BUSENRrs> {
        AHB3EN_W::new(self, 5)
    }
    ///Bit 6 - AHB4 enable
    #[inline(always)]
    pub fn ahb4en(&mut self) -> AHB4EN_W<'_, BUSENRrs> {
        AHB4EN_W::new(self, 6)
    }
    ///Bit 7 - AHB5 enable
    #[inline(always)]
    pub fn ahb5en(&mut self) -> AHB5EN_W<'_, BUSENRrs> {
        AHB5EN_W::new(self, 7)
    }
    ///Bit 8 - APB1 enable
    #[inline(always)]
    pub fn apb1en(&mut self) -> APB1EN_W<'_, BUSENRrs> {
        APB1EN_W::new(self, 8)
    }
    ///Bit 9 - APB2 enable
    #[inline(always)]
    pub fn apb2en(&mut self) -> APB2EN_W<'_, BUSENRrs> {
        APB2EN_W::new(self, 9)
    }
    ///Bit 10 - APB3 enable
    #[inline(always)]
    pub fn apb3en(&mut self) -> APB3EN_W<'_, BUSENRrs> {
        APB3EN_W::new(self, 10)
    }
    ///Bit 11 - APB4 enable
    #[inline(always)]
    pub fn apb4en(&mut self) -> APB4EN_W<'_, BUSENRrs> {
        APB4EN_W::new(self, 11)
    }
    ///Bit 12 - APB5 enable
    #[inline(always)]
    pub fn apb5en(&mut self) -> APB5EN_W<'_, BUSENRrs> {
        APB5EN_W::new(self, 12)
    }
}
/**RCC SoC buses enable register

You can [`read`](crate::Reg::read) this register and get [`busenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:BUSENR)*/
pub struct BUSENRrs;
impl crate::RegisterSpec for BUSENRrs {
    type Ux = u32;
}
///`read()` method returns [`busenr::R`](R) reader structure
impl crate::Readable for BUSENRrs {}
///`write(|w| ..)` method takes [`busenr::W`](W) writer structure
impl crate::Writable for BUSENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BUSENR to value 0x03
impl crate::Resettable for BUSENRrs {
    const RESET_VALUE: u32 = 0x03;
}
