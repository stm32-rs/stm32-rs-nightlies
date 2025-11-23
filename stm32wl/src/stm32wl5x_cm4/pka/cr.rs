///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**PKA enable.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN {
    ///0: Disable PKA
    Disabled = 0,
    ///1: Enable PKA
    Enabled = 1,
}
impl From<EN> for bool {
    #[inline(always)]
    fn from(variant: EN) -> Self {
        variant as u8 != 0
    }
}
///Field `EN` reader - PKA enable.
pub type EN_R = crate::BitReader<EN>;
impl EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EN {
        match self.bits {
            false => EN::Disabled,
            true => EN::Enabled,
        }
    }
    ///Disable PKA
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN::Disabled
    }
    ///Enable PKA
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN::Enabled
    }
}
///Field `EN` writer - PKA enable.
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable PKA
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Disabled)
    }
    ///Enable PKA
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Enabled)
    }
}
/**start the operation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTW {
    ///1: Writing 1 to this bit starts the operation which is selected by MODE\[5:0\], using the operands and data already written to the PKA RAM - This bit is always read as 0
    Start = 1,
}
impl From<STARTW> for bool {
    #[inline(always)]
    fn from(variant: STARTW) -> Self {
        variant as u8 != 0
    }
}
///Field `START` reader - start the operation
pub type START_R = crate::BitReader<STARTW>;
impl START_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<STARTW> {
        match self.bits {
            true => Some(STARTW::Start),
            _ => None,
        }
    }
    ///Writing 1 to this bit starts the operation which is selected by MODE\[5:0\], using the operands and data already written to the PKA RAM - This bit is always read as 0
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STARTW::Start
    }
}
///Field `START` writer - start the operation
pub type START_W<'a, REG> = crate::BitWriter<'a, REG, STARTW>;
impl<'a, REG> START_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writing 1 to this bit starts the operation which is selected by MODE\[5:0\], using the operands and data already written to the PKA RAM - This bit is always read as 0
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(STARTW::Start)
    }
}
/**PKA operation code

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE {
    ///0: Montgomery parameter computation then modular exponentiation
    MontgomeryCompExp = 0,
    ///1: Montgomery parameter computation only
    MontgomeryComp = 1,
    ///2: Modular exponentiation only (Montgomery parameter must be loaded first)
    MontgomeryExp = 2,
    ///7: RSA CRT exponentiation
    Rsa = 7,
    ///8: Modular inversion
    ModularInv = 8,
    ///9: Arithmetic addition
    ArithmeticAdd = 9,
    ///10: Arithmetic subtraction
    ArithmeticSub = 10,
    ///11: Arithmetic multiplication
    ArithmeticMul = 11,
    ///12: Arithmetic comparison
    ArithmeticComp = 12,
    ///13: Modular reduction
    ModularRed = 13,
    ///14: Modular addition
    ModularAdd = 14,
    ///15: Modular subtraction
    ModularSub = 15,
    ///16: Montgomery multiplication
    ModularMul = 16,
    ///32: Montgomery parameter computation then ECC scalar multiplication
    MontgomeryCompScalar = 32,
    ///34: ECC scalar multiplication only (Montgomery parameter must be loaded first)
    MontgomeryScalar = 34,
    ///36: ECDSA sign
    Ecdsasign = 36,
    ///38: ECDSA verification
    Ecdsaverif = 38,
    ///40: Point on elliptic curve Fp check
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
impl crate::IsEnum for MODE {}
///Field `MODE` reader - PKA operation code
pub type MODE_R = crate::FieldReader<MODE>;
impl MODE_R {
    ///Get enumerated values variant
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
    ///Montgomery parameter computation then modular exponentiation
    #[inline(always)]
    pub fn is_montgomery_comp_exp(&self) -> bool {
        *self == MODE::MontgomeryCompExp
    }
    ///Montgomery parameter computation only
    #[inline(always)]
    pub fn is_montgomery_comp(&self) -> bool {
        *self == MODE::MontgomeryComp
    }
    ///Modular exponentiation only (Montgomery parameter must be loaded first)
    #[inline(always)]
    pub fn is_montgomery_exp(&self) -> bool {
        *self == MODE::MontgomeryExp
    }
    ///RSA CRT exponentiation
    #[inline(always)]
    pub fn is_rsa(&self) -> bool {
        *self == MODE::Rsa
    }
    ///Modular inversion
    #[inline(always)]
    pub fn is_modular_inv(&self) -> bool {
        *self == MODE::ModularInv
    }
    ///Arithmetic addition
    #[inline(always)]
    pub fn is_arithmetic_add(&self) -> bool {
        *self == MODE::ArithmeticAdd
    }
    ///Arithmetic subtraction
    #[inline(always)]
    pub fn is_arithmetic_sub(&self) -> bool {
        *self == MODE::ArithmeticSub
    }
    ///Arithmetic multiplication
    #[inline(always)]
    pub fn is_arithmetic_mul(&self) -> bool {
        *self == MODE::ArithmeticMul
    }
    ///Arithmetic comparison
    #[inline(always)]
    pub fn is_arithmetic_comp(&self) -> bool {
        *self == MODE::ArithmeticComp
    }
    ///Modular reduction
    #[inline(always)]
    pub fn is_modular_red(&self) -> bool {
        *self == MODE::ModularRed
    }
    ///Modular addition
    #[inline(always)]
    pub fn is_modular_add(&self) -> bool {
        *self == MODE::ModularAdd
    }
    ///Modular subtraction
    #[inline(always)]
    pub fn is_modular_sub(&self) -> bool {
        *self == MODE::ModularSub
    }
    ///Montgomery multiplication
    #[inline(always)]
    pub fn is_modular_mul(&self) -> bool {
        *self == MODE::ModularMul
    }
    ///Montgomery parameter computation then ECC scalar multiplication
    #[inline(always)]
    pub fn is_montgomery_comp_scalar(&self) -> bool {
        *self == MODE::MontgomeryCompScalar
    }
    ///ECC scalar multiplication only (Montgomery parameter must be loaded first)
    #[inline(always)]
    pub fn is_montgomery_scalar(&self) -> bool {
        *self == MODE::MontgomeryScalar
    }
    ///ECDSA sign
    #[inline(always)]
    pub fn is_ecdsasign(&self) -> bool {
        *self == MODE::Ecdsasign
    }
    ///ECDSA verification
    #[inline(always)]
    pub fn is_ecdsaverif(&self) -> bool {
        *self == MODE::Ecdsaverif
    }
    ///Point on elliptic curve Fp check
    #[inline(always)]
    pub fn is_elliptic(&self) -> bool {
        *self == MODE::Elliptic
    }
}
///Field `MODE` writer - PKA operation code
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 6, MODE>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Montgomery parameter computation then modular exponentiation
    #[inline(always)]
    pub fn montgomery_comp_exp(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::MontgomeryCompExp)
    }
    ///Montgomery parameter computation only
    #[inline(always)]
    pub fn montgomery_comp(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::MontgomeryComp)
    }
    ///Modular exponentiation only (Montgomery parameter must be loaded first)
    #[inline(always)]
    pub fn montgomery_exp(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::MontgomeryExp)
    }
    ///RSA CRT exponentiation
    #[inline(always)]
    pub fn rsa(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Rsa)
    }
    ///Modular inversion
    #[inline(always)]
    pub fn modular_inv(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::ModularInv)
    }
    ///Arithmetic addition
    #[inline(always)]
    pub fn arithmetic_add(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::ArithmeticAdd)
    }
    ///Arithmetic subtraction
    #[inline(always)]
    pub fn arithmetic_sub(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::ArithmeticSub)
    }
    ///Arithmetic multiplication
    #[inline(always)]
    pub fn arithmetic_mul(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::ArithmeticMul)
    }
    ///Arithmetic comparison
    #[inline(always)]
    pub fn arithmetic_comp(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::ArithmeticComp)
    }
    ///Modular reduction
    #[inline(always)]
    pub fn modular_red(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::ModularRed)
    }
    ///Modular addition
    #[inline(always)]
    pub fn modular_add(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::ModularAdd)
    }
    ///Modular subtraction
    #[inline(always)]
    pub fn modular_sub(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::ModularSub)
    }
    ///Montgomery multiplication
    #[inline(always)]
    pub fn modular_mul(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::ModularMul)
    }
    ///Montgomery parameter computation then ECC scalar multiplication
    #[inline(always)]
    pub fn montgomery_comp_scalar(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::MontgomeryCompScalar)
    }
    ///ECC scalar multiplication only (Montgomery parameter must be loaded first)
    #[inline(always)]
    pub fn montgomery_scalar(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::MontgomeryScalar)
    }
    ///ECDSA sign
    #[inline(always)]
    pub fn ecdsasign(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Ecdsasign)
    }
    ///ECDSA verification
    #[inline(always)]
    pub fn ecdsaverif(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Ecdsaverif)
    }
    ///Point on elliptic curve Fp check
    #[inline(always)]
    pub fn elliptic(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Elliptic)
    }
}
/**PROCENDIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROCENDIE {
    ///0: No interrupt is generated when PROCENDF flag is set in PKA_SR
    Disabled = 0,
    ///1: An interrupt is generated when PROCENDF flag is set in PKA_SR
    Enabled = 1,
}
impl From<PROCENDIE> for bool {
    #[inline(always)]
    fn from(variant: PROCENDIE) -> Self {
        variant as u8 != 0
    }
}
///Field `PROCENDIE` reader - PROCENDIE
pub type PROCENDIE_R = crate::BitReader<PROCENDIE>;
impl PROCENDIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PROCENDIE {
        match self.bits {
            false => PROCENDIE::Disabled,
            true => PROCENDIE::Enabled,
        }
    }
    ///No interrupt is generated when PROCENDF flag is set in PKA_SR
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PROCENDIE::Disabled
    }
    ///An interrupt is generated when PROCENDF flag is set in PKA_SR
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PROCENDIE::Enabled
    }
}
///Field `PROCENDIE` writer - PROCENDIE
pub type PROCENDIE_W<'a, REG> = crate::BitWriter<'a, REG, PROCENDIE>;
impl<'a, REG> PROCENDIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No interrupt is generated when PROCENDF flag is set in PKA_SR
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PROCENDIE::Disabled)
    }
    ///An interrupt is generated when PROCENDF flag is set in PKA_SR
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PROCENDIE::Enabled)
    }
}
/**RAM error interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMERRIE {
    ///0: No interrupt is generated when RAMERRF flag is set in PKA_SR
    Disabled = 0,
    ///1: An interrupt is generated when RAMERRF flag is set in PKA_SR
    Enabled = 1,
}
impl From<RAMERRIE> for bool {
    #[inline(always)]
    fn from(variant: RAMERRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `RAMERRIE` reader - RAM error interrupt enable
pub type RAMERRIE_R = crate::BitReader<RAMERRIE>;
impl RAMERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RAMERRIE {
        match self.bits {
            false => RAMERRIE::Disabled,
            true => RAMERRIE::Enabled,
        }
    }
    ///No interrupt is generated when RAMERRF flag is set in PKA_SR
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RAMERRIE::Disabled
    }
    ///An interrupt is generated when RAMERRF flag is set in PKA_SR
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RAMERRIE::Enabled
    }
}
///Field `RAMERRIE` writer - RAM error interrupt enable
pub type RAMERRIE_W<'a, REG> = crate::BitWriter<'a, REG, RAMERRIE>;
impl<'a, REG> RAMERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No interrupt is generated when RAMERRF flag is set in PKA_SR
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RAMERRIE::Disabled)
    }
    ///An interrupt is generated when RAMERRF flag is set in PKA_SR
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RAMERRIE::Enabled)
    }
}
/**Address error interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRERRIE {
    ///0: No interrupt is generated when ADDRERRF flag is set in PKA_SR
    Disabled = 0,
    ///1: An interrupt is generated when ADDRERRF flag is set in PKA_SR
    Enabled = 1,
}
impl From<ADDRERRIE> for bool {
    #[inline(always)]
    fn from(variant: ADDRERRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `ADDRERRIE` reader - Address error interrupt enable
pub type ADDRERRIE_R = crate::BitReader<ADDRERRIE>;
impl ADDRERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADDRERRIE {
        match self.bits {
            false => ADDRERRIE::Disabled,
            true => ADDRERRIE::Enabled,
        }
    }
    ///No interrupt is generated when ADDRERRF flag is set in PKA_SR
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRERRIE::Disabled
    }
    ///An interrupt is generated when ADDRERRF flag is set in PKA_SR
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRERRIE::Enabled
    }
}
///Field `ADDRERRIE` writer - Address error interrupt enable
pub type ADDRERRIE_W<'a, REG> = crate::BitWriter<'a, REG, ADDRERRIE>;
impl<'a, REG> ADDRERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No interrupt is generated when ADDRERRF flag is set in PKA_SR
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADDRERRIE::Disabled)
    }
    ///An interrupt is generated when ADDRERRF flag is set in PKA_SR
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADDRERRIE::Enabled)
    }
}
impl R {
    ///Bit 0 - PKA enable.
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - start the operation
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 8:13 - PKA operation code
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bit 17 - PROCENDIE
    #[inline(always)]
    pub fn procendie(&self) -> PROCENDIE_R {
        PROCENDIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - RAM error interrupt enable
    #[inline(always)]
    pub fn ramerrie(&self) -> RAMERRIE_R {
        RAMERRIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Address error interrupt enable
    #[inline(always)]
    pub fn addrerrie(&self) -> ADDRERRIE_R {
        ADDRERRIE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("addrerrie", &self.addrerrie())
            .field("ramerrie", &self.ramerrie())
            .field("procendie", &self.procendie())
            .field("mode", &self.mode())
            .field("start", &self.start())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0 - PKA enable.
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - start the operation
    #[inline(always)]
    pub fn start(&mut self) -> START_W<'_, CRrs> {
        START_W::new(self, 1)
    }
    ///Bits 8:13 - PKA operation code
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<'_, CRrs> {
        MODE_W::new(self, 8)
    }
    ///Bit 17 - PROCENDIE
    #[inline(always)]
    pub fn procendie(&mut self) -> PROCENDIE_W<'_, CRrs> {
        PROCENDIE_W::new(self, 17)
    }
    ///Bit 19 - RAM error interrupt enable
    #[inline(always)]
    pub fn ramerrie(&mut self) -> RAMERRIE_W<'_, CRrs> {
        RAMERRIE_W::new(self, 19)
    }
    ///Bit 20 - Address error interrupt enable
    #[inline(always)]
    pub fn addrerrie(&mut self) -> ADDRERRIE_W<'_, CRrs> {
        ADDRERRIE_W::new(self, 20)
    }
}
/**control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#PKA:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
