#[doc = "Register `CKGATENR` reader"]
pub type R = crate::R<CKGATENRrs>;
#[doc = "Register `CKGATENR` writer"]
pub type W = crate::W<CKGATENRrs>;
#[doc = "AHB to APB1 Bridge clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHB2APB1_CKEN {
    #[doc = "0: The clock gating is enabled"]
    Enabled = 0,
    #[doc = "1: The clock gating is disabled, the clock is always enabled"]
    Disabled = 1,
}
impl From<AHB2APB1_CKEN> for bool {
    #[inline(always)]
    fn from(variant: AHB2APB1_CKEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHB2APB1_CKEN` reader - AHB to APB1 Bridge clock enable"]
pub type AHB2APB1_CKEN_R = crate::BitReader<AHB2APB1_CKEN>;
impl AHB2APB1_CKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AHB2APB1_CKEN {
        match self.bits {
            false => AHB2APB1_CKEN::Enabled,
            true => AHB2APB1_CKEN::Disabled,
        }
    }
    #[doc = "The clock gating is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AHB2APB1_CKEN::Enabled
    }
    #[doc = "The clock gating is disabled, the clock is always enabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AHB2APB1_CKEN::Disabled
    }
}
#[doc = "Field `AHB2APB1_CKEN` writer - AHB to APB1 Bridge clock enable"]
pub type AHB2APB1_CKEN_W<'a, REG> = crate::BitWriter<'a, REG, AHB2APB1_CKEN>;
impl<'a, REG> AHB2APB1_CKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AHB2APB1_CKEN::Enabled)
    }
    #[doc = "The clock gating is disabled, the clock is always enabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AHB2APB1_CKEN::Disabled)
    }
}
#[doc = "Field `AHB2APB2_CKEN` reader - AHB to APB2 Bridge clock enable"]
pub use AHB2APB1_CKEN_R as AHB2APB2_CKEN_R;
#[doc = "Field `CM4DBG_CKEN` reader - Cortex M4 ETM clock enable"]
pub use AHB2APB1_CKEN_R as CM4DBG_CKEN_R;
#[doc = "Field `SPARE_CKEN` reader - Spare clock enable"]
pub use AHB2APB1_CKEN_R as SPARE_CKEN_R;
#[doc = "Field `SRAM_CKEN` reader - SRQAM controller clock enable"]
pub use AHB2APB1_CKEN_R as SRAM_CKEN_R;
#[doc = "Field `FLITF_CKEN` reader - Flash Interface clock enable"]
pub use AHB2APB1_CKEN_R as FLITF_CKEN_R;
#[doc = "Field `RCC_CKEN` reader - RCC clock enable"]
pub use AHB2APB1_CKEN_R as RCC_CKEN_R;
#[doc = "Field `EVTCL_CKEN` reader - EVTCL_CKEN"]
pub use AHB2APB1_CKEN_R as EVTCL_CKEN_R;
#[doc = "Field `AHB2APB2_CKEN` writer - AHB to APB2 Bridge clock enable"]
pub use AHB2APB1_CKEN_W as AHB2APB2_CKEN_W;
#[doc = "Field `CM4DBG_CKEN` writer - Cortex M4 ETM clock enable"]
pub use AHB2APB1_CKEN_W as CM4DBG_CKEN_W;
#[doc = "Field `SPARE_CKEN` writer - Spare clock enable"]
pub use AHB2APB1_CKEN_W as SPARE_CKEN_W;
#[doc = "Field `SRAM_CKEN` writer - SRQAM controller clock enable"]
pub use AHB2APB1_CKEN_W as SRAM_CKEN_W;
#[doc = "Field `FLITF_CKEN` writer - Flash Interface clock enable"]
pub use AHB2APB1_CKEN_W as FLITF_CKEN_W;
#[doc = "Field `RCC_CKEN` writer - RCC clock enable"]
pub use AHB2APB1_CKEN_W as RCC_CKEN_W;
#[doc = "Field `EVTCL_CKEN` writer - EVTCL_CKEN"]
pub use AHB2APB1_CKEN_W as EVTCL_CKEN_W;
impl R {
    #[doc = "Bit 0 - AHB to APB1 Bridge clock enable"]
    #[inline(always)]
    pub fn ahb2apb1_cken(&self) -> AHB2APB1_CKEN_R {
        AHB2APB1_CKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AHB to APB2 Bridge clock enable"]
    #[inline(always)]
    pub fn ahb2apb2_cken(&self) -> AHB2APB2_CKEN_R {
        AHB2APB2_CKEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cortex M4 ETM clock enable"]
    #[inline(always)]
    pub fn cm4dbg_cken(&self) -> CM4DBG_CKEN_R {
        CM4DBG_CKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Spare clock enable"]
    #[inline(always)]
    pub fn spare_cken(&self) -> SPARE_CKEN_R {
        SPARE_CKEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SRQAM controller clock enable"]
    #[inline(always)]
    pub fn sram_cken(&self) -> SRAM_CKEN_R {
        SRAM_CKEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flash Interface clock enable"]
    #[inline(always)]
    pub fn flitf_cken(&self) -> FLITF_CKEN_R {
        FLITF_CKEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RCC clock enable"]
    #[inline(always)]
    pub fn rcc_cken(&self) -> RCC_CKEN_R {
        RCC_CKEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EVTCL_CKEN"]
    #[inline(always)]
    pub fn evtcl_cken(&self) -> EVTCL_CKEN_R {
        EVTCL_CKEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AHB to APB1 Bridge clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ahb2apb1_cken(&mut self) -> AHB2APB1_CKEN_W<CKGATENRrs> {
        AHB2APB1_CKEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - AHB to APB2 Bridge clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ahb2apb2_cken(&mut self) -> AHB2APB2_CKEN_W<CKGATENRrs> {
        AHB2APB2_CKEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Cortex M4 ETM clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn cm4dbg_cken(&mut self) -> CM4DBG_CKEN_W<CKGATENRrs> {
        CM4DBG_CKEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Spare clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spare_cken(&mut self) -> SPARE_CKEN_W<CKGATENRrs> {
        SPARE_CKEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - SRQAM controller clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sram_cken(&mut self) -> SRAM_CKEN_W<CKGATENRrs> {
        SRAM_CKEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Flash Interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn flitf_cken(&mut self) -> FLITF_CKEN_W<CKGATENRrs> {
        FLITF_CKEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - RCC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn rcc_cken(&mut self) -> RCC_CKEN_W<CKGATENRrs> {
        RCC_CKEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - EVTCL_CKEN"]
    #[inline(always)]
    #[must_use]
    pub fn evtcl_cken(&mut self) -> EVTCL_CKEN_W<CKGATENRrs> {
        EVTCL_CKEN_W::new(self, 7)
    }
}
#[doc = "RCC clocks gated enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckgatenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckgatenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CKGATENRrs;
impl crate::RegisterSpec for CKGATENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckgatenr::R`](R) reader structure"]
impl crate::Readable for CKGATENRrs {}
#[doc = "`write(|w| ..)` method takes [`ckgatenr::W`](W) writer structure"]
impl crate::Writable for CKGATENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CKGATENR to value 0"]
impl crate::Resettable for CKGATENRrs {
    const RESET_VALUE: u32 = 0;
}
