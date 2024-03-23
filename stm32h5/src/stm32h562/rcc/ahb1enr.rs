#[doc = "Register `AHB1ENR` reader"]
pub type R = crate::R<AHB1ENRrs>;
#[doc = "Register `AHB1ENR` writer"]
pub type W = crate::W<AHB1ENRrs>;
#[doc = "GPDMA1 clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPDMA1EN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<GPDMA1EN> for bool {
    #[inline(always)]
    fn from(variant: GPDMA1EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPDMA1EN` reader - GPDMA1 clock enable Set and reset by software."]
pub type GPDMA1EN_R = crate::BitReader<GPDMA1EN>;
impl GPDMA1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPDMA1EN {
        match self.bits {
            false => GPDMA1EN::Disabled,
            true => GPDMA1EN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPDMA1EN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPDMA1EN::Enabled
    }
}
#[doc = "Field `GPDMA1EN` writer - GPDMA1 clock enable Set and reset by software."]
pub type GPDMA1EN_W<'a, REG> = crate::BitWriter<'a, REG, GPDMA1EN>;
impl<'a, REG> GPDMA1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA1EN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA1EN::Enabled)
    }
}
#[doc = "Field `GPDMA2EN` reader - GPDMA2 clock enable Set and reset by software."]
pub use GPDMA1EN_R as GPDMA2EN_R;
#[doc = "Field `FLITFEN` reader - Flash interface clock enable Set and reset by software."]
pub use GPDMA1EN_R as FLITFEN_R;
#[doc = "Field `CRCEN` reader - CRC clock enable Set and reset by software."]
pub use GPDMA1EN_R as CRCEN_R;
#[doc = "Field `CORDICEN` reader - CORDIC clock enable Set and reset by software."]
pub use GPDMA1EN_R as CORDICEN_R;
#[doc = "Field `FMACEN` reader - FMAC clock enable Set and reset by software."]
pub use GPDMA1EN_R as FMACEN_R;
#[doc = "Field `RAMCFGEN` reader - RAMCFG clock enable Set and reset by software."]
pub use GPDMA1EN_R as RAMCFGEN_R;
#[doc = "Field `TZSC1EN` reader - TZSC1 clock enable Set and reset by software"]
pub use GPDMA1EN_R as TZSC1EN_R;
#[doc = "Field `BKPRAMEN` reader - BKPRAM clock enable Set and reset by software"]
pub use GPDMA1EN_R as BKPRAMEN_R;
#[doc = "Field `DCACHEEN` reader - DCACHE clock enable Set and reset by software"]
pub use GPDMA1EN_R as DCACHEEN_R;
#[doc = "Field `SRAM1EN` reader - SRAM1 clock enable Set and reset by software."]
pub use GPDMA1EN_R as SRAM1EN_R;
#[doc = "Field `GPDMA2EN` writer - GPDMA2 clock enable Set and reset by software."]
pub use GPDMA1EN_W as GPDMA2EN_W;
#[doc = "Field `FLITFEN` writer - Flash interface clock enable Set and reset by software."]
pub use GPDMA1EN_W as FLITFEN_W;
#[doc = "Field `CRCEN` writer - CRC clock enable Set and reset by software."]
pub use GPDMA1EN_W as CRCEN_W;
#[doc = "Field `CORDICEN` writer - CORDIC clock enable Set and reset by software."]
pub use GPDMA1EN_W as CORDICEN_W;
#[doc = "Field `FMACEN` writer - FMAC clock enable Set and reset by software."]
pub use GPDMA1EN_W as FMACEN_W;
#[doc = "Field `RAMCFGEN` writer - RAMCFG clock enable Set and reset by software."]
pub use GPDMA1EN_W as RAMCFGEN_W;
#[doc = "Field `TZSC1EN` writer - TZSC1 clock enable Set and reset by software"]
pub use GPDMA1EN_W as TZSC1EN_W;
#[doc = "Field `BKPRAMEN` writer - BKPRAM clock enable Set and reset by software"]
pub use GPDMA1EN_W as BKPRAMEN_W;
#[doc = "Field `DCACHEEN` writer - DCACHE clock enable Set and reset by software"]
pub use GPDMA1EN_W as DCACHEEN_W;
#[doc = "Field `SRAM1EN` writer - SRAM1 clock enable Set and reset by software."]
pub use GPDMA1EN_W as SRAM1EN_W;
impl R {
    #[doc = "Bit 0 - GPDMA1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn gpdma1en(&self) -> GPDMA1EN_R {
        GPDMA1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPDMA2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn gpdma2en(&self) -> GPDMA2EN_R {
        GPDMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash interface clock enable Set and reset by software."]
    #[inline(always)]
    pub fn flitfen(&self) -> FLITFEN_R {
        FLITFEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable Set and reset by software."]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - CORDIC clock enable Set and reset by software."]
    #[inline(always)]
    pub fn cordicen(&self) -> CORDICEN_R {
        CORDICEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - FMAC clock enable Set and reset by software."]
    #[inline(always)]
    pub fn fmacen(&self) -> FMACEN_R {
        FMACEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - RAMCFG clock enable Set and reset by software."]
    #[inline(always)]
    pub fn ramcfgen(&self) -> RAMCFGEN_R {
        RAMCFGEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - TZSC1 clock enable Set and reset by software"]
    #[inline(always)]
    pub fn tzsc1en(&self) -> TZSC1EN_R {
        TZSC1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - BKPRAM clock enable Set and reset by software"]
    #[inline(always)]
    pub fn bkpramen(&self) -> BKPRAMEN_R {
        BKPRAMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - DCACHE clock enable Set and reset by software"]
    #[inline(always)]
    pub fn dcacheen(&self) -> DCACHEEN_R {
        DCACHEEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SRAM1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn sram1en(&self) -> SRAM1EN_R {
        SRAM1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPDMA1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpdma1en(&mut self) -> GPDMA1EN_W<AHB1ENRrs> {
        GPDMA1EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPDMA2 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpdma2en(&mut self) -> GPDMA2EN_W<AHB1ENRrs> {
        GPDMA2EN_W::new(self, 1)
    }
    #[doc = "Bit 8 - Flash interface clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn flitfen(&mut self) -> FLITFEN_W<AHB1ENRrs> {
        FLITFEN_W::new(self, 8)
    }
    #[doc = "Bit 12 - CRC clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<AHB1ENRrs> {
        CRCEN_W::new(self, 12)
    }
    #[doc = "Bit 14 - CORDIC clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn cordicen(&mut self) -> CORDICEN_W<AHB1ENRrs> {
        CORDICEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - FMAC clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn fmacen(&mut self) -> FMACEN_W<AHB1ENRrs> {
        FMACEN_W::new(self, 15)
    }
    #[doc = "Bit 17 - RAMCFG clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn ramcfgen(&mut self) -> RAMCFGEN_W<AHB1ENRrs> {
        RAMCFGEN_W::new(self, 17)
    }
    #[doc = "Bit 24 - TZSC1 clock enable Set and reset by software"]
    #[inline(always)]
    #[must_use]
    pub fn tzsc1en(&mut self) -> TZSC1EN_W<AHB1ENRrs> {
        TZSC1EN_W::new(self, 24)
    }
    #[doc = "Bit 28 - BKPRAM clock enable Set and reset by software"]
    #[inline(always)]
    #[must_use]
    pub fn bkpramen(&mut self) -> BKPRAMEN_W<AHB1ENRrs> {
        BKPRAMEN_W::new(self, 28)
    }
    #[doc = "Bit 30 - DCACHE clock enable Set and reset by software"]
    #[inline(always)]
    #[must_use]
    pub fn dcacheen(&mut self) -> DCACHEEN_W<AHB1ENRrs> {
        DCACHEEN_W::new(self, 30)
    }
    #[doc = "Bit 31 - SRAM1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sram1en(&mut self) -> SRAM1EN_W<AHB1ENRrs> {
        SRAM1EN_W::new(self, 31)
    }
}
#[doc = "RCC AHB1 peripherals clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB1ENRrs;
impl crate::RegisterSpec for AHB1ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1enr::R`](R) reader structure"]
impl crate::Readable for AHB1ENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb1enr::W`](W) writer structure"]
impl crate::Writable for AHB1ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB1ENR to value 0xd000_0100"]
impl crate::Resettable for AHB1ENRrs {
    const RESET_VALUE: u32 = 0xd000_0100;
}
