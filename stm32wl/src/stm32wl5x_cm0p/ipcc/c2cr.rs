#[doc = "Register `C2CR` reader"]
pub type R = crate::R<C2CRrs>;
#[doc = "Register `C2CR` writer"]
pub type W = crate::W<C2CRrs>;
#[doc = "RXOIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXOIE {
    #[doc = "0: Processor RX occupied interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Enable an unmasked processor receive channel occupied to generate an RX occupied interrupt"]
    Enabled = 1,
}
impl From<RXOIE> for bool {
    #[inline(always)]
    fn from(variant: RXOIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXOIE` reader - RXOIE"]
pub type RXOIE_R = crate::BitReader<RXOIE>;
impl RXOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXOIE {
        match self.bits {
            false => RXOIE::Disabled,
            true => RXOIE::Enabled,
        }
    }
    #[doc = "Processor RX occupied interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXOIE::Disabled
    }
    #[doc = "Enable an unmasked processor receive channel occupied to generate an RX occupied interrupt"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXOIE::Enabled
    }
}
#[doc = "Field `RXOIE` writer - RXOIE"]
pub type RXOIE_W<'a, REG> = crate::BitWriter<'a, REG, RXOIE>;
impl<'a, REG> RXOIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Processor RX occupied interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXOIE::Disabled)
    }
    #[doc = "Enable an unmasked processor receive channel occupied to generate an RX occupied interrupt"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXOIE::Enabled)
    }
}
#[doc = "TXFIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFIE {
    #[doc = "0: Processor TX free interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Enable an unmasked processor transmit channel free to generate a TX free interrupt"]
    Enabled = 1,
}
impl From<TXFIE> for bool {
    #[inline(always)]
    fn from(variant: TXFIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFIE` reader - TXFIE"]
pub type TXFIE_R = crate::BitReader<TXFIE>;
impl TXFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXFIE {
        match self.bits {
            false => TXFIE::Disabled,
            true => TXFIE::Enabled,
        }
    }
    #[doc = "Processor TX free interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXFIE::Disabled
    }
    #[doc = "Enable an unmasked processor transmit channel free to generate a TX free interrupt"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXFIE::Enabled
    }
}
#[doc = "Field `TXFIE` writer - TXFIE"]
pub type TXFIE_W<'a, REG> = crate::BitWriter<'a, REG, TXFIE>;
impl<'a, REG> TXFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Processor TX free interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXFIE::Disabled)
    }
    #[doc = "Enable an unmasked processor transmit channel free to generate a TX free interrupt"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXFIE::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - RXOIE"]
    #[inline(always)]
    pub fn rxoie(&self) -> RXOIE_R {
        RXOIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - TXFIE"]
    #[inline(always)]
    pub fn txfie(&self) -> TXFIE_R {
        TXFIE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXOIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxoie(&mut self) -> RXOIE_W<C2CRrs> {
        RXOIE_W::new(self, 0)
    }
    #[doc = "Bit 16 - TXFIE"]
    #[inline(always)]
    #[must_use]
    pub fn txfie(&mut self) -> TXFIE_W<C2CRrs> {
        TXFIE_W::new(self, 16)
    }
}
#[doc = "IPCC Processor 2 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2CRrs;
impl crate::RegisterSpec for C2CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2cr::R`](R) reader structure"]
impl crate::Readable for C2CRrs {}
#[doc = "`write(|w| ..)` method takes [`c2cr::W`](W) writer structure"]
impl crate::Writable for C2CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2CR to value 0"]
impl crate::Resettable for C2CRrs {
    const RESET_VALUE: u32 = 0;
}
