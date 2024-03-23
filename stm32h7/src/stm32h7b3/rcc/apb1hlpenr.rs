#[doc = "Register `APB1HLPENR` reader"]
pub type R = crate::R<APB1HLPENRrs>;
#[doc = "Register `APB1HLPENR` writer"]
pub type W = crate::W<APB1HLPENRrs>;
#[doc = "clock recovery system peripheral clock enable during CSleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSLPEN {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    Enabled = 1,
}
impl From<CRSLPEN> for bool {
    #[inline(always)]
    fn from(variant: CRSLPEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRSLPEN` reader - clock recovery system peripheral clock enable during CSleep mode Set and reset by software."]
pub type CRSLPEN_R = crate::BitReader<CRSLPEN>;
impl CRSLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRSLPEN {
        match self.bits {
            false => CRSLPEN::Disabled,
            true => CRSLPEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRSLPEN::Disabled
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRSLPEN::Enabled
    }
}
#[doc = "Field `CRSLPEN` writer - clock recovery system peripheral clock enable during CSleep mode Set and reset by software."]
pub type CRSLPEN_W<'a, REG> = crate::BitWriter<'a, REG, CRSLPEN>;
impl<'a, REG> CRSLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRSLPEN::Disabled)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRSLPEN::Enabled)
    }
}
#[doc = "Field `SWPMILPEN` reader - SWPMI peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SWPMI are the kernel clock selected by SWPMISEL and provided to swpmi_ker_ck input, and the rcc_pclk1 bus interface clock."]
pub use CRSLPEN_R as SWPMILPEN_R;
#[doc = "Field `OPAMPLPEN` reader - OPAMP peripheral clock enable during CSleep mode Set and reset by software."]
pub use CRSLPEN_R as OPAMPLPEN_R;
#[doc = "Field `MDIOSLPEN` reader - MDIOS peripheral clock enable during CSleep mode Set and reset by software."]
pub use CRSLPEN_R as MDIOSLPEN_R;
#[doc = "Field `FDCANLPEN` reader - FDCAN peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the FDCAN are: the kernel clock selected by FDCANSEL and provided to fdcan_clk input, and the rcc_pclk1 bus interface clock."]
pub use CRSLPEN_R as FDCANLPEN_R;
#[doc = "Field `SWPMILPEN` writer - SWPMI peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SWPMI are the kernel clock selected by SWPMISEL and provided to swpmi_ker_ck input, and the rcc_pclk1 bus interface clock."]
pub use CRSLPEN_W as SWPMILPEN_W;
#[doc = "Field `OPAMPLPEN` writer - OPAMP peripheral clock enable during CSleep mode Set and reset by software."]
pub use CRSLPEN_W as OPAMPLPEN_W;
#[doc = "Field `MDIOSLPEN` writer - MDIOS peripheral clock enable during CSleep mode Set and reset by software."]
pub use CRSLPEN_W as MDIOSLPEN_W;
#[doc = "Field `FDCANLPEN` writer - FDCAN peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the FDCAN are: the kernel clock selected by FDCANSEL and provided to fdcan_clk input, and the rcc_pclk1 bus interface clock."]
pub use CRSLPEN_W as FDCANLPEN_W;
impl R {
    #[doc = "Bit 1 - clock recovery system peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn crslpen(&self) -> CRSLPEN_R {
        CRSLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SWPMI peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SWPMI are the kernel clock selected by SWPMISEL and provided to swpmi_ker_ck input, and the rcc_pclk1 bus interface clock."]
    #[inline(always)]
    pub fn swpmilpen(&self) -> SWPMILPEN_R {
        SWPMILPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - OPAMP peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn opamplpen(&self) -> OPAMPLPEN_R {
        OPAMPLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MDIOS peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn mdioslpen(&self) -> MDIOSLPEN_R {
        MDIOSLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - FDCAN peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the FDCAN are: the kernel clock selected by FDCANSEL and provided to fdcan_clk input, and the rcc_pclk1 bus interface clock."]
    #[inline(always)]
    pub fn fdcanlpen(&self) -> FDCANLPEN_R {
        FDCANLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - clock recovery system peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn crslpen(&mut self) -> CRSLPEN_W<APB1HLPENRrs> {
        CRSLPEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - SWPMI peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SWPMI are the kernel clock selected by SWPMISEL and provided to swpmi_ker_ck input, and the rcc_pclk1 bus interface clock."]
    #[inline(always)]
    #[must_use]
    pub fn swpmilpen(&mut self) -> SWPMILPEN_W<APB1HLPENRrs> {
        SWPMILPEN_W::new(self, 2)
    }
    #[doc = "Bit 4 - OPAMP peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn opamplpen(&mut self) -> OPAMPLPEN_W<APB1HLPENRrs> {
        OPAMPLPEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - MDIOS peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn mdioslpen(&mut self) -> MDIOSLPEN_W<APB1HLPENRrs> {
        MDIOSLPEN_W::new(self, 5)
    }
    #[doc = "Bit 8 - FDCAN peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the FDCAN are: the kernel clock selected by FDCANSEL and provided to fdcan_clk input, and the rcc_pclk1 bus interface clock."]
    #[inline(always)]
    #[must_use]
    pub fn fdcanlpen(&mut self) -> FDCANLPEN_W<APB1HLPENRrs> {
        FDCANLPEN_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1hlpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1hlpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1HLPENRrs;
impl crate::RegisterSpec for APB1HLPENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1hlpenr::R`](R) reader structure"]
impl crate::Readable for APB1HLPENRrs {}
#[doc = "`write(|w| ..)` method takes [`apb1hlpenr::W`](W) writer structure"]
impl crate::Writable for APB1HLPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1HLPENR to value 0x0136"]
impl crate::Resettable for APB1HLPENRrs {
    const RESET_VALUE: u32 = 0x0136;
}
