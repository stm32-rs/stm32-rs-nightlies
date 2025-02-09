///Register `LCOLCR` reader
pub type R = crate::R<LCOLCRrs>;
///Register `LCOLCR` writer
pub type W = crate::W<LCOLCRrs>;
/**Color coding This field configures the DPI color coding. Others: Reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COLC {
    ///0: 16-bit configuration 1
    B0x0 = 0,
    ///1: 16-bit configuration 2
    B0x1 = 1,
    ///2: 16-bit configuration 3
    B0x2 = 2,
    ///3: 18-bit configuration 1
    B0x3 = 3,
    ///4: 18-bit configuration 2
    B0x4 = 4,
    ///5: 24-bit
    B0x5 = 5,
}
impl From<COLC> for u8 {
    #[inline(always)]
    fn from(variant: COLC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COLC {
    type Ux = u8;
}
impl crate::IsEnum for COLC {}
///Field `COLC` reader - Color coding This field configures the DPI color coding. Others: Reserved
pub type COLC_R = crate::FieldReader<COLC>;
impl COLC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<COLC> {
        match self.bits {
            0 => Some(COLC::B0x0),
            1 => Some(COLC::B0x1),
            2 => Some(COLC::B0x2),
            3 => Some(COLC::B0x3),
            4 => Some(COLC::B0x4),
            5 => Some(COLC::B0x5),
            _ => None,
        }
    }
    ///16-bit configuration 1
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == COLC::B0x0
    }
    ///16-bit configuration 2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == COLC::B0x1
    }
    ///16-bit configuration 3
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == COLC::B0x2
    }
    ///18-bit configuration 1
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == COLC::B0x3
    }
    ///18-bit configuration 2
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == COLC::B0x4
    }
    ///24-bit
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == COLC::B0x5
    }
}
///Field `COLC` writer - Color coding This field configures the DPI color coding. Others: Reserved
pub type COLC_W<'a, REG> = crate::FieldWriter<'a, REG, 4, COLC>;
impl<'a, REG> COLC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///16-bit configuration 1
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(COLC::B0x0)
    }
    ///16-bit configuration 2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(COLC::B0x1)
    }
    ///16-bit configuration 3
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(COLC::B0x2)
    }
    ///18-bit configuration 1
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(COLC::B0x3)
    }
    ///18-bit configuration 2
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(COLC::B0x4)
    }
    ///24-bit
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(COLC::B0x5)
    }
}
/**Loosely packet enable This bit enables the loosely packed variant to 18-bit configuration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPE {
    ///0: Loosely packet variant disabled
    B0x0 = 0,
    ///1: Loosely packet variant enabled
    B0x1 = 1,
}
impl From<LPE> for bool {
    #[inline(always)]
    fn from(variant: LPE) -> Self {
        variant as u8 != 0
    }
}
///Field `LPE` reader - Loosely packet enable This bit enables the loosely packed variant to 18-bit configuration
pub type LPE_R = crate::BitReader<LPE>;
impl LPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPE {
        match self.bits {
            false => LPE::B0x0,
            true => LPE::B0x1,
        }
    }
    ///Loosely packet variant disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPE::B0x0
    }
    ///Loosely packet variant enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPE::B0x1
    }
}
///Field `LPE` writer - Loosely packet enable This bit enables the loosely packed variant to 18-bit configuration
pub type LPE_W<'a, REG> = crate::BitWriter<'a, REG, LPE>;
impl<'a, REG> LPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Loosely packet variant disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPE::B0x0)
    }
    ///Loosely packet variant enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPE::B0x1)
    }
}
impl R {
    ///Bits 0:3 - Color coding This field configures the DPI color coding. Others: Reserved
    #[inline(always)]
    pub fn colc(&self) -> COLC_R {
        COLC_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 8 - Loosely packet enable This bit enables the loosely packed variant to 18-bit configuration
    #[inline(always)]
    pub fn lpe(&self) -> LPE_R {
        LPE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCOLCR")
            .field("colc", &self.colc())
            .field("lpe", &self.lpe())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Color coding This field configures the DPI color coding. Others: Reserved
    #[inline(always)]
    pub fn colc(&mut self) -> COLC_W<LCOLCRrs> {
        COLC_W::new(self, 0)
    }
    ///Bit 8 - Loosely packet enable This bit enables the loosely packed variant to 18-bit configuration
    #[inline(always)]
    pub fn lpe(&mut self) -> LPE_W<LCOLCRrs> {
        LPE_W::new(self, 8)
    }
}
/**DSI Host LTDC color coding register

You can [`read`](crate::Reg::read) this register and get [`lcolcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcolcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:LCOLCR)*/
pub struct LCOLCRrs;
impl crate::RegisterSpec for LCOLCRrs {
    type Ux = u32;
}
///`read()` method returns [`lcolcr::R`](R) reader structure
impl crate::Readable for LCOLCRrs {}
///`write(|w| ..)` method takes [`lcolcr::W`](W) writer structure
impl crate::Writable for LCOLCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LCOLCR to value 0
impl crate::Resettable for LCOLCRrs {
    const RESET_VALUE: u32 = 0;
}
