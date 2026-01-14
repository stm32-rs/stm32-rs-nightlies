///Register `CKGATENR` reader
pub type R = crate::R<CKGATENRrs>;
///Register `CKGATENR` writer
pub type W = crate::W<CKGATENRrs>;
/**AHB to APB1 Bridge clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHB2APB1_CKEN {
    ///0: The clock gating is enabled
    Enabled = 0,
    ///1: The clock gating is disabled, the clock is always enabled
    Disabled = 1,
}
impl From<AHB2APB1_CKEN> for bool {
    #[inline(always)]
    fn from(variant: AHB2APB1_CKEN) -> Self {
        variant as u8 != 0
    }
}
///Field `AHB2APB1_CKEN` reader - AHB to APB1 Bridge clock enable
pub type AHB2APB1_CKEN_R = crate::BitReader<AHB2APB1_CKEN>;
impl AHB2APB1_CKEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AHB2APB1_CKEN {
        match self.bits {
            false => AHB2APB1_CKEN::Enabled,
            true => AHB2APB1_CKEN::Disabled,
        }
    }
    ///The clock gating is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AHB2APB1_CKEN::Enabled
    }
    ///The clock gating is disabled, the clock is always enabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AHB2APB1_CKEN::Disabled
    }
}
///Field `AHB2APB1_CKEN` writer - AHB to APB1 Bridge clock enable
pub type AHB2APB1_CKEN_W<'a, REG> = crate::BitWriter<'a, REG, AHB2APB1_CKEN>;
impl<'a, REG> AHB2APB1_CKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The clock gating is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AHB2APB1_CKEN::Enabled)
    }
    ///The clock gating is disabled, the clock is always enabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AHB2APB1_CKEN::Disabled)
    }
}
///Field `AHB2APB2_CKEN` reader - AHB to APB2 Bridge clock enable
pub use AHB2APB1_CKEN_R as AHB2APB2_CKEN_R;
///Field `CM4DBG_CKEN` reader - Cortex M4 ETM clock enable
pub use AHB2APB1_CKEN_R as CM4DBG_CKEN_R;
///Field `SPARE_CKEN` reader - Spare clock enable
pub use AHB2APB1_CKEN_R as SPARE_CKEN_R;
///Field `SRAM_CKEN` reader - SRQAM controller clock enable
pub use AHB2APB1_CKEN_R as SRAM_CKEN_R;
///Field `FLITF_CKEN` reader - Flash Interface clock enable
pub use AHB2APB1_CKEN_R as FLITF_CKEN_R;
///Field `RCC_CKEN` reader - RCC clock enable
pub use AHB2APB1_CKEN_R as RCC_CKEN_R;
///Field `AHB2APB2_CKEN` writer - AHB to APB2 Bridge clock enable
pub use AHB2APB1_CKEN_W as AHB2APB2_CKEN_W;
///Field `CM4DBG_CKEN` writer - Cortex M4 ETM clock enable
pub use AHB2APB1_CKEN_W as CM4DBG_CKEN_W;
///Field `SPARE_CKEN` writer - Spare clock enable
pub use AHB2APB1_CKEN_W as SPARE_CKEN_W;
///Field `SRAM_CKEN` writer - SRQAM controller clock enable
pub use AHB2APB1_CKEN_W as SRAM_CKEN_W;
///Field `FLITF_CKEN` writer - Flash Interface clock enable
pub use AHB2APB1_CKEN_W as FLITF_CKEN_W;
///Field `RCC_CKEN` writer - RCC clock enable
pub use AHB2APB1_CKEN_W as RCC_CKEN_W;
impl R {
    ///Bit 0 - AHB to APB1 Bridge clock enable
    #[inline(always)]
    pub fn ahb2apb1_cken(&self) -> AHB2APB1_CKEN_R {
        AHB2APB1_CKEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AHB to APB2 Bridge clock enable
    #[inline(always)]
    pub fn ahb2apb2_cken(&self) -> AHB2APB2_CKEN_R {
        AHB2APB2_CKEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Cortex M4 ETM clock enable
    #[inline(always)]
    pub fn cm4dbg_cken(&self) -> CM4DBG_CKEN_R {
        CM4DBG_CKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Spare clock enable
    #[inline(always)]
    pub fn spare_cken(&self) -> SPARE_CKEN_R {
        SPARE_CKEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SRQAM controller clock enable
    #[inline(always)]
    pub fn sram_cken(&self) -> SRAM_CKEN_R {
        SRAM_CKEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Flash Interface clock enable
    #[inline(always)]
    pub fn flitf_cken(&self) -> FLITF_CKEN_R {
        FLITF_CKEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RCC clock enable
    #[inline(always)]
    pub fn rcc_cken(&self) -> RCC_CKEN_R {
        RCC_CKEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CKGATENR")
            .field("ahb2apb1_cken", &self.ahb2apb1_cken())
            .field("ahb2apb2_cken", &self.ahb2apb2_cken())
            .field("cm4dbg_cken", &self.cm4dbg_cken())
            .field("spare_cken", &self.spare_cken())
            .field("sram_cken", &self.sram_cken())
            .field("flitf_cken", &self.flitf_cken())
            .field("rcc_cken", &self.rcc_cken())
            .finish()
    }
}
impl W {
    ///Bit 0 - AHB to APB1 Bridge clock enable
    #[inline(always)]
    pub fn ahb2apb1_cken(&mut self) -> AHB2APB1_CKEN_W<'_, CKGATENRrs> {
        AHB2APB1_CKEN_W::new(self, 0)
    }
    ///Bit 1 - AHB to APB2 Bridge clock enable
    #[inline(always)]
    pub fn ahb2apb2_cken(&mut self) -> AHB2APB2_CKEN_W<'_, CKGATENRrs> {
        AHB2APB2_CKEN_W::new(self, 1)
    }
    ///Bit 2 - Cortex M4 ETM clock enable
    #[inline(always)]
    pub fn cm4dbg_cken(&mut self) -> CM4DBG_CKEN_W<'_, CKGATENRrs> {
        CM4DBG_CKEN_W::new(self, 2)
    }
    ///Bit 3 - Spare clock enable
    #[inline(always)]
    pub fn spare_cken(&mut self) -> SPARE_CKEN_W<'_, CKGATENRrs> {
        SPARE_CKEN_W::new(self, 3)
    }
    ///Bit 4 - SRQAM controller clock enable
    #[inline(always)]
    pub fn sram_cken(&mut self) -> SRAM_CKEN_W<'_, CKGATENRrs> {
        SRAM_CKEN_W::new(self, 4)
    }
    ///Bit 5 - Flash Interface clock enable
    #[inline(always)]
    pub fn flitf_cken(&mut self) -> FLITF_CKEN_W<'_, CKGATENRrs> {
        FLITF_CKEN_W::new(self, 5)
    }
    ///Bit 6 - RCC clock enable
    #[inline(always)]
    pub fn rcc_cken(&mut self) -> RCC_CKEN_W<'_, CKGATENRrs> {
        RCC_CKEN_W::new(self, 6)
    }
}
/**clocks gated enable register

You can [`read`](crate::Reg::read) this register and get [`ckgatenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgatenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F446.html#RCC:CKGATENR)*/
pub struct CKGATENRrs;
impl crate::RegisterSpec for CKGATENRrs {
    type Ux = u32;
}
///`read()` method returns [`ckgatenr::R`](R) reader structure
impl crate::Readable for CKGATENRrs {}
///`write(|w| ..)` method takes [`ckgatenr::W`](W) writer structure
impl crate::Writable for CKGATENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CKGATENR to value 0
impl crate::Resettable for CKGATENRrs {}
