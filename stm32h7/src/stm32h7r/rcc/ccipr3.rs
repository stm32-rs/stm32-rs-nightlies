///Register `CCIPR3` reader
pub type R = crate::R<CCIPR3rs>;
///Register `CCIPR3` writer
pub type W = crate::W<CCIPR3rs>;
/**USART1 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART1SEL {
    ///0: pclk2 selected as clock
    Pclk2 = 0,
    ///1: pll2_q selected as clock
    Pll2Q = 1,
    ///2: pll3_q selected as clock
    Pll3Q = 2,
    ///3: hsi_ker selected as clock
    HsiKer = 3,
    ///4: csi_ker selected as clock
    CsiKer = 4,
    ///5: lse selected as clock
    Lse = 5,
}
impl From<USART1SEL> for u8 {
    #[inline(always)]
    fn from(variant: USART1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART1SEL {
    type Ux = u8;
}
impl crate::IsEnum for USART1SEL {}
///Field `USART1SEL` reader - USART1 kernel clock source selection
pub type USART1SEL_R = crate::FieldReader<USART1SEL>;
impl USART1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<USART1SEL> {
        match self.bits {
            0 => Some(USART1SEL::Pclk2),
            1 => Some(USART1SEL::Pll2Q),
            2 => Some(USART1SEL::Pll3Q),
            3 => Some(USART1SEL::HsiKer),
            4 => Some(USART1SEL::CsiKer),
            5 => Some(USART1SEL::Lse),
            _ => None,
        }
    }
    ///pclk2 selected as clock
    #[inline(always)]
    pub fn is_pclk2(&self) -> bool {
        *self == USART1SEL::Pclk2
    }
    ///pll2_q selected as clock
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == USART1SEL::Pll2Q
    }
    ///pll3_q selected as clock
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == USART1SEL::Pll3Q
    }
    ///hsi_ker selected as clock
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == USART1SEL::HsiKer
    }
    ///csi_ker selected as clock
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == USART1SEL::CsiKer
    }
    ///lse selected as clock
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART1SEL::Lse
    }
}
///Field `USART1SEL` writer - USART1 kernel clock source selection
pub type USART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, USART1SEL>;
impl<'a, REG> USART1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///pclk2 selected as clock
    #[inline(always)]
    pub fn pclk2(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Pclk2)
    }
    ///pll2_q selected as clock
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Pll2Q)
    }
    ///pll3_q selected as clock
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Pll3Q)
    }
    ///hsi_ker selected as clock
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::HsiKer)
    }
    ///csi_ker selected as clock
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::CsiKer)
    }
    ///lse selected as clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL::Lse)
    }
}
/**SPI4 and 5 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI45SEL {
    ///0: pclk2 selected as clock
    Pclk2 = 0,
    ///1: pll2_q is selected as clock
    Pll2Q = 1,
    ///2: pll3_q is selected as clock
    Pll3Q = 2,
    ///3: hsi_ker is selected as clock
    HsiKer = 3,
    ///4: csi_ker is selected as clock
    CsiKer = 4,
    ///5: hse_ker is selected as clock
    HseKer = 5,
}
impl From<SPI45SEL> for u8 {
    #[inline(always)]
    fn from(variant: SPI45SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPI45SEL {
    type Ux = u8;
}
impl crate::IsEnum for SPI45SEL {}
///Field `SPI45SEL` reader - SPI4 and 5 kernel clock source selection
pub type SPI45SEL_R = crate::FieldReader<SPI45SEL>;
impl SPI45SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPI45SEL> {
        match self.bits {
            0 => Some(SPI45SEL::Pclk2),
            1 => Some(SPI45SEL::Pll2Q),
            2 => Some(SPI45SEL::Pll3Q),
            3 => Some(SPI45SEL::HsiKer),
            4 => Some(SPI45SEL::CsiKer),
            5 => Some(SPI45SEL::HseKer),
            _ => None,
        }
    }
    ///pclk2 selected as clock
    #[inline(always)]
    pub fn is_pclk2(&self) -> bool {
        *self == SPI45SEL::Pclk2
    }
    ///pll2_q is selected as clock
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == SPI45SEL::Pll2Q
    }
    ///pll3_q is selected as clock
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == SPI45SEL::Pll3Q
    }
    ///hsi_ker is selected as clock
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == SPI45SEL::HsiKer
    }
    ///csi_ker is selected as clock
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == SPI45SEL::CsiKer
    }
    ///hse_ker is selected as clock
    #[inline(always)]
    pub fn is_hse_ker(&self) -> bool {
        *self == SPI45SEL::HseKer
    }
}
///Field `SPI45SEL` writer - SPI4 and 5 kernel clock source selection
pub type SPI45SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SPI45SEL>;
impl<'a, REG> SPI45SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///pclk2 selected as clock
    #[inline(always)]
    pub fn pclk2(self) -> &'a mut crate::W<REG> {
        self.variant(SPI45SEL::Pclk2)
    }
    ///pll2_q is selected as clock
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut crate::W<REG> {
        self.variant(SPI45SEL::Pll2Q)
    }
    ///pll3_q is selected as clock
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut crate::W<REG> {
        self.variant(SPI45SEL::Pll3Q)
    }
    ///hsi_ker is selected as clock
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(SPI45SEL::HsiKer)
    }
    ///csi_ker is selected as clock
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut crate::W<REG> {
        self.variant(SPI45SEL::CsiKer)
    }
    ///hse_ker is selected as clock
    #[inline(always)]
    pub fn hse_ker(self) -> &'a mut crate::W<REG> {
        self.variant(SPI45SEL::HseKer)
    }
}
/**SPI/I2S1 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI1SEL {
    ///0: pll1_q selected as SPI/I2S1 and 7 clock
    Pll1Q = 0,
    ///1: pll2_p selected as SPI/I2S1 and 7 clock
    Pll2P = 1,
    ///2: pll3_p selected as SPI/I2S1 and 7 clock
    Pll3P = 2,
    ///3: I2S_CKIN selected as SPI/I2S1 and 7 clock
    I2sCkin = 3,
    ///4: per selected as SPI/I2S1,and 7 clock
    Per = 4,
}
impl From<SPI1SEL> for u8 {
    #[inline(always)]
    fn from(variant: SPI1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPI1SEL {
    type Ux = u8;
}
impl crate::IsEnum for SPI1SEL {}
///Field `SPI1SEL` reader - SPI/I2S1 kernel clock source selection
pub type SPI1SEL_R = crate::FieldReader<SPI1SEL>;
impl SPI1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPI1SEL> {
        match self.bits {
            0 => Some(SPI1SEL::Pll1Q),
            1 => Some(SPI1SEL::Pll2P),
            2 => Some(SPI1SEL::Pll3P),
            3 => Some(SPI1SEL::I2sCkin),
            4 => Some(SPI1SEL::Per),
            _ => None,
        }
    }
    ///pll1_q selected as SPI/I2S1 and 7 clock
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SPI1SEL::Pll1Q
    }
    ///pll2_p selected as SPI/I2S1 and 7 clock
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == SPI1SEL::Pll2P
    }
    ///pll3_p selected as SPI/I2S1 and 7 clock
    #[inline(always)]
    pub fn is_pll3_p(&self) -> bool {
        *self == SPI1SEL::Pll3P
    }
    ///I2S_CKIN selected as SPI/I2S1 and 7 clock
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == SPI1SEL::I2sCkin
    }
    ///per selected as SPI/I2S1,and 7 clock
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == SPI1SEL::Per
    }
}
///Field `SPI1SEL` writer - SPI/I2S1 kernel clock source selection
pub type SPI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SPI1SEL>;
impl<'a, REG> SPI1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///pll1_q selected as SPI/I2S1 and 7 clock
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1SEL::Pll1Q)
    }
    ///pll2_p selected as SPI/I2S1 and 7 clock
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1SEL::Pll2P)
    }
    ///pll3_p selected as SPI/I2S1 and 7 clock
    #[inline(always)]
    pub fn pll3_p(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1SEL::Pll3P)
    }
    ///I2S_CKIN selected as SPI/I2S1 and 7 clock
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1SEL::I2sCkin)
    }
    ///per selected as SPI/I2S1,and 7 clock
    #[inline(always)]
    pub fn per(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1SEL::Per)
    }
}
/**SAI1 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI1SEL {
    ///0: pll1_q selected as clock
    Pll1Q = 0,
    ///1: pll2_p selected as clock
    Pll2P = 1,
    ///2: pll3_p selected as clock
    Pll3P = 2,
    ///3: I2S_CKIN selected as clock
    I2sCkin = 3,
    ///4: per selected as clock
    Per = 4,
}
impl From<SAI1SEL> for u8 {
    #[inline(always)]
    fn from(variant: SAI1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SAI1SEL {
    type Ux = u8;
}
impl crate::IsEnum for SAI1SEL {}
///Field `SAI1SEL` reader - SAI1 kernel clock source selection
pub type SAI1SEL_R = crate::FieldReader<SAI1SEL>;
impl SAI1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SAI1SEL> {
        match self.bits {
            0 => Some(SAI1SEL::Pll1Q),
            1 => Some(SAI1SEL::Pll2P),
            2 => Some(SAI1SEL::Pll3P),
            3 => Some(SAI1SEL::I2sCkin),
            4 => Some(SAI1SEL::Per),
            _ => None,
        }
    }
    ///pll1_q selected as clock
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SAI1SEL::Pll1Q
    }
    ///pll2_p selected as clock
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == SAI1SEL::Pll2P
    }
    ///pll3_p selected as clock
    #[inline(always)]
    pub fn is_pll3_p(&self) -> bool {
        *self == SAI1SEL::Pll3P
    }
    ///I2S_CKIN selected as clock
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == SAI1SEL::I2sCkin
    }
    ///per selected as clock
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == SAI1SEL::Per
    }
}
///Field `SAI1SEL` writer - SAI1 kernel clock source selection
pub type SAI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SAI1SEL>;
impl<'a, REG> SAI1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///pll1_q selected as clock
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Pll1Q)
    }
    ///pll2_p selected as clock
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Pll2P)
    }
    ///pll3_p selected as clock
    #[inline(always)]
    pub fn pll3_p(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Pll3P)
    }
    ///I2S_CKIN selected as clock
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::I2sCkin)
    }
    ///per selected as clock
    #[inline(always)]
    pub fn per(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SEL::Per)
    }
}
/**SAI2 kernel clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI2SEL {
    ///0: pll1_q selected as clock
    Pll1Q = 0,
    ///1: pll2_p selected as clock
    Pll2P = 1,
    ///2: pll3_p selected as clock
    Pll3P = 2,
    ///3: I2S_CKIN selected as clock
    I2sCkin = 3,
    ///4: per selected as clock
    Per = 4,
    ///5: spdifrx_symb selected as clock
    SpdifrxSymb = 5,
}
impl From<SAI2SEL> for u8 {
    #[inline(always)]
    fn from(variant: SAI2SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SAI2SEL {
    type Ux = u8;
}
impl crate::IsEnum for SAI2SEL {}
///Field `SAI2SEL` reader - SAI2 kernel clock source selection
pub type SAI2SEL_R = crate::FieldReader<SAI2SEL>;
impl SAI2SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SAI2SEL> {
        match self.bits {
            0 => Some(SAI2SEL::Pll1Q),
            1 => Some(SAI2SEL::Pll2P),
            2 => Some(SAI2SEL::Pll3P),
            3 => Some(SAI2SEL::I2sCkin),
            4 => Some(SAI2SEL::Per),
            5 => Some(SAI2SEL::SpdifrxSymb),
            _ => None,
        }
    }
    ///pll1_q selected as clock
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SAI2SEL::Pll1Q
    }
    ///pll2_p selected as clock
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == SAI2SEL::Pll2P
    }
    ///pll3_p selected as clock
    #[inline(always)]
    pub fn is_pll3_p(&self) -> bool {
        *self == SAI2SEL::Pll3P
    }
    ///I2S_CKIN selected as clock
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == SAI2SEL::I2sCkin
    }
    ///per selected as clock
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == SAI2SEL::Per
    }
    ///spdifrx_symb selected as clock
    #[inline(always)]
    pub fn is_spdifrx_symb(&self) -> bool {
        *self == SAI2SEL::SpdifrxSymb
    }
}
///Field `SAI2SEL` writer - SAI2 kernel clock source selection
pub type SAI2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SAI2SEL>;
impl<'a, REG> SAI2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///pll1_q selected as clock
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut crate::W<REG> {
        self.variant(SAI2SEL::Pll1Q)
    }
    ///pll2_p selected as clock
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut crate::W<REG> {
        self.variant(SAI2SEL::Pll2P)
    }
    ///pll3_p selected as clock
    #[inline(always)]
    pub fn pll3_p(self) -> &'a mut crate::W<REG> {
        self.variant(SAI2SEL::Pll3P)
    }
    ///I2S_CKIN selected as clock
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut crate::W<REG> {
        self.variant(SAI2SEL::I2sCkin)
    }
    ///per selected as clock
    #[inline(always)]
    pub fn per(self) -> &'a mut crate::W<REG> {
        self.variant(SAI2SEL::Per)
    }
    ///spdifrx_symb selected as clock
    #[inline(always)]
    pub fn spdifrx_symb(self) -> &'a mut crate::W<REG> {
        self.variant(SAI2SEL::SpdifrxSymb)
    }
}
impl R {
    ///Bits 0:2 - USART1 kernel clock source selection
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - SPI4 and 5 kernel clock source selection
    #[inline(always)]
    pub fn spi45sel(&self) -> SPI45SEL_R {
        SPI45SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - SPI/I2S1 kernel clock source selection
    #[inline(always)]
    pub fn spi1sel(&self) -> SPI1SEL_R {
        SPI1SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 16:18 - SAI1 kernel clock source selection
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:22 - SAI2 kernel clock source selection
    #[inline(always)]
    pub fn sai2sel(&self) -> SAI2SEL_R {
        SAI2SEL_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR3")
            .field("usart1sel", &self.usart1sel())
            .field("spi45sel", &self.spi45sel())
            .field("spi1sel", &self.spi1sel())
            .field("sai1sel", &self.sai1sel())
            .field("sai2sel", &self.sai2sel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - USART1 kernel clock source selection
    #[inline(always)]
    pub fn usart1sel(&mut self) -> USART1SEL_W<'_, CCIPR3rs> {
        USART1SEL_W::new(self, 0)
    }
    ///Bits 4:6 - SPI4 and 5 kernel clock source selection
    #[inline(always)]
    pub fn spi45sel(&mut self) -> SPI45SEL_W<'_, CCIPR3rs> {
        SPI45SEL_W::new(self, 4)
    }
    ///Bits 8:10 - SPI/I2S1 kernel clock source selection
    #[inline(always)]
    pub fn spi1sel(&mut self) -> SPI1SEL_W<'_, CCIPR3rs> {
        SPI1SEL_W::new(self, 8)
    }
    ///Bits 16:18 - SAI1 kernel clock source selection
    #[inline(always)]
    pub fn sai1sel(&mut self) -> SAI1SEL_W<'_, CCIPR3rs> {
        SAI1SEL_W::new(self, 16)
    }
    ///Bits 20:22 - SAI2 kernel clock source selection
    #[inline(always)]
    pub fn sai2sel(&mut self) -> SAI2SEL_W<'_, CCIPR3rs> {
        SAI2SEL_W::new(self, 20)
    }
}
/**RCC APB2 peripherals kernel clock selection register

You can [`read`](crate::Reg::read) this register and get [`ccipr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:CCIPR3)*/
pub struct CCIPR3rs;
impl crate::RegisterSpec for CCIPR3rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr3::R`](R) reader structure
impl crate::Readable for CCIPR3rs {}
///`write(|w| ..)` method takes [`ccipr3::W`](W) writer structure
impl crate::Writable for CCIPR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR3 to value 0
impl crate::Resettable for CCIPR3rs {}
