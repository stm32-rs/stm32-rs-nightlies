///Register `APB1HLPENR` reader
pub type R = crate::R<APB1HLPENRrs>;
///Register `APB1HLPENR` writer
pub type W = crate::W<APB1HLPENRrs>;
/**clock recovery system peripheral clock enable during CSleep mode Set and reset by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSLPEN {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<CRSLPEN> for bool {
    #[inline(always)]
    fn from(variant: CRSLPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CRSLPEN` reader - clock recovery system peripheral clock enable during CSleep mode Set and reset by software.
pub type CRSLPEN_R = crate::BitReader<CRSLPEN>;
impl CRSLPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRSLPEN {
        match self.bits {
            false => CRSLPEN::Disabled,
            true => CRSLPEN::Enabled,
        }
    }
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRSLPEN::Disabled
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRSLPEN::Enabled
    }
}
///Field `CRSLPEN` writer - clock recovery system peripheral clock enable during CSleep mode Set and reset by software.
pub type CRSLPEN_W<'a, REG> = crate::BitWriter<'a, REG, CRSLPEN>;
impl<'a, REG> CRSLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRSLPEN::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRSLPEN::Enabled)
    }
}
///Field `SWPMILPEN` reader - SWPMI peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SWPMI are the kernel clock selected by SWPMISEL and provided to swpmi_ker_ck input, and the rcc_pclk1 bus interface clock.
pub use CRSLPEN_R as SWPMILPEN_R;
///Field `OPAMPLPEN` reader - OPAMP peripheral clock enable during CSleep mode Set and reset by software.
pub use CRSLPEN_R as OPAMPLPEN_R;
///Field `MDIOSLPEN` reader - MDIOS peripheral clock enable during CSleep mode Set and reset by software.
pub use CRSLPEN_R as MDIOSLPEN_R;
///Field `FDCANLPEN` reader - FDCAN peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the FDCAN are: the kernel clock selected by FDCANSEL and provided to fdcan_clk input, and the rcc_pclk1 bus interface clock.
pub use CRSLPEN_R as FDCANLPEN_R;
///Field `SWPMILPEN` writer - SWPMI peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SWPMI are the kernel clock selected by SWPMISEL and provided to swpmi_ker_ck input, and the rcc_pclk1 bus interface clock.
pub use CRSLPEN_W as SWPMILPEN_W;
///Field `OPAMPLPEN` writer - OPAMP peripheral clock enable during CSleep mode Set and reset by software.
pub use CRSLPEN_W as OPAMPLPEN_W;
///Field `MDIOSLPEN` writer - MDIOS peripheral clock enable during CSleep mode Set and reset by software.
pub use CRSLPEN_W as MDIOSLPEN_W;
///Field `FDCANLPEN` writer - FDCAN peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the FDCAN are: the kernel clock selected by FDCANSEL and provided to fdcan_clk input, and the rcc_pclk1 bus interface clock.
pub use CRSLPEN_W as FDCANLPEN_W;
impl R {
    ///Bit 1 - clock recovery system peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn crslpen(&self) -> CRSLPEN_R {
        CRSLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SWPMI peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SWPMI are the kernel clock selected by SWPMISEL and provided to swpmi_ker_ck input, and the rcc_pclk1 bus interface clock.
    #[inline(always)]
    pub fn swpmilpen(&self) -> SWPMILPEN_R {
        SWPMILPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - OPAMP peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn opamplpen(&self) -> OPAMPLPEN_R {
        OPAMPLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MDIOS peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn mdioslpen(&self) -> MDIOSLPEN_R {
        MDIOSLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - FDCAN peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the FDCAN are: the kernel clock selected by FDCANSEL and provided to fdcan_clk input, and the rcc_pclk1 bus interface clock.
    #[inline(always)]
    pub fn fdcanlpen(&self) -> FDCANLPEN_R {
        FDCANLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1HLPENR")
            .field("crslpen", &self.crslpen())
            .field("swpmilpen", &self.swpmilpen())
            .field("opamplpen", &self.opamplpen())
            .field("mdioslpen", &self.mdioslpen())
            .field("fdcanlpen", &self.fdcanlpen())
            .finish()
    }
}
impl W {
    ///Bit 1 - clock recovery system peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn crslpen(&mut self) -> CRSLPEN_W<'_, APB1HLPENRrs> {
        CRSLPEN_W::new(self, 1)
    }
    ///Bit 2 - SWPMI peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SWPMI are the kernel clock selected by SWPMISEL and provided to swpmi_ker_ck input, and the rcc_pclk1 bus interface clock.
    #[inline(always)]
    pub fn swpmilpen(&mut self) -> SWPMILPEN_W<'_, APB1HLPENRrs> {
        SWPMILPEN_W::new(self, 2)
    }
    ///Bit 4 - OPAMP peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn opamplpen(&mut self) -> OPAMPLPEN_W<'_, APB1HLPENRrs> {
        OPAMPLPEN_W::new(self, 4)
    }
    ///Bit 5 - MDIOS peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn mdioslpen(&mut self) -> MDIOSLPEN_W<'_, APB1HLPENRrs> {
        MDIOSLPEN_W::new(self, 5)
    }
    ///Bit 8 - FDCAN peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the FDCAN are: the kernel clock selected by FDCANSEL and provided to fdcan_clk input, and the rcc_pclk1 bus interface clock.
    #[inline(always)]
    pub fn fdcanlpen(&mut self) -> FDCANLPEN_W<'_, APB1HLPENRrs> {
        FDCANLPEN_W::new(self, 8)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`apb1hlpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hlpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#RCC:APB1HLPENR)*/
pub struct APB1HLPENRrs;
impl crate::RegisterSpec for APB1HLPENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1hlpenr::R`](R) reader structure
impl crate::Readable for APB1HLPENRrs {}
///`write(|w| ..)` method takes [`apb1hlpenr::W`](W) writer structure
impl crate::Writable for APB1HLPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1HLPENR to value 0x0136
impl crate::Resettable for APB1HLPENRrs {
    const RESET_VALUE: u32 = 0x0136;
}
