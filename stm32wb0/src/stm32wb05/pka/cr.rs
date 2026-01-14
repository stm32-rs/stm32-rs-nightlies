///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `EN` reader - Peripheral enable. - 0 : Disable PKA. - 1 : Enable PKA.
pub type EN_R = crate::BitReader;
///Field `EN` writer - Peripheral enable. - 0 : Disable PKA. - 1 : Enable PKA.
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `START` reader - Start the operation - 0: No operation - 1: Writing 1' to this bit starts the operation which is selected by MODE\[5:0\], using the operands and data already written to the PKA RAM. This bit is always read as 0'. Nota: START is ignored if PKA is busy.
pub type START_R = crate::BitReader;
///Field `START` writer - Start the operation - 0: No operation - 1: Writing 1' to this bit starts the operation which is selected by MODE\[5:0\], using the operands and data already written to the PKA RAM. This bit is always read as 0'. Nota: START is ignored if PKA is busy.
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SECLVL` reader - Security enable. - 0: No side channel countermeasure - 1: Square and Multiply always / Double and Add always
pub type SECLVL_R = crate::BitReader;
///Field `SECLVL` writer - Security enable. - 0: No side channel countermeasure - 1: Square and Multiply always / Double and Add always
pub type SECLVL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE` reader - PKA operation code - 000000 : Compute Montgomery parameter and modular exponentiation - 000001 : Compute Montgomery parameter - 000010 : Compute modular exponentiation only (Montgomery parameter should be loaded) - 100000 : Compute Montgomery parameter and compute ECC kP operation - 100010 : Compute the ECC kP primitive only (Montgomery parameter should be loaded) - 100100 : ECDSA sign - 100110 : ECDSA Verification - 101000 : Point Check - 000111 : RSA CRT exponentiation - 001000 : Modular inversion - 001001 : Arithmetic addition - 001010 : Arithmetic Subtraction - 001011 : Arithmetic multiplication - 001100 : Comparison - 001101 : Modular Reduction - 001110 : Modular Addition - 001111 : Modular Subtraction - 010000 : Montgomery Multiplication
pub type MODE_R = crate::FieldReader;
///Field `MODE` writer - PKA operation code - 000000 : Compute Montgomery parameter and modular exponentiation - 000001 : Compute Montgomery parameter - 000010 : Compute modular exponentiation only (Montgomery parameter should be loaded) - 100000 : Compute Montgomery parameter and compute ECC kP operation - 100010 : Compute the ECC kP primitive only (Montgomery parameter should be loaded) - 100100 : ECDSA sign - 100110 : ECDSA Verification - 101000 : Point Check - 000111 : RSA CRT exponentiation - 001000 : Modular inversion - 001001 : Arithmetic addition - 001010 : Arithmetic Subtraction - 001011 : Arithmetic multiplication - 001100 : Comparison - 001101 : Modular Reduction - 001110 : Modular Addition - 001111 : Modular Subtraction - 010000 : Montgomery Multiplication
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `PROCENDIE` reader - End of operation interrupt enable - 0: Interrupt is disabled. - 1: An interrupt is generated when PROCENDF (PKA_SR\[17\]) is set.
pub type PROCENDIE_R = crate::BitReader;
///Field `PROCENDIE` writer - End of operation interrupt enable - 0: Interrupt is disabled. - 1: An interrupt is generated when PROCENDF (PKA_SR\[17\]) is set.
pub type PROCENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMERRIE` reader - RAM error interrupt enable - 0: Interrupt is disabled. - 1: An interrupt is generated when RAMERRF (PKA_SR\[19\]) is set.
pub type RAMERRIE_R = crate::BitReader;
///Field `RAMERRIE` writer - RAM error interrupt enable - 0: Interrupt is disabled. - 1: An interrupt is generated when RAMERRF (PKA_SR\[19\]) is set.
pub type RAMERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADDRERRIE` reader - Address error interrupt enable - 0: Interrupt is disabled. - 1: An interrupt is generated when ADDRERRF (PKA_SR\[20\] is set.
pub type ADDRERRIE_R = crate::BitReader;
///Field `ADDRERRIE` writer - Address error interrupt enable - 0: Interrupt is disabled. - 1: An interrupt is generated when ADDRERRF (PKA_SR\[20\] is set.
pub type ADDRERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Peripheral enable. - 0 : Disable PKA. - 1 : Enable PKA.
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Start the operation - 0: No operation - 1: Writing 1' to this bit starts the operation which is selected by MODE\[5:0\], using the operands and data already written to the PKA RAM. This bit is always read as 0'. Nota: START is ignored if PKA is busy.
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Security enable. - 0: No side channel countermeasure - 1: Square and Multiply always / Double and Add always
    #[inline(always)]
    pub fn seclvl(&self) -> SECLVL_R {
        SECLVL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 8:13 - PKA operation code - 000000 : Compute Montgomery parameter and modular exponentiation - 000001 : Compute Montgomery parameter - 000010 : Compute modular exponentiation only (Montgomery parameter should be loaded) - 100000 : Compute Montgomery parameter and compute ECC kP operation - 100010 : Compute the ECC kP primitive only (Montgomery parameter should be loaded) - 100100 : ECDSA sign - 100110 : ECDSA Verification - 101000 : Point Check - 000111 : RSA CRT exponentiation - 001000 : Modular inversion - 001001 : Arithmetic addition - 001010 : Arithmetic Subtraction - 001011 : Arithmetic multiplication - 001100 : Comparison - 001101 : Modular Reduction - 001110 : Modular Addition - 001111 : Modular Subtraction - 010000 : Montgomery Multiplication
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bit 17 - End of operation interrupt enable - 0: Interrupt is disabled. - 1: An interrupt is generated when PROCENDF (PKA_SR\[17\]) is set.
    #[inline(always)]
    pub fn procendie(&self) -> PROCENDIE_R {
        PROCENDIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - RAM error interrupt enable - 0: Interrupt is disabled. - 1: An interrupt is generated when RAMERRF (PKA_SR\[19\]) is set.
    #[inline(always)]
    pub fn ramerrie(&self) -> RAMERRIE_R {
        RAMERRIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Address error interrupt enable - 0: Interrupt is disabled. - 1: An interrupt is generated when ADDRERRF (PKA_SR\[20\] is set.
    #[inline(always)]
    pub fn addrerrie(&self) -> ADDRERRIE_R {
        ADDRERRIE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("en", &self.en())
            .field("start", &self.start())
            .field("seclvl", &self.seclvl())
            .field("mode", &self.mode())
            .field("procendie", &self.procendie())
            .field("ramerrie", &self.ramerrie())
            .field("addrerrie", &self.addrerrie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Peripheral enable. - 0 : Disable PKA. - 1 : Enable PKA.
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - Start the operation - 0: No operation - 1: Writing 1' to this bit starts the operation which is selected by MODE\[5:0\], using the operands and data already written to the PKA RAM. This bit is always read as 0'. Nota: START is ignored if PKA is busy.
    #[inline(always)]
    pub fn start(&mut self) -> START_W<'_, CRrs> {
        START_W::new(self, 1)
    }
    ///Bit 2 - Security enable. - 0: No side channel countermeasure - 1: Square and Multiply always / Double and Add always
    #[inline(always)]
    pub fn seclvl(&mut self) -> SECLVL_W<'_, CRrs> {
        SECLVL_W::new(self, 2)
    }
    ///Bits 8:13 - PKA operation code - 000000 : Compute Montgomery parameter and modular exponentiation - 000001 : Compute Montgomery parameter - 000010 : Compute modular exponentiation only (Montgomery parameter should be loaded) - 100000 : Compute Montgomery parameter and compute ECC kP operation - 100010 : Compute the ECC kP primitive only (Montgomery parameter should be loaded) - 100100 : ECDSA sign - 100110 : ECDSA Verification - 101000 : Point Check - 000111 : RSA CRT exponentiation - 001000 : Modular inversion - 001001 : Arithmetic addition - 001010 : Arithmetic Subtraction - 001011 : Arithmetic multiplication - 001100 : Comparison - 001101 : Modular Reduction - 001110 : Modular Addition - 001111 : Modular Subtraction - 010000 : Montgomery Multiplication
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<'_, CRrs> {
        MODE_W::new(self, 8)
    }
    ///Bit 17 - End of operation interrupt enable - 0: Interrupt is disabled. - 1: An interrupt is generated when PROCENDF (PKA_SR\[17\]) is set.
    #[inline(always)]
    pub fn procendie(&mut self) -> PROCENDIE_W<'_, CRrs> {
        PROCENDIE_W::new(self, 17)
    }
    ///Bit 19 - RAM error interrupt enable - 0: Interrupt is disabled. - 1: An interrupt is generated when RAMERRF (PKA_SR\[19\]) is set.
    #[inline(always)]
    pub fn ramerrie(&mut self) -> RAMERRIE_W<'_, CRrs> {
        RAMERRIE_W::new(self, 19)
    }
    ///Bit 20 - Address error interrupt enable - 0: Interrupt is disabled. - 1: An interrupt is generated when ADDRERRF (PKA_SR\[20\] is set.
    #[inline(always)]
    pub fn addrerrie(&mut self) -> ADDRERRIE_W<'_, CRrs> {
        ADDRERRIE_W::new(self, 20)
    }
}
/**PKA_CR register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#PKA:CR)*/
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
