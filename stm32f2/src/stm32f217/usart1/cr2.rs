#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Field `ADD` reader - Address of the USART node"]
pub type ADD_R = crate::FieldReader;
#[doc = "Field `ADD` writer - Address of the USART node"]
pub type ADD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "lin break detection length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBDL {
    #[doc = "0: 10-bit break detection"]
    Lbdl10 = 0,
    #[doc = "1: 11-bit break detection"]
    Lbdl11 = 1,
}
impl From<LBDL> for bool {
    #[inline(always)]
    fn from(variant: LBDL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBDL` reader - lin break detection length"]
pub type LBDL_R = crate::BitReader<LBDL>;
impl LBDL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LBDL {
        match self.bits {
            false => LBDL::Lbdl10,
            true => LBDL::Lbdl11,
        }
    }
    #[doc = "10-bit break detection"]
    #[inline(always)]
    pub fn is_lbdl10(&self) -> bool {
        *self == LBDL::Lbdl10
    }
    #[doc = "11-bit break detection"]
    #[inline(always)]
    pub fn is_lbdl11(&self) -> bool {
        *self == LBDL::Lbdl11
    }
}
#[doc = "Field `LBDL` writer - lin break detection length"]
pub type LBDL_W<'a, REG> = crate::BitWriter<'a, REG, LBDL>;
impl<'a, REG> LBDL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "10-bit break detection"]
    #[inline(always)]
    pub fn lbdl10(self) -> &'a mut crate::W<REG> {
        self.variant(LBDL::Lbdl10)
    }
    #[doc = "11-bit break detection"]
    #[inline(always)]
    pub fn lbdl11(self) -> &'a mut crate::W<REG> {
        self.variant(LBDL::Lbdl11)
    }
}
#[doc = "LIN break detection interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBDIE {
    #[doc = "0: LIN break detection interrupt disabled"]
    Disabled = 0,
    #[doc = "1: LIN break detection interrupt enabled"]
    Enabled = 1,
}
impl From<LBDIE> for bool {
    #[inline(always)]
    fn from(variant: LBDIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBDIE` reader - LIN break detection interrupt enable"]
pub type LBDIE_R = crate::BitReader<LBDIE>;
impl LBDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LBDIE {
        match self.bits {
            false => LBDIE::Disabled,
            true => LBDIE::Enabled,
        }
    }
    #[doc = "LIN break detection interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LBDIE::Disabled
    }
    #[doc = "LIN break detection interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LBDIE::Enabled
    }
}
#[doc = "Field `LBDIE` writer - LIN break detection interrupt enable"]
pub type LBDIE_W<'a, REG> = crate::BitWriter<'a, REG, LBDIE>;
impl<'a, REG> LBDIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LIN break detection interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LBDIE::Disabled)
    }
    #[doc = "LIN break detection interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LBDIE::Enabled)
    }
}
#[doc = "Field `LBCL` reader - Last bit clock pulse"]
pub type LBCL_R = crate::BitReader;
#[doc = "Field `LBCL` writer - Last bit clock pulse"]
pub type LBCL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Clock phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHA {
    #[doc = "0: The first clock transition is the first data capture edge"]
    First = 0,
    #[doc = "1: The second clock transition is the first data capture edge"]
    Second = 1,
}
impl From<CPHA> for bool {
    #[inline(always)]
    fn from(variant: CPHA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - Clock phase"]
pub type CPHA_R = crate::BitReader<CPHA>;
impl CPHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPHA {
        match self.bits {
            false => CPHA::First,
            true => CPHA::Second,
        }
    }
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn is_first(&self) -> bool {
        *self == CPHA::First
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        *self == CPHA::Second
    }
}
#[doc = "Field `CPHA` writer - Clock phase"]
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG, CPHA>;
impl<'a, REG> CPHA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn first(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA::First)
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn second(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA::Second)
    }
}
#[doc = "Clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL {
    #[doc = "0: Steady low value on CK pin outside transmission window"]
    Low = 0,
    #[doc = "1: Steady high value on CK pin outside transmission window"]
    High = 1,
}
impl From<CPOL> for bool {
    #[inline(always)]
    fn from(variant: CPOL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Clock polarity"]
pub type CPOL_R = crate::BitReader<CPOL>;
impl CPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPOL {
        match self.bits {
            false => CPOL::Low,
            true => CPOL::High,
        }
    }
    #[doc = "Steady low value on CK pin outside transmission window"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CPOL::Low
    }
    #[doc = "Steady high value on CK pin outside transmission window"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CPOL::High
    }
}
#[doc = "Field `CPOL` writer - Clock polarity"]
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG, CPOL>;
impl<'a, REG> CPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Steady low value on CK pin outside transmission window"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL::Low)
    }
    #[doc = "Steady high value on CK pin outside transmission window"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL::High)
    }
}
#[doc = "Clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKEN {
    #[doc = "0: CK pin disabled"]
    Disabled = 0,
    #[doc = "1: CK pin enabled"]
    Enabled = 1,
}
impl From<CLKEN> for bool {
    #[inline(always)]
    fn from(variant: CLKEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKEN` reader - Clock enable"]
pub type CLKEN_R = crate::BitReader<CLKEN>;
impl CLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKEN {
        match self.bits {
            false => CLKEN::Disabled,
            true => CLKEN::Enabled,
        }
    }
    #[doc = "CK pin disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKEN::Disabled
    }
    #[doc = "CK pin enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CLKEN::Enabled
    }
}
#[doc = "Field `CLKEN` writer - Clock enable"]
pub type CLKEN_W<'a, REG> = crate::BitWriter<'a, REG, CLKEN>;
impl<'a, REG> CLKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CK pin disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CLKEN::Disabled)
    }
    #[doc = "CK pin enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CLKEN::Enabled)
    }
}
#[doc = "STOP bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STOP {
    #[doc = "0: 1 stop bit"]
    Stop1 = 0,
    #[doc = "1: 0.5 stop bits"]
    Stop0p5 = 1,
    #[doc = "2: 2 stop bits"]
    Stop2 = 2,
    #[doc = "3: 1.5 stop bits"]
    Stop1p5 = 3,
}
impl From<STOP> for u8 {
    #[inline(always)]
    fn from(variant: STOP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STOP {
    type Ux = u8;
}
#[doc = "Field `STOP` reader - STOP bits"]
pub type STOP_R = crate::FieldReader<STOP>;
impl STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOP {
        match self.bits {
            0 => STOP::Stop1,
            1 => STOP::Stop0p5,
            2 => STOP::Stop2,
            3 => STOP::Stop1p5,
            _ => unreachable!(),
        }
    }
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == STOP::Stop1
    }
    #[doc = "0.5 stop bits"]
    #[inline(always)]
    pub fn is_stop0p5(&self) -> bool {
        *self == STOP::Stop0p5
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == STOP::Stop2
    }
    #[doc = "1.5 stop bits"]
    #[inline(always)]
    pub fn is_stop1p5(&self) -> bool {
        *self == STOP::Stop1p5
    }
}
#[doc = "Field `STOP` writer - STOP bits"]
pub type STOP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, STOP>;
impl<'a, REG> STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn stop1(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::Stop1)
    }
    #[doc = "0.5 stop bits"]
    #[inline(always)]
    pub fn stop0p5(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::Stop0p5)
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn stop2(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::Stop2)
    }
    #[doc = "1.5 stop bits"]
    #[inline(always)]
    pub fn stop1p5(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::Stop1p5)
    }
}
#[doc = "LIN mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINEN {
    #[doc = "0: LIN mode disabled"]
    Disabled = 0,
    #[doc = "1: LIN mode enabled"]
    Enabled = 1,
}
impl From<LINEN> for bool {
    #[inline(always)]
    fn from(variant: LINEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINEN` reader - LIN mode enable"]
pub type LINEN_R = crate::BitReader<LINEN>;
impl LINEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LINEN {
        match self.bits {
            false => LINEN::Disabled,
            true => LINEN::Enabled,
        }
    }
    #[doc = "LIN mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LINEN::Disabled
    }
    #[doc = "LIN mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LINEN::Enabled
    }
}
#[doc = "Field `LINEN` writer - LIN mode enable"]
pub type LINEN_W<'a, REG> = crate::BitWriter<'a, REG, LINEN>;
impl<'a, REG> LINEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LIN mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LINEN::Disabled)
    }
    #[doc = "LIN mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LINEN::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:3 - Address of the USART node"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - lin break detection length"]
    #[inline(always)]
    pub fn lbdl(&self) -> LBDL_R {
        LBDL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    pub fn lbdie(&self) -> LBDIE_R {
        LBDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    pub fn lbcl(&self) -> LBCL_R {
        LBCL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn linen(&self) -> LINEN_R {
        LINEN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address of the USART node"]
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> ADD_W<CR2rs> {
        ADD_W::new(self, 0)
    }
    #[doc = "Bit 5 - lin break detection length"]
    #[inline(always)]
    #[must_use]
    pub fn lbdl(&mut self) -> LBDL_W<CR2rs> {
        LBDL_W::new(self, 5)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lbdie(&mut self) -> LBDIE_W<CR2rs> {
        LBDIE_W::new(self, 6)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    #[must_use]
    pub fn lbcl(&mut self) -> LBCL_W<CR2rs> {
        LBCL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<CR2rs> {
        CPHA_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<CR2rs> {
        CPOL_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<CR2rs> {
        CLKEN_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<CR2rs> {
        STOP_W::new(self, 12)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn linen(&mut self) -> LINEN_W<CR2rs> {
        LINEN_W::new(self, 14)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
