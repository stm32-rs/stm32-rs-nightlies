///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
/**PREDIV1 division factor

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PREDIV1 {
    ///0: PREDIV input clock not divided
    Div1 = 0,
    ///1: PREDIV input clock divided by 2
    Div2 = 1,
    ///2: PREDIV input clock divided by 3
    Div3 = 2,
    ///3: PREDIV input clock divided by 4
    Div4 = 3,
    ///4: PREDIV input clock divided by 5
    Div5 = 4,
    ///5: PREDIV input clock divided by 6
    Div6 = 5,
    ///6: PREDIV input clock divided by 7
    Div7 = 6,
    ///7: PREDIV input clock divided by 8
    Div8 = 7,
    ///8: PREDIV input clock divided by 9
    Div9 = 8,
    ///9: PREDIV input clock divided by 10
    Div10 = 9,
    ///10: PREDIV input clock divided by 11
    Div11 = 10,
    ///11: PREDIV input clock divided by 12
    Div12 = 11,
    ///12: PREDIV input clock divided by 13
    Div13 = 12,
    ///13: PREDIV input clock divided by 14
    Div14 = 13,
    ///14: PREDIV input clock divided by 15
    Div15 = 14,
    ///15: PREDIV input clock divided by 16
    Div16 = 15,
}
impl From<PREDIV1> for u8 {
    #[inline(always)]
    fn from(variant: PREDIV1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PREDIV1 {
    type Ux = u8;
}
impl crate::IsEnum for PREDIV1 {}
///Field `PREDIV1` reader - PREDIV1 division factor
pub type PREDIV1_R = crate::FieldReader<PREDIV1>;
impl PREDIV1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PREDIV1 {
        match self.bits {
            0 => PREDIV1::Div1,
            1 => PREDIV1::Div2,
            2 => PREDIV1::Div3,
            3 => PREDIV1::Div4,
            4 => PREDIV1::Div5,
            5 => PREDIV1::Div6,
            6 => PREDIV1::Div7,
            7 => PREDIV1::Div8,
            8 => PREDIV1::Div9,
            9 => PREDIV1::Div10,
            10 => PREDIV1::Div11,
            11 => PREDIV1::Div12,
            12 => PREDIV1::Div13,
            13 => PREDIV1::Div14,
            14 => PREDIV1::Div15,
            15 => PREDIV1::Div16,
            _ => unreachable!(),
        }
    }
    ///PREDIV input clock not divided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PREDIV1::Div1
    }
    ///PREDIV input clock divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PREDIV1::Div2
    }
    ///PREDIV input clock divided by 3
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PREDIV1::Div3
    }
    ///PREDIV input clock divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PREDIV1::Div4
    }
    ///PREDIV input clock divided by 5
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PREDIV1::Div5
    }
    ///PREDIV input clock divided by 6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PREDIV1::Div6
    }
    ///PREDIV input clock divided by 7
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PREDIV1::Div7
    }
    ///PREDIV input clock divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PREDIV1::Div8
    }
    ///PREDIV input clock divided by 9
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PREDIV1::Div9
    }
    ///PREDIV input clock divided by 10
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PREDIV1::Div10
    }
    ///PREDIV input clock divided by 11
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PREDIV1::Div11
    }
    ///PREDIV input clock divided by 12
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PREDIV1::Div12
    }
    ///PREDIV input clock divided by 13
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PREDIV1::Div13
    }
    ///PREDIV input clock divided by 14
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PREDIV1::Div14
    }
    ///PREDIV input clock divided by 15
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PREDIV1::Div15
    }
    ///PREDIV input clock divided by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PREDIV1::Div16
    }
}
///Field `PREDIV1` writer - PREDIV1 division factor
pub type PREDIV1_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PREDIV1, crate::Safe>;
impl<'a, REG> PREDIV1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PREDIV input clock not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div1)
    }
    ///PREDIV input clock divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div2)
    }
    ///PREDIV input clock divided by 3
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div3)
    }
    ///PREDIV input clock divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div4)
    }
    ///PREDIV input clock divided by 5
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div5)
    }
    ///PREDIV input clock divided by 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div6)
    }
    ///PREDIV input clock divided by 7
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div7)
    }
    ///PREDIV input clock divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div8)
    }
    ///PREDIV input clock divided by 9
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div9)
    }
    ///PREDIV input clock divided by 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div10)
    }
    ///PREDIV input clock divided by 11
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div11)
    }
    ///PREDIV input clock divided by 12
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div12)
    }
    ///PREDIV input clock divided by 13
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div13)
    }
    ///PREDIV input clock divided by 14
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div14)
    }
    ///PREDIV input clock divided by 15
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div15)
    }
    ///PREDIV input clock divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1::Div16)
    }
}
///Field `PREDIV2` reader - PREDIV2 division factor
pub use PREDIV1_R as PREDIV2_R;
///Field `PREDIV2` writer - PREDIV2 division factor
pub use PREDIV1_W as PREDIV2_W;
/**PLL2 Multiplication Factor

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL2MUL {
    ///6: PLL clock entry x8
    Mul8 = 6,
    ///7: PLL clock entry x9
    Mul9 = 7,
    ///8: PLL clock entry x10
    Mul10 = 8,
    ///9: PLL clock entry x11
    Mul11 = 9,
    ///10: PLL clock entry x12
    Mul12 = 10,
    ///11: PLL clock entry x13
    Mul13 = 11,
    ///12: PLL clock entry x14
    Mul14 = 12,
    ///14: PLL clock entry x16
    Mul16 = 14,
    ///15: PLL clock entry x20
    Mul20 = 15,
}
impl From<PLL2MUL> for u8 {
    #[inline(always)]
    fn from(variant: PLL2MUL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL2MUL {
    type Ux = u8;
}
impl crate::IsEnum for PLL2MUL {}
///Field `PLL2MUL` reader - PLL2 Multiplication Factor
pub type PLL2MUL_R = crate::FieldReader<PLL2MUL>;
impl PLL2MUL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLL2MUL> {
        match self.bits {
            6 => Some(PLL2MUL::Mul8),
            7 => Some(PLL2MUL::Mul9),
            8 => Some(PLL2MUL::Mul10),
            9 => Some(PLL2MUL::Mul11),
            10 => Some(PLL2MUL::Mul12),
            11 => Some(PLL2MUL::Mul13),
            12 => Some(PLL2MUL::Mul14),
            14 => Some(PLL2MUL::Mul16),
            15 => Some(PLL2MUL::Mul20),
            _ => None,
        }
    }
    ///PLL clock entry x8
    #[inline(always)]
    pub fn is_mul8(&self) -> bool {
        *self == PLL2MUL::Mul8
    }
    ///PLL clock entry x9
    #[inline(always)]
    pub fn is_mul9(&self) -> bool {
        *self == PLL2MUL::Mul9
    }
    ///PLL clock entry x10
    #[inline(always)]
    pub fn is_mul10(&self) -> bool {
        *self == PLL2MUL::Mul10
    }
    ///PLL clock entry x11
    #[inline(always)]
    pub fn is_mul11(&self) -> bool {
        *self == PLL2MUL::Mul11
    }
    ///PLL clock entry x12
    #[inline(always)]
    pub fn is_mul12(&self) -> bool {
        *self == PLL2MUL::Mul12
    }
    ///PLL clock entry x13
    #[inline(always)]
    pub fn is_mul13(&self) -> bool {
        *self == PLL2MUL::Mul13
    }
    ///PLL clock entry x14
    #[inline(always)]
    pub fn is_mul14(&self) -> bool {
        *self == PLL2MUL::Mul14
    }
    ///PLL clock entry x16
    #[inline(always)]
    pub fn is_mul16(&self) -> bool {
        *self == PLL2MUL::Mul16
    }
    ///PLL clock entry x20
    #[inline(always)]
    pub fn is_mul20(&self) -> bool {
        *self == PLL2MUL::Mul20
    }
}
///Field `PLL2MUL` writer - PLL2 Multiplication Factor
pub type PLL2MUL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PLL2MUL>;
impl<'a, REG> PLL2MUL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLL clock entry x8
    #[inline(always)]
    pub fn mul8(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2MUL::Mul8)
    }
    ///PLL clock entry x9
    #[inline(always)]
    pub fn mul9(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2MUL::Mul9)
    }
    ///PLL clock entry x10
    #[inline(always)]
    pub fn mul10(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2MUL::Mul10)
    }
    ///PLL clock entry x11
    #[inline(always)]
    pub fn mul11(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2MUL::Mul11)
    }
    ///PLL clock entry x12
    #[inline(always)]
    pub fn mul12(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2MUL::Mul12)
    }
    ///PLL clock entry x13
    #[inline(always)]
    pub fn mul13(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2MUL::Mul13)
    }
    ///PLL clock entry x14
    #[inline(always)]
    pub fn mul14(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2MUL::Mul14)
    }
    ///PLL clock entry x16
    #[inline(always)]
    pub fn mul16(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2MUL::Mul16)
    }
    ///PLL clock entry x20
    #[inline(always)]
    pub fn mul20(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2MUL::Mul20)
    }
}
///Field `PLL3MUL` reader - PLL3 Multiplication Factor
pub use PLL2MUL_R as PLL3MUL_R;
///Field `PLL3MUL` writer - PLL3 Multiplication Factor
pub use PLL2MUL_W as PLL3MUL_W;
/**PREDIV1 entry clock source

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PREDIV1SRC {
    ///0: HSE oscillator clock selected as PREDIV1 clock entry
    Hse = 0,
    ///1: PLL2 selected as PREDIV1 clock entry
    Pll2 = 1,
}
impl From<PREDIV1SRC> for bool {
    #[inline(always)]
    fn from(variant: PREDIV1SRC) -> Self {
        variant as u8 != 0
    }
}
///Field `PREDIV1SRC` reader - PREDIV1 entry clock source
pub type PREDIV1SRC_R = crate::BitReader<PREDIV1SRC>;
impl PREDIV1SRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PREDIV1SRC {
        match self.bits {
            false => PREDIV1SRC::Hse,
            true => PREDIV1SRC::Pll2,
        }
    }
    ///HSE oscillator clock selected as PREDIV1 clock entry
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == PREDIV1SRC::Hse
    }
    ///PLL2 selected as PREDIV1 clock entry
    #[inline(always)]
    pub fn is_pll2(&self) -> bool {
        *self == PREDIV1SRC::Pll2
    }
}
///Field `PREDIV1SRC` writer - PREDIV1 entry clock source
pub type PREDIV1SRC_W<'a, REG> = crate::BitWriter<'a, REG, PREDIV1SRC>;
impl<'a, REG> PREDIV1SRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSE oscillator clock selected as PREDIV1 clock entry
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1SRC::Hse)
    }
    ///PLL2 selected as PREDIV1 clock entry
    #[inline(always)]
    pub fn pll2(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV1SRC::Pll2)
    }
}
/**I2S2 clock source

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2S2SRC {
    ///0: System clock (SYSCLK) selected as I2S clock entry
    Sysclk = 0,
    ///1: PLL3 VCO clock selected as I2S clock entry
    Pll3 = 1,
}
impl From<I2S2SRC> for bool {
    #[inline(always)]
    fn from(variant: I2S2SRC) -> Self {
        variant as u8 != 0
    }
}
///Field `I2S2SRC` reader - I2S2 clock source
pub type I2S2SRC_R = crate::BitReader<I2S2SRC>;
impl I2S2SRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2S2SRC {
        match self.bits {
            false => I2S2SRC::Sysclk,
            true => I2S2SRC::Pll3,
        }
    }
    ///System clock (SYSCLK) selected as I2S clock entry
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == I2S2SRC::Sysclk
    }
    ///PLL3 VCO clock selected as I2S clock entry
    #[inline(always)]
    pub fn is_pll3(&self) -> bool {
        *self == I2S2SRC::Pll3
    }
}
///Field `I2S2SRC` writer - I2S2 clock source
pub type I2S2SRC_W<'a, REG> = crate::BitWriter<'a, REG, I2S2SRC>;
impl<'a, REG> I2S2SRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///System clock (SYSCLK) selected as I2S clock entry
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(I2S2SRC::Sysclk)
    }
    ///PLL3 VCO clock selected as I2S clock entry
    #[inline(always)]
    pub fn pll3(self) -> &'a mut crate::W<REG> {
        self.variant(I2S2SRC::Pll3)
    }
}
///Field `I2S3SRC` reader - I2S3 clock source
pub use I2S2SRC_R as I2S3SRC_R;
///Field `I2S3SRC` writer - I2S3 clock source
pub use I2S2SRC_W as I2S3SRC_W;
impl R {
    ///Bits 0:3 - PREDIV1 division factor
    #[inline(always)]
    pub fn prediv1(&self) -> PREDIV1_R {
        PREDIV1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - PREDIV2 division factor
    #[inline(always)]
    pub fn prediv2(&self) -> PREDIV2_R {
        PREDIV2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - PLL2 Multiplication Factor
    #[inline(always)]
    pub fn pll2mul(&self) -> PLL2MUL_R {
        PLL2MUL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - PLL3 Multiplication Factor
    #[inline(always)]
    pub fn pll3mul(&self) -> PLL3MUL_R {
        PLL3MUL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bit 16 - PREDIV1 entry clock source
    #[inline(always)]
    pub fn prediv1src(&self) -> PREDIV1SRC_R {
        PREDIV1SRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - I2S2 clock source
    #[inline(always)]
    pub fn i2s2src(&self) -> I2S2SRC_R {
        I2S2SRC_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - I2S3 clock source
    #[inline(always)]
    pub fn i2s3src(&self) -> I2S3SRC_R {
        I2S3SRC_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("prediv1", &self.prediv1())
            .field("prediv2", &self.prediv2())
            .field("pll2mul", &self.pll2mul())
            .field("pll3mul", &self.pll3mul())
            .field("prediv1src", &self.prediv1src())
            .field("i2s2src", &self.i2s2src())
            .field("i2s3src", &self.i2s3src())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - PREDIV1 division factor
    #[inline(always)]
    pub fn prediv1(&mut self) -> PREDIV1_W<'_, CFGR2rs> {
        PREDIV1_W::new(self, 0)
    }
    ///Bits 4:7 - PREDIV2 division factor
    #[inline(always)]
    pub fn prediv2(&mut self) -> PREDIV2_W<'_, CFGR2rs> {
        PREDIV2_W::new(self, 4)
    }
    ///Bits 8:11 - PLL2 Multiplication Factor
    #[inline(always)]
    pub fn pll2mul(&mut self) -> PLL2MUL_W<'_, CFGR2rs> {
        PLL2MUL_W::new(self, 8)
    }
    ///Bits 12:15 - PLL3 Multiplication Factor
    #[inline(always)]
    pub fn pll3mul(&mut self) -> PLL3MUL_W<'_, CFGR2rs> {
        PLL3MUL_W::new(self, 12)
    }
    ///Bit 16 - PREDIV1 entry clock source
    #[inline(always)]
    pub fn prediv1src(&mut self) -> PREDIV1SRC_W<'_, CFGR2rs> {
        PREDIV1SRC_W::new(self, 16)
    }
    ///Bit 17 - I2S2 clock source
    #[inline(always)]
    pub fn i2s2src(&mut self) -> I2S2SRC_W<'_, CFGR2rs> {
        I2S2SRC_W::new(self, 17)
    }
    ///Bit 18 - I2S3 clock source
    #[inline(always)]
    pub fn i2s3src(&mut self) -> I2S3SRC_W<'_, CFGR2rs> {
        I2S3SRC_W::new(self, 18)
    }
}
/**Clock configuration register2 (RCC_CFGR2)

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#RCC:CFGR2)*/
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
