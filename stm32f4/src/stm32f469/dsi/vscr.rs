///Register `VSCR` reader
pub type R = crate::R<VSCRrs>;
///Register `VSCR` writer
pub type W = crate::W<VSCRrs>;
/**Enable When set to 1, DSI Host LTDC interface receives the active configuration from the auxiliary registers. When this bit is set along with the UR bit, the auxiliary registers are automatically updated.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN {
    ///0: Register update is disabled.
    B0x0 = 0,
    ///1: Register update is enabled.
    B0x1 = 1,
}
impl From<EN> for bool {
    #[inline(always)]
    fn from(variant: EN) -> Self {
        variant as u8 != 0
    }
}
///Field `EN` reader - Enable When set to 1, DSI Host LTDC interface receives the active configuration from the auxiliary registers. When this bit is set along with the UR bit, the auxiliary registers are automatically updated.
pub type EN_R = crate::BitReader<EN>;
impl EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EN {
        match self.bits {
            false => EN::B0x0,
            true => EN::B0x1,
        }
    }
    ///Register update is disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EN::B0x0
    }
    ///Register update is enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EN::B0x1
    }
}
///Field `EN` writer - Enable When set to 1, DSI Host LTDC interface receives the active configuration from the auxiliary registers. When this bit is set along with the UR bit, the auxiliary registers are automatically updated.
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Register update is disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EN::B0x0)
    }
    ///Register update is enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EN::B0x1)
    }
}
/**Update register When set to 1, the LTDC registers are copied to the auxiliary registers. After copying, this bit is auto cleared.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UR {
    ///0: No update requested
    B0x0 = 0,
    ///1: Register update requested
    B0x1 = 1,
}
impl From<UR> for bool {
    #[inline(always)]
    fn from(variant: UR) -> Self {
        variant as u8 != 0
    }
}
///Field `UR` reader - Update register When set to 1, the LTDC registers are copied to the auxiliary registers. After copying, this bit is auto cleared.
pub type UR_R = crate::BitReader<UR>;
impl UR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UR {
        match self.bits {
            false => UR::B0x0,
            true => UR::B0x1,
        }
    }
    ///No update requested
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UR::B0x0
    }
    ///Register update requested
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UR::B0x1
    }
}
///Field `UR` writer - Update register When set to 1, the LTDC registers are copied to the auxiliary registers. After copying, this bit is auto cleared.
pub type UR_W<'a, REG> = crate::BitWriter<'a, REG, UR>;
impl<'a, REG> UR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No update requested
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UR::B0x0)
    }
    ///Register update requested
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UR::B0x1)
    }
}
impl R {
    ///Bit 0 - Enable When set to 1, DSI Host LTDC interface receives the active configuration from the auxiliary registers. When this bit is set along with the UR bit, the auxiliary registers are automatically updated.
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Update register When set to 1, the LTDC registers are copied to the auxiliary registers. After copying, this bit is auto cleared.
    #[inline(always)]
    pub fn ur(&self) -> UR_R {
        UR_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VSCR")
            .field("en", &self.en())
            .field("ur", &self.ur())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable When set to 1, DSI Host LTDC interface receives the active configuration from the auxiliary registers. When this bit is set along with the UR bit, the auxiliary registers are automatically updated.
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<VSCRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 8 - Update register When set to 1, the LTDC registers are copied to the auxiliary registers. After copying, this bit is auto cleared.
    #[inline(always)]
    pub fn ur(&mut self) -> UR_W<VSCRrs> {
        UR_W::new(self, 8)
    }
}
/**DSI Host video shadow control register

You can [`read`](crate::Reg::read) this register and get [`vscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:VSCR)*/
pub struct VSCRrs;
impl crate::RegisterSpec for VSCRrs {
    type Ux = u32;
}
///`read()` method returns [`vscr::R`](R) reader structure
impl crate::Readable for VSCRrs {}
///`write(|w| ..)` method takes [`vscr::W`](W) writer structure
impl crate::Writable for VSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VSCR to value 0
impl crate::Resettable for VSCRrs {}
