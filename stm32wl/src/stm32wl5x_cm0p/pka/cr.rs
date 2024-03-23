#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "PKA enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN {
    #[doc = "0: Disable PKA"]
    Disabled = 0,
    #[doc = "1: Enable PKA"]
    Enabled = 1,
}
impl From<EN> for bool {
    #[inline(always)]
    fn from(variant: EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - PKA enable."]
pub type EN_R = crate::BitReader<EN>;
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN {
        match self.bits {
            false => EN::Disabled,
            true => EN::Enabled,
        }
    }
    #[doc = "Disable PKA"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN::Disabled
    }
    #[doc = "Enable PKA"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN::Enabled
    }
}
#[doc = "Field `EN` writer - PKA enable."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable PKA"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Disabled)
    }
    #[doc = "Enable PKA"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Enabled)
    }
}
#[doc = "start the operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTW {
    #[doc = "1: Writing 1 to this bit starts the operation which is selected by MODE\\[5:0\\], using the operands and data already written to the PKA RAM - This bit is always read as 0"]
    Start = 1,
}
impl From<STARTW> for bool {
    #[inline(always)]
    fn from(variant: STARTW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - start the operation"]
pub type START_R = crate::BitReader<STARTW>;
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<STARTW> {
        match self.bits {
            true => Some(STARTW::Start),
            _ => None,
        }
    }
    #[doc = "Writing 1 to this bit starts the operation which is selected by MODE\\[5:0\\], using the operands and data already written to the PKA RAM - This bit is always read as 0"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STARTW::Start
    }
}
#[doc = "Field `START` writer - start the operation"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG, STARTW>;
impl<'a, REG> START_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 1 to this bit starts the operation which is selected by MODE\\[5:0\\], using the operands and data already written to the PKA RAM - This bit is always read as 0"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(STARTW::Start)
    }
}
#[doc = "PKA operation code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE {
    #[doc = "0: Montgomery parameter computation then modular exponentiation"]
    MontgomeryCompExp = 0,
    #[doc = "1: Montgomery parameter computation only"]
    MontgomeryComp = 1,
    #[doc = "2: Modular exponentiation only (Montgomery parameter must be loaded first)"]
    MontgomeryExp = 2,
    #[doc = "7: RSA CRT exponentiation"]
    Rsa = 7,
    #[doc = "8: Modular inversion"]
    ModularInv = 8,
    #[doc = "9: Arithmetic addition"]
    ArithmeticAdd = 9,
    #[doc = "10: Arithmetic subtraction"]
    ArithmeticSub = 10,
    #[doc = "11: Arithmetic multiplication"]
    ArithmeticMul = 11,
    #[doc = "12: Arithmetic comparison"]
    ArithmeticComp = 12,
    #[doc = "13: Modular reduction"]
    ModularRed = 13,
    #[doc = "14: Modular addition"]
    ModularAdd = 14,
    #[doc = "15: Modular subtraction"]
    ModularSub = 15,
    #[doc = "16: Montgomery multiplication"]
    ModularMul = 16,
    #[doc = "32: Montgomery parameter computation then ECC scalar multiplication"]
    MontgomeryCompScalar = 32,
    #[doc = "34: ECC scalar multiplication only (Montgomery parameter must be loaded first)"]
    MontgomeryScalar = 34,
    #[doc = "36: ECDSA sign"]
    Ecdsasign = 36,
    #[doc = "38: ECDSA verification"]
    Ecdsaverif = 38,
    #[doc = "40: Point on elliptic curve Fp check"]
    Elliptic = 40,
}
impl From<MODE> for u8 {
    #[inline(always)]
    fn from(variant: MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE {
    type Ux = u8;
}
#[doc = "Field `MODE` reader - PKA operation code"]
pub type MODE_R = crate::FieldReader<MODE>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE> {
        match self.bits {
            0 => Some(MODE::MontgomeryCompExp),
            1 => Some(MODE::MontgomeryComp),
            2 => Some(MODE::MontgomeryExp),
            7 => Some(MODE::Rsa),
            8 => Some(MODE::ModularInv),
            9 => Some(MODE::ArithmeticAdd),
            10 => Some(MODE::ArithmeticSub),
            11 => Some(MODE::ArithmeticMul),
            12 => Some(MODE::ArithmeticComp),
            13 => Some(MODE::ModularRed),
            14 => Some(MODE::ModularAdd),
            15 => Some(MODE::ModularSub),
            16 => Some(MODE::ModularMul),
            32 => Some(MODE::MontgomeryCompScalar),
            34 => Some(MODE::MontgomeryScalar),
            36 => Some(MODE::Ecdsasign),
            38 => Some(MODE::Ecdsaverif),
            40 => Some(MODE::Elliptic),
            _ => None,
        }
    }
    #[doc = "Montgomery parameter computation then modular exponentiation"]
    #[inline(always)]
    pub fn is_montgomery_comp_exp(&self) -> bool {
        *self == MODE::MontgomeryCompExp
    }
    #[doc = "Montgomery parameter computation only"]
    #[inline(always)]
    pub fn is_montgomery_comp(&self) -> bool {
        *self == MODE::MontgomeryComp
    }
    #[doc = "Modular exponentiation only (Montgomery parameter must be loaded first)"]
    #[inline(always)]
    pub fn is_montgomery_exp(&self) -> bool {
        *self == MODE::MontgomeryExp
    }
    #[doc = "RSA CRT exponentiation"]
    #[inline(always)]
    pub fn is_rsa(&self) -> bool {
        *self == MODE::Rsa
    }
    #[doc = "Modular inversion"]
    #[inline(always)]
    pub fn is_modular_inv(&self) -> bool {
        *self == MODE::ModularInv
    }
    #[doc = "Arithmetic addition"]
    #[inline(always)]
    pub fn is_arithmetic_add(&self) -> bool {
        *self == MODE::ArithmeticAdd
    }
    #[doc = "Arithmetic subtraction"]
    #[inline(always)]
    pub fn is_arithmetic_sub(&self) -> bool {
        *self == MODE::ArithmeticSub
    }
    #[doc = "Arithmetic multiplication"]
    #[inline(always)]
    pub fn is_arithmetic_mul(&self) -> bool {
        *self == MODE::ArithmeticMul
    }
    #[doc = "Arithmetic comparison"]
    #[inline(always)]
    pub fn is_arithmetic_comp(&self) -> bool {
        *self == MODE::ArithmeticComp
    }
    #[doc = "Modular reduction"]
    #[inline(always)]
    pub fn is_modular_red(&self) -> bool {
        *self == MODE::ModularRed
    }
    #[doc = "Modular addition"]
    #[inline(always)]
    pub fn is_modular_add(&self) -> bool {
        *self == MODE::ModularAdd
    }
    #[doc = "Modular subtraction"]
    #[inline(always)]
    pub fn is_modular_sub(&self) -> bool {
        *self == MODE::ModularSub
    }
    #[doc = "Montgomery multiplication"]
    #[inline(always)]
    pub fn is_modular_mul(&self) -> bool {
        *self == MODE::ModularMul
    }
    #[doc = "Montgomery parameter computation then ECC scalar multiplication"]
    #[inline(always)]
    pub fn is_montgomery_comp_scalar(&self) -> bool {
        *self == MODE::MontgomeryCompScalar
    }
    #[doc = "ECC scalar multiplication only (Montgomery parameter must be loaded first)"]
    #[inline(always)]
    pub fn is_montgomery_scalar(&self) -> bool {
        *self == MODE::MontgomeryScalar
    }
    #[doc = "ECDSA sign"]
    #[inline(always)]
    pub fn is_ecdsasign(&self) -> bool {
        *self == MODE::Ecdsasign
    }
    #[doc = "ECDSA verification"]
    #[inline(always)]
    pub fn is_ecdsaverif(&self) -> bool {
        *self == MODE::Ecdsaverif
    }
    #[doc = "Point on elliptic curve Fp check"]
    #[inline(always)]
    pub fn is_elliptic(&self) -> bool {
        *self == MODE::Elliptic
    }
}
#[doc = "Field `MODE` writer - PKA operation code"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 6, MODE>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Montgomery parameter computation then modular exponentiation"]
    #[inline(always)]
    pub fn montgomery_comp_exp(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::MontgomeryCompExp)
    }
    #[doc = "Montgomery parameter computation only"]
    #[inline(always)]
    pub fn montgomery_comp(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::MontgomeryComp)
    }
    #[doc = "Modular exponentiation only (Montgomery parameter must be loaded first)"]
    #[inline(always)]
    pub fn montgomery_exp(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::MontgomeryExp)
    }
    #[doc = "RSA CRT exponentiation"]
    #[inline(always)]
    pub fn rsa(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Rsa)
    }
    #[doc = "Modular inversion"]
    #[inline(always)]
    pub fn modular_inv(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::ModularInv)
    }
    #[doc = "Arithmetic addition"]
    #[inline(always)]
    pub fn arithmetic_add(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::ArithmeticAdd)
    }
    #[doc = "Arithmetic subtraction"]
    #[inline(always)]
    pub fn arithmetic_sub(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::ArithmeticSub)
    }
    #[doc = "Arithmetic multiplication"]
    #[inline(always)]
    pub fn arithmetic_mul(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::ArithmeticMul)
    }
    #[doc = "Arithmetic comparison"]
    #[inline(always)]
    pub fn arithmetic_comp(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::ArithmeticComp)
    }
    #[doc = "Modular reduction"]
    #[inline(always)]
    pub fn modular_red(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::ModularRed)
    }
    #[doc = "Modular addition"]
    #[inline(always)]
    pub fn modular_add(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::ModularAdd)
    }
    #[doc = "Modular subtraction"]
    #[inline(always)]
    pub fn modular_sub(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::ModularSub)
    }
    #[doc = "Montgomery multiplication"]
    #[inline(always)]
    pub fn modular_mul(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::ModularMul)
    }
    #[doc = "Montgomery parameter computation then ECC scalar multiplication"]
    #[inline(always)]
    pub fn montgomery_comp_scalar(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::MontgomeryCompScalar)
    }
    #[doc = "ECC scalar multiplication only (Montgomery parameter must be loaded first)"]
    #[inline(always)]
    pub fn montgomery_scalar(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::MontgomeryScalar)
    }
    #[doc = "ECDSA sign"]
    #[inline(always)]
    pub fn ecdsasign(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Ecdsasign)
    }
    #[doc = "ECDSA verification"]
    #[inline(always)]
    pub fn ecdsaverif(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Ecdsaverif)
    }
    #[doc = "Point on elliptic curve Fp check"]
    #[inline(always)]
    pub fn elliptic(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Elliptic)
    }
}
#[doc = "PROCENDIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROCENDIE {
    #[doc = "0: No interrupt is generated when PROCENDF flag is set in PKA_SR"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated when PROCENDF flag is set in PKA_SR"]
    Enabled = 1,
}
impl From<PROCENDIE> for bool {
    #[inline(always)]
    fn from(variant: PROCENDIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROCENDIE` reader - PROCENDIE"]
pub type PROCENDIE_R = crate::BitReader<PROCENDIE>;
impl PROCENDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PROCENDIE {
        match self.bits {
            false => PROCENDIE::Disabled,
            true => PROCENDIE::Enabled,
        }
    }
    #[doc = "No interrupt is generated when PROCENDF flag is set in PKA_SR"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PROCENDIE::Disabled
    }
    #[doc = "An interrupt is generated when PROCENDF flag is set in PKA_SR"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PROCENDIE::Enabled
    }
}
#[doc = "Field `PROCENDIE` writer - PROCENDIE"]
pub type PROCENDIE_W<'a, REG> = crate::BitWriter<'a, REG, PROCENDIE>;
impl<'a, REG> PROCENDIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt is generated when PROCENDF flag is set in PKA_SR"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PROCENDIE::Disabled)
    }
    #[doc = "An interrupt is generated when PROCENDF flag is set in PKA_SR"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PROCENDIE::Enabled)
    }
}
#[doc = "RAM error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMERRIE {
    #[doc = "0: No interrupt is generated when RAMERRF flag is set in PKA_SR"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated when RAMERRF flag is set in PKA_SR"]
    Enabled = 1,
}
impl From<RAMERRIE> for bool {
    #[inline(always)]
    fn from(variant: RAMERRIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMERRIE` reader - RAM error interrupt enable"]
pub type RAMERRIE_R = crate::BitReader<RAMERRIE>;
impl RAMERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RAMERRIE {
        match self.bits {
            false => RAMERRIE::Disabled,
            true => RAMERRIE::Enabled,
        }
    }
    #[doc = "No interrupt is generated when RAMERRF flag is set in PKA_SR"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RAMERRIE::Disabled
    }
    #[doc = "An interrupt is generated when RAMERRF flag is set in PKA_SR"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RAMERRIE::Enabled
    }
}
#[doc = "Field `RAMERRIE` writer - RAM error interrupt enable"]
pub type RAMERRIE_W<'a, REG> = crate::BitWriter<'a, REG, RAMERRIE>;
impl<'a, REG> RAMERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt is generated when RAMERRF flag is set in PKA_SR"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RAMERRIE::Disabled)
    }
    #[doc = "An interrupt is generated when RAMERRF flag is set in PKA_SR"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RAMERRIE::Enabled)
    }
}
#[doc = "Address error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRERRIE {
    #[doc = "0: No interrupt is generated when ADDRERRF flag is set in PKA_SR"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated when ADDRERRF flag is set in PKA_SR"]
    Enabled = 1,
}
impl From<ADDRERRIE> for bool {
    #[inline(always)]
    fn from(variant: ADDRERRIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRERRIE` reader - Address error interrupt enable"]
pub type ADDRERRIE_R = crate::BitReader<ADDRERRIE>;
impl ADDRERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDRERRIE {
        match self.bits {
            false => ADDRERRIE::Disabled,
            true => ADDRERRIE::Enabled,
        }
    }
    #[doc = "No interrupt is generated when ADDRERRF flag is set in PKA_SR"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRERRIE::Disabled
    }
    #[doc = "An interrupt is generated when ADDRERRF flag is set in PKA_SR"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRERRIE::Enabled
    }
}
#[doc = "Field `ADDRERRIE` writer - Address error interrupt enable"]
pub type ADDRERRIE_W<'a, REG> = crate::BitWriter<'a, REG, ADDRERRIE>;
impl<'a, REG> ADDRERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt is generated when ADDRERRF flag is set in PKA_SR"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADDRERRIE::Disabled)
    }
    #[doc = "An interrupt is generated when ADDRERRF flag is set in PKA_SR"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADDRERRIE::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - PKA enable."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - start the operation"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:13 - PKA operation code"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 17 - PROCENDIE"]
    #[inline(always)]
    pub fn procendie(&self) -> PROCENDIE_R {
        PROCENDIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - RAM error interrupt enable"]
    #[inline(always)]
    pub fn ramerrie(&self) -> RAMERRIE_R {
        RAMERRIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Address error interrupt enable"]
    #[inline(always)]
    pub fn addrerrie(&self) -> ADDRERRIE_R {
        ADDRERRIE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PKA enable."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - start the operation"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CRrs> {
        START_W::new(self, 1)
    }
    #[doc = "Bits 8:13 - PKA operation code"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CRrs> {
        MODE_W::new(self, 8)
    }
    #[doc = "Bit 17 - PROCENDIE"]
    #[inline(always)]
    #[must_use]
    pub fn procendie(&mut self) -> PROCENDIE_W<CRrs> {
        PROCENDIE_W::new(self, 17)
    }
    #[doc = "Bit 19 - RAM error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ramerrie(&mut self) -> RAMERRIE_W<CRrs> {
        RAMERRIE_W::new(self, 19)
    }
    #[doc = "Bit 20 - Address error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn addrerrie(&mut self) -> ADDRERRIE_W<CRrs> {
        ADDRERRIE_W::new(self, 20)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
