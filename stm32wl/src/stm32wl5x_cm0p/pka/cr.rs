#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Address error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRERRIE_A {
    #[doc = "0: No interrupt is generated when ADDRERRF flag is set in PKA_SR"]
    DISABLED = 0,
    #[doc = "1: An interrupt is generated when ADDRERRF flag is set in PKA_SR"]
    ENABLED = 1,
}
impl From<ADDRERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRERRIE` reader - Address error interrupt enable"]
pub struct ADDRERRIE_R(crate::FieldReader<bool, ADDRERRIE_A>);
impl ADDRERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADDRERRIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRERRIE_A {
        match self.bits {
            false => ADDRERRIE_A::DISABLED,
            true => ADDRERRIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ADDRERRIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ADDRERRIE_A::ENABLED
    }
}
impl core::ops::Deref for ADDRERRIE_R {
    type Target = crate::FieldReader<bool, ADDRERRIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRERRIE` writer - Address error interrupt enable"]
pub struct ADDRERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRERRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRERRIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt is generated when ADDRERRF flag is set in PKA_SR"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRERRIE_A::DISABLED)
    }
    #[doc = "An interrupt is generated when ADDRERRF flag is set in PKA_SR"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRERRIE_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "RAM error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMERRIE_A {
    #[doc = "0: No interrupt is generated when RAMERRF flag is set in PKA_SR"]
    DISABLED = 0,
    #[doc = "1: An interrupt is generated when RAMERRF flag is set in PKA_SR"]
    ENABLED = 1,
}
impl From<RAMERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: RAMERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMERRIE` reader - RAM error interrupt enable"]
pub struct RAMERRIE_R(crate::FieldReader<bool, RAMERRIE_A>);
impl RAMERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAMERRIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMERRIE_A {
        match self.bits {
            false => RAMERRIE_A::DISABLED,
            true => RAMERRIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RAMERRIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RAMERRIE_A::ENABLED
    }
}
impl core::ops::Deref for RAMERRIE_R {
    type Target = crate::FieldReader<bool, RAMERRIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMERRIE` writer - RAM error interrupt enable"]
pub struct RAMERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMERRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAMERRIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt is generated when RAMERRF flag is set in PKA_SR"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RAMERRIE_A::DISABLED)
    }
    #[doc = "An interrupt is generated when RAMERRF flag is set in PKA_SR"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RAMERRIE_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "PROCENDIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROCENDIE_A {
    #[doc = "0: No interrupt is generated when PROCENDF flag is set in PKA_SR"]
    DISABLED = 0,
    #[doc = "1: An interrupt is generated when PROCENDF flag is set in PKA_SR"]
    ENABLED = 1,
}
impl From<PROCENDIE_A> for bool {
    #[inline(always)]
    fn from(variant: PROCENDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROCENDIE` reader - PROCENDIE"]
pub struct PROCENDIE_R(crate::FieldReader<bool, PROCENDIE_A>);
impl PROCENDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROCENDIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROCENDIE_A {
        match self.bits {
            false => PROCENDIE_A::DISABLED,
            true => PROCENDIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PROCENDIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PROCENDIE_A::ENABLED
    }
}
impl core::ops::Deref for PROCENDIE_R {
    type Target = crate::FieldReader<bool, PROCENDIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROCENDIE` writer - PROCENDIE"]
pub struct PROCENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PROCENDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROCENDIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt is generated when PROCENDF flag is set in PKA_SR"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PROCENDIE_A::DISABLED)
    }
    #[doc = "An interrupt is generated when PROCENDF flag is set in PKA_SR"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PROCENDIE_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "PKA operation code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Montgomery parameter computation then modular exponentiation"]
    MONTGOMERYCOMPEXP = 0,
    #[doc = "1: Montgomery parameter computation only"]
    MONTGOMERYCOMP = 1,
    #[doc = "2: Modular exponentiation only (Montgomery parameter must be loaded first)"]
    MONTGOMERYEXP = 2,
    #[doc = "32: Montgomery parameter computation then ECC scalar multiplication"]
    MONTGOMERYCOMPSCALAR = 32,
    #[doc = "34: ECC scalar multiplication only (Montgomery parameter must be loaded first)"]
    MONTGOMERYSCALAR = 34,
    #[doc = "36: ECDSA sign"]
    ECDSASIGN = 36,
    #[doc = "38: ECDSA verification"]
    ECDSAVERIF = 38,
    #[doc = "40: Point on elliptic curve Fp check"]
    ELLIPTIC = 40,
    #[doc = "7: RSA CRT exponentiation"]
    RSA = 7,
    #[doc = "8: Modular inversion"]
    MODULARINV = 8,
    #[doc = "9: Arithmetic addition"]
    ARITHMETICADD = 9,
    #[doc = "10: Arithmetic subtraction"]
    ARITHMETICSUB = 10,
    #[doc = "11: Arithmetic multiplication"]
    ARITHMETICMUL = 11,
    #[doc = "12: Arithmetic comparison"]
    ARITHMETICCOMP = 12,
    #[doc = "13: Modular reduction"]
    MODULARRED = 13,
    #[doc = "14: Modular addition"]
    MODULARADD = 14,
    #[doc = "15: Modular subtraction"]
    MODULARSUB = 15,
    #[doc = "16: Montgomery multiplication"]
    MODULARMUL = 16,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - PKA operation code"]
pub struct MODE_R(crate::FieldReader<u8, MODE_A>);
impl MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::MONTGOMERYCOMPEXP),
            1 => Some(MODE_A::MONTGOMERYCOMP),
            2 => Some(MODE_A::MONTGOMERYEXP),
            32 => Some(MODE_A::MONTGOMERYCOMPSCALAR),
            34 => Some(MODE_A::MONTGOMERYSCALAR),
            36 => Some(MODE_A::ECDSASIGN),
            38 => Some(MODE_A::ECDSAVERIF),
            40 => Some(MODE_A::ELLIPTIC),
            7 => Some(MODE_A::RSA),
            8 => Some(MODE_A::MODULARINV),
            9 => Some(MODE_A::ARITHMETICADD),
            10 => Some(MODE_A::ARITHMETICSUB),
            11 => Some(MODE_A::ARITHMETICMUL),
            12 => Some(MODE_A::ARITHMETICCOMP),
            13 => Some(MODE_A::MODULARRED),
            14 => Some(MODE_A::MODULARADD),
            15 => Some(MODE_A::MODULARSUB),
            16 => Some(MODE_A::MODULARMUL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MONTGOMERYCOMPEXP`"]
    #[inline(always)]
    pub fn is_montgomery_comp_exp(&self) -> bool {
        **self == MODE_A::MONTGOMERYCOMPEXP
    }
    #[doc = "Checks if the value of the field is `MONTGOMERYCOMP`"]
    #[inline(always)]
    pub fn is_montgomery_comp(&self) -> bool {
        **self == MODE_A::MONTGOMERYCOMP
    }
    #[doc = "Checks if the value of the field is `MONTGOMERYEXP`"]
    #[inline(always)]
    pub fn is_montgomery_exp(&self) -> bool {
        **self == MODE_A::MONTGOMERYEXP
    }
    #[doc = "Checks if the value of the field is `MONTGOMERYCOMPSCALAR`"]
    #[inline(always)]
    pub fn is_montgomery_comp_scalar(&self) -> bool {
        **self == MODE_A::MONTGOMERYCOMPSCALAR
    }
    #[doc = "Checks if the value of the field is `MONTGOMERYSCALAR`"]
    #[inline(always)]
    pub fn is_montgomery_scalar(&self) -> bool {
        **self == MODE_A::MONTGOMERYSCALAR
    }
    #[doc = "Checks if the value of the field is `ECDSASIGN`"]
    #[inline(always)]
    pub fn is_ecdsasign(&self) -> bool {
        **self == MODE_A::ECDSASIGN
    }
    #[doc = "Checks if the value of the field is `ECDSAVERIF`"]
    #[inline(always)]
    pub fn is_ecdsaverif(&self) -> bool {
        **self == MODE_A::ECDSAVERIF
    }
    #[doc = "Checks if the value of the field is `ELLIPTIC`"]
    #[inline(always)]
    pub fn is_elliptic(&self) -> bool {
        **self == MODE_A::ELLIPTIC
    }
    #[doc = "Checks if the value of the field is `RSA`"]
    #[inline(always)]
    pub fn is_rsa(&self) -> bool {
        **self == MODE_A::RSA
    }
    #[doc = "Checks if the value of the field is `MODULARINV`"]
    #[inline(always)]
    pub fn is_modular_inv(&self) -> bool {
        **self == MODE_A::MODULARINV
    }
    #[doc = "Checks if the value of the field is `ARITHMETICADD`"]
    #[inline(always)]
    pub fn is_arithmetic_add(&self) -> bool {
        **self == MODE_A::ARITHMETICADD
    }
    #[doc = "Checks if the value of the field is `ARITHMETICSUB`"]
    #[inline(always)]
    pub fn is_arithmetic_sub(&self) -> bool {
        **self == MODE_A::ARITHMETICSUB
    }
    #[doc = "Checks if the value of the field is `ARITHMETICMUL`"]
    #[inline(always)]
    pub fn is_arithmetic_mul(&self) -> bool {
        **self == MODE_A::ARITHMETICMUL
    }
    #[doc = "Checks if the value of the field is `ARITHMETICCOMP`"]
    #[inline(always)]
    pub fn is_arithmetic_comp(&self) -> bool {
        **self == MODE_A::ARITHMETICCOMP
    }
    #[doc = "Checks if the value of the field is `MODULARRED`"]
    #[inline(always)]
    pub fn is_modular_red(&self) -> bool {
        **self == MODE_A::MODULARRED
    }
    #[doc = "Checks if the value of the field is `MODULARADD`"]
    #[inline(always)]
    pub fn is_modular_add(&self) -> bool {
        **self == MODE_A::MODULARADD
    }
    #[doc = "Checks if the value of the field is `MODULARSUB`"]
    #[inline(always)]
    pub fn is_modular_sub(&self) -> bool {
        **self == MODE_A::MODULARSUB
    }
    #[doc = "Checks if the value of the field is `MODULARMUL`"]
    #[inline(always)]
    pub fn is_modular_mul(&self) -> bool {
        **self == MODE_A::MODULARMUL
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - PKA operation code"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Montgomery parameter computation then modular exponentiation"]
    #[inline(always)]
    pub fn montgomery_comp_exp(self) -> &'a mut W {
        self.variant(MODE_A::MONTGOMERYCOMPEXP)
    }
    #[doc = "Montgomery parameter computation only"]
    #[inline(always)]
    pub fn montgomery_comp(self) -> &'a mut W {
        self.variant(MODE_A::MONTGOMERYCOMP)
    }
    #[doc = "Modular exponentiation only (Montgomery parameter must be loaded first)"]
    #[inline(always)]
    pub fn montgomery_exp(self) -> &'a mut W {
        self.variant(MODE_A::MONTGOMERYEXP)
    }
    #[doc = "Montgomery parameter computation then ECC scalar multiplication"]
    #[inline(always)]
    pub fn montgomery_comp_scalar(self) -> &'a mut W {
        self.variant(MODE_A::MONTGOMERYCOMPSCALAR)
    }
    #[doc = "ECC scalar multiplication only (Montgomery parameter must be loaded first)"]
    #[inline(always)]
    pub fn montgomery_scalar(self) -> &'a mut W {
        self.variant(MODE_A::MONTGOMERYSCALAR)
    }
    #[doc = "ECDSA sign"]
    #[inline(always)]
    pub fn ecdsasign(self) -> &'a mut W {
        self.variant(MODE_A::ECDSASIGN)
    }
    #[doc = "ECDSA verification"]
    #[inline(always)]
    pub fn ecdsaverif(self) -> &'a mut W {
        self.variant(MODE_A::ECDSAVERIF)
    }
    #[doc = "Point on elliptic curve Fp check"]
    #[inline(always)]
    pub fn elliptic(self) -> &'a mut W {
        self.variant(MODE_A::ELLIPTIC)
    }
    #[doc = "RSA CRT exponentiation"]
    #[inline(always)]
    pub fn rsa(self) -> &'a mut W {
        self.variant(MODE_A::RSA)
    }
    #[doc = "Modular inversion"]
    #[inline(always)]
    pub fn modular_inv(self) -> &'a mut W {
        self.variant(MODE_A::MODULARINV)
    }
    #[doc = "Arithmetic addition"]
    #[inline(always)]
    pub fn arithmetic_add(self) -> &'a mut W {
        self.variant(MODE_A::ARITHMETICADD)
    }
    #[doc = "Arithmetic subtraction"]
    #[inline(always)]
    pub fn arithmetic_sub(self) -> &'a mut W {
        self.variant(MODE_A::ARITHMETICSUB)
    }
    #[doc = "Arithmetic multiplication"]
    #[inline(always)]
    pub fn arithmetic_mul(self) -> &'a mut W {
        self.variant(MODE_A::ARITHMETICMUL)
    }
    #[doc = "Arithmetic comparison"]
    #[inline(always)]
    pub fn arithmetic_comp(self) -> &'a mut W {
        self.variant(MODE_A::ARITHMETICCOMP)
    }
    #[doc = "Modular reduction"]
    #[inline(always)]
    pub fn modular_red(self) -> &'a mut W {
        self.variant(MODE_A::MODULARRED)
    }
    #[doc = "Modular addition"]
    #[inline(always)]
    pub fn modular_add(self) -> &'a mut W {
        self.variant(MODE_A::MODULARADD)
    }
    #[doc = "Modular subtraction"]
    #[inline(always)]
    pub fn modular_sub(self) -> &'a mut W {
        self.variant(MODE_A::MODULARSUB)
    }
    #[doc = "Montgomery multiplication"]
    #[inline(always)]
    pub fn modular_mul(self) -> &'a mut W {
        self.variant(MODE_A::MODULARMUL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "start the operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_A {
    #[doc = "1: Writing 1 to this bit starts the operation which is selected by MODE\\[5:0\\], using the operands and data already written to the PKA RAM - This bit is always read as 0"]
    START = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - start the operation"]
pub struct START_R(crate::FieldReader<bool, START_A>);
impl START_R {
    pub(crate) fn new(bits: bool) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<START_A> {
        match self.bits {
            true => Some(START_A::START),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == START_A::START
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<bool, START_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START` writer - start the operation"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Writing 1 to this bit starts the operation which is selected by MODE\\[5:0\\], using the operands and data already written to the PKA RAM - This bit is always read as 0"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(START_A::START)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "PKA enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: Disable PKA"]
    DISABLED = 0,
    #[doc = "1: Enable PKA"]
    ENABLED = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - PKA enable."]
pub struct EN_R(crate::FieldReader<bool, EN_A>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::DISABLED,
            true => EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EN_A::ENABLED
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - PKA enable."]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable PKA"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_A::DISABLED)
    }
    #[doc = "Enable PKA"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 20 - Address error interrupt enable"]
    #[inline(always)]
    pub fn addrerrie(&self) -> ADDRERRIE_R {
        ADDRERRIE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - RAM error interrupt enable"]
    #[inline(always)]
    pub fn ramerrie(&self) -> RAMERRIE_R {
        RAMERRIE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PROCENDIE"]
    #[inline(always)]
    pub fn procendie(&self) -> PROCENDIE_R {
        PROCENDIE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - PKA operation code"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 1 - start the operation"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - PKA enable."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - Address error interrupt enable"]
    #[inline(always)]
    pub fn addrerrie(&mut self) -> ADDRERRIE_W {
        ADDRERRIE_W { w: self }
    }
    #[doc = "Bit 19 - RAM error interrupt enable"]
    #[inline(always)]
    pub fn ramerrie(&mut self) -> RAMERRIE_W {
        RAMERRIE_W { w: self }
    }
    #[doc = "Bit 17 - PROCENDIE"]
    #[inline(always)]
    pub fn procendie(&mut self) -> PROCENDIE_W {
        PROCENDIE_W { w: self }
    }
    #[doc = "Bits 8:13 - PKA operation code"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 1 - start the operation"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 0 - PKA enable."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
