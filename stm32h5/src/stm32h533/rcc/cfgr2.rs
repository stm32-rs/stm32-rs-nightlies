///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
/**AHB prescaler

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HPRE {
    ///8: SYSCLK divided by 2
    Div2 = 8,
    ///9: SYSCLK divided by 4
    Div4 = 9,
    ///10: SYSCLK divided by 8
    Div8 = 10,
    ///11: SYSCLK divided by 16
    Div16 = 11,
    ///12: SYSCLK divided by 64
    Div64 = 12,
    ///13: SYSCLK divided by 128
    Div128 = 13,
    ///14: SYSCLK divided by 256
    Div256 = 14,
    ///15: SYSCLK divided by 512
    Div512 = 15,
    ///0: SYSCLK not divided
    Div1 = 0,
}
impl From<HPRE> for u8 {
    #[inline(always)]
    fn from(variant: HPRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HPRE {
    type Ux = u8;
}
impl crate::IsEnum for HPRE {}
///Field `HPRE` reader - AHB prescaler
pub type HPRE_R = crate::FieldReader<HPRE>;
impl HPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HPRE {
        match self.bits {
            8 => HPRE::Div2,
            9 => HPRE::Div4,
            10 => HPRE::Div8,
            11 => HPRE::Div16,
            12 => HPRE::Div64,
            13 => HPRE::Div128,
            14 => HPRE::Div256,
            15 => HPRE::Div512,
            _ => HPRE::Div1,
        }
    }
    ///SYSCLK divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HPRE::Div2
    }
    ///SYSCLK divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HPRE::Div4
    }
    ///SYSCLK divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HPRE::Div8
    }
    ///SYSCLK divided by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == HPRE::Div16
    }
    ///SYSCLK divided by 64
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == HPRE::Div64
    }
    ///SYSCLK divided by 128
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == HPRE::Div128
    }
    ///SYSCLK divided by 256
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == HPRE::Div256
    }
    ///SYSCLK divided by 512
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == HPRE::Div512
    }
    ///SYSCLK not divided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        matches!(self.variant(), HPRE::Div1)
    }
}
///Field `HPRE` writer - AHB prescaler
pub type HPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, HPRE, crate::Safe>;
impl<'a, REG> HPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SYSCLK divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div2)
    }
    ///SYSCLK divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div4)
    }
    ///SYSCLK divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div8)
    }
    ///SYSCLK divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div16)
    }
    ///SYSCLK divided by 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div64)
    }
    ///SYSCLK divided by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div128)
    }
    ///SYSCLK divided by 256
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div256)
    }
    ///SYSCLK divided by 512
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div512)
    }
    ///SYSCLK not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div1)
    }
}
/**APB low-speed prescaler (APB1)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PPRE1 {
    ///4: HCLK divided by 2
    Div2 = 4,
    ///5: HCLK divided by 4
    Div4 = 5,
    ///6: HCLK divided by 8
    Div8 = 6,
    ///7: HCLK divided by 16
    Div16 = 7,
    ///0: HCLK not divided
    Div1 = 0,
}
impl From<PPRE1> for u8 {
    #[inline(always)]
    fn from(variant: PPRE1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PPRE1 {
    type Ux = u8;
}
impl crate::IsEnum for PPRE1 {}
///Field `PPRE1` reader - APB low-speed prescaler (APB1)
pub type PPRE1_R = crate::FieldReader<PPRE1>;
impl PPRE1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PPRE1 {
        match self.bits {
            4 => PPRE1::Div2,
            5 => PPRE1::Div4,
            6 => PPRE1::Div8,
            7 => PPRE1::Div16,
            _ => PPRE1::Div1,
        }
    }
    ///HCLK divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PPRE1::Div2
    }
    ///HCLK divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PPRE1::Div4
    }
    ///HCLK divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PPRE1::Div8
    }
    ///HCLK divided by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PPRE1::Div16
    }
    ///HCLK not divided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        matches!(self.variant(), PPRE1::Div1)
    }
}
///Field `PPRE1` writer - APB low-speed prescaler (APB1)
pub type PPRE1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PPRE1, crate::Safe>;
impl<'a, REG> PPRE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HCLK divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1::Div2)
    }
    ///HCLK divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1::Div4)
    }
    ///HCLK divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1::Div8)
    }
    ///HCLK divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1::Div16)
    }
    ///HCLK not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE1::Div1)
    }
}
///Field `PPRE2` reader - APB high-speed prescaler (APB2)
pub use PPRE1_R as PPRE2_R;
///Field `PPRE3` reader - APB low-speed prescaler (APB3)
pub use PPRE1_R as PPRE3_R;
///Field `PPRE2` writer - APB high-speed prescaler (APB2)
pub use PPRE1_W as PPRE2_W;
///Field `PPRE3` writer - APB low-speed prescaler (APB3)
pub use PPRE1_W as PPRE3_W;
/**AHB1 clock disable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHB1DIS {
    ///0: The selected clock is enabled
    Enabled = 0,
    ///1: The selected clock is disabled
    Disabled = 1,
}
impl From<AHB1DIS> for bool {
    #[inline(always)]
    fn from(variant: AHB1DIS) -> Self {
        variant as u8 != 0
    }
}
///Field `AHB1DIS` reader - AHB1 clock disable
pub type AHB1DIS_R = crate::BitReader<AHB1DIS>;
impl AHB1DIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AHB1DIS {
        match self.bits {
            false => AHB1DIS::Enabled,
            true => AHB1DIS::Disabled,
        }
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AHB1DIS::Enabled
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AHB1DIS::Disabled
    }
}
///Field `AHB1DIS` writer - AHB1 clock disable
pub type AHB1DIS_W<'a, REG> = crate::BitWriter<'a, REG, AHB1DIS>;
impl<'a, REG> AHB1DIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AHB1DIS::Enabled)
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AHB1DIS::Disabled)
    }
}
///Field `AHB2DIS` reader - AHB2 clock disable
pub use AHB1DIS_R as AHB2DIS_R;
///Field `AHB4DIS` reader - AHB4 clock disable
pub use AHB1DIS_R as AHB4DIS_R;
///Field `APB1DIS` reader - APB1 clock disable value
pub use AHB1DIS_R as APB1DIS_R;
///Field `APB2DIS` reader - APB2 clock disable value
pub use AHB1DIS_R as APB2DIS_R;
///Field `APB3DIS` reader - APB3 clock disable value.
pub use AHB1DIS_R as APB3DIS_R;
///Field `AHB2DIS` writer - AHB2 clock disable
pub use AHB1DIS_W as AHB2DIS_W;
///Field `AHB4DIS` writer - AHB4 clock disable
pub use AHB1DIS_W as AHB4DIS_W;
///Field `APB1DIS` writer - APB1 clock disable value
pub use AHB1DIS_W as APB1DIS_W;
///Field `APB2DIS` writer - APB2 clock disable value
pub use AHB1DIS_W as APB2DIS_W;
///Field `APB3DIS` writer - APB3 clock disable value.
pub use AHB1DIS_W as APB3DIS_W;
impl R {
    ///Bits 0:3 - AHB prescaler
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:6 - APB low-speed prescaler (APB1)
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - APB high-speed prescaler (APB2)
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - APB low-speed prescaler (APB3)
    #[inline(always)]
    pub fn ppre3(&self) -> PPRE3_R {
        PPRE3_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 16 - AHB1 clock disable
    #[inline(always)]
    pub fn ahb1dis(&self) -> AHB1DIS_R {
        AHB1DIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - AHB2 clock disable
    #[inline(always)]
    pub fn ahb2dis(&self) -> AHB2DIS_R {
        AHB2DIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - AHB4 clock disable
    #[inline(always)]
    pub fn ahb4dis(&self) -> AHB4DIS_R {
        AHB4DIS_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - APB1 clock disable value
    #[inline(always)]
    pub fn apb1dis(&self) -> APB1DIS_R {
        APB1DIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - APB2 clock disable value
    #[inline(always)]
    pub fn apb2dis(&self) -> APB2DIS_R {
        APB2DIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - APB3 clock disable value.
    #[inline(always)]
    pub fn apb3dis(&self) -> APB3DIS_R {
        APB3DIS_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("hpre", &self.hpre())
            .field("ppre1", &self.ppre1())
            .field("ppre2", &self.ppre2())
            .field("ppre3", &self.ppre3())
            .field("ahb1dis", &self.ahb1dis())
            .field("ahb2dis", &self.ahb2dis())
            .field("ahb4dis", &self.ahb4dis())
            .field("apb1dis", &self.apb1dis())
            .field("apb2dis", &self.apb2dis())
            .field("apb3dis", &self.apb3dis())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - AHB prescaler
    #[inline(always)]
    pub fn hpre(&mut self) -> HPRE_W<'_, CFGR2rs> {
        HPRE_W::new(self, 0)
    }
    ///Bits 4:6 - APB low-speed prescaler (APB1)
    #[inline(always)]
    pub fn ppre1(&mut self) -> PPRE1_W<'_, CFGR2rs> {
        PPRE1_W::new(self, 4)
    }
    ///Bits 8:10 - APB high-speed prescaler (APB2)
    #[inline(always)]
    pub fn ppre2(&mut self) -> PPRE2_W<'_, CFGR2rs> {
        PPRE2_W::new(self, 8)
    }
    ///Bits 12:14 - APB low-speed prescaler (APB3)
    #[inline(always)]
    pub fn ppre3(&mut self) -> PPRE3_W<'_, CFGR2rs> {
        PPRE3_W::new(self, 12)
    }
    ///Bit 16 - AHB1 clock disable
    #[inline(always)]
    pub fn ahb1dis(&mut self) -> AHB1DIS_W<'_, CFGR2rs> {
        AHB1DIS_W::new(self, 16)
    }
    ///Bit 17 - AHB2 clock disable
    #[inline(always)]
    pub fn ahb2dis(&mut self) -> AHB2DIS_W<'_, CFGR2rs> {
        AHB2DIS_W::new(self, 17)
    }
    ///Bit 19 - AHB4 clock disable
    #[inline(always)]
    pub fn ahb4dis(&mut self) -> AHB4DIS_W<'_, CFGR2rs> {
        AHB4DIS_W::new(self, 19)
    }
    ///Bit 20 - APB1 clock disable value
    #[inline(always)]
    pub fn apb1dis(&mut self) -> APB1DIS_W<'_, CFGR2rs> {
        APB1DIS_W::new(self, 20)
    }
    ///Bit 21 - APB2 clock disable value
    #[inline(always)]
    pub fn apb2dis(&mut self) -> APB2DIS_W<'_, CFGR2rs> {
        APB2DIS_W::new(self, 21)
    }
    ///Bit 22 - APB3 clock disable value.
    #[inline(always)]
    pub fn apb3dis(&mut self) -> APB3DIS_W<'_, CFGR2rs> {
        APB3DIS_W::new(self, 22)
    }
}
/**RCC CPU domain clock configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#RCC:CFGR2)*/
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr2::R`](R) reader structure
impl crate::Readable for CFGR2rs {}
///`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2rs {}
